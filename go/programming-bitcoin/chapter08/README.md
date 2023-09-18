# chapter08 p2sh 스크립트

비밀키를 하나만 사용할 경우, 비밀키를 분실하거나 도난당하게 되면 모든 자금을 잃게 됩니다. 이를 방지하기 위해 여러 개의 비밀키를 사용하는 다중서명(multisig)을 사용합니다. 다중서명은 비트코인의 스크립트를 이용하여 구현할 수 있으며, 그 중에서 p2sh(pay-to-script-hash)를 사용하는 방법을 알아봅니다.

## 8.1 다중서명

다중서명을 지원하도록 잠금 스크립트를 구성하는 첫 번째 시도로 베어 다중서명(bare multisig)가 있습니다. 베어 다중서명은 잠금 스크립트에 여러 개의 공개키가 들어가며, 공개키가 그대로 노출되기 때문에 베어(bare)라고 부릅니다. 다중서명을 이해하려면 OP_CHECKMULTISIG 연산을 알아야 합니다. OP_CHECKMULTISIG 연산은 스택 위에 있는 여러 개의 공개키와 서명을 가져와 유효한 서명의 개수가 일정 수 이상이면 1을, 그렇지 않으면 0을 반환합니다.

다중서명은 m of n 방식으로 구현할 수 있습니다. m은 서명의 개수, n은 공개키의 개수를 의미합니다. 예를 들어 2 of 3 다중서명은 3개의 공개키 중에서 2개의 서명이 유효하면 잠금 스크립트가 해제되는 방식입니다. m과 n은 1에서 20 사이의 정수를 사용할 수 있으며 이 값이 커지는 만큼 잠금 스크립트와 해제 스크립트의 크기가 커집니다.

### OP_CHECKMULTISIG Off-by-one 버그

OP_CHECKMULTISIG 명령어는 m과 n, m개의 서명, 그리고 n개의 공개키 즉 n + m + 2개의 원소를 가져와야 합니다. 그러나 OP_CHECKMULTISIG 명령어는 1개의 원소를 더 가져옵니다. 그리고는 가져온 원소로 어떠한 동작도 하지 않습니다. 아무것도 하지 않지만 원소의 개수가 모자라면 명령어가 실패하므로 임의의 원소를 하나 추가해줘야 합니다. 이 때 트랜잭션 가변성 문제로 인해 네트워크상 대부분 노드가 마지막 원소가 OP_0이 아니면 트랜잭션을 전파하지 않으므로 마지막 원소는 가능하면 OP_0으로 채워주는 것이 좋습니다. OP_CHECKMULTISIG 명령어의 Off-by-one 버그는 비트코인의 초기 버전부터 존재한 버그로, 이 버그를 수정하면 비트코인의 하드포크가 발생합니다. 따라서 이 버그는 영원히 남아있을 것입니다.

---

## 8.2 OP_CHECKMULTISIG 함수

```go
func OpCheckMultiSig(s *[]any, z []byte) bool {
	if len(*s) < 1 {
		return false
	}

	n, ok := (*s)[len(*s)-1].(int) // n
	if !ok {
		return false
	}
	*s = (*s)[:len(*s)-1]

	if len(*s) < n+1 {
		return false
	}

	pubKeys := make([][]byte, n) // pubkeys

	for i := 0; i < n; i++ {
		pubKey, ok := (*s)[len(*s)-1].([]byte)
		if !ok {
			return false
		}
		*s = (*s)[:len(*s)-1]
		pubKeys[i] = pubKey
	}

	m, ok := (*s)[len(*s)-1].(int) // m
	if !ok {
		return false
	}

	if len(*s) < m+1 {
		return false
	}

	derSigs := make([][]byte, m) // der sigs

	for i := 0; i < m; i++ {
		derSig, ok := (*s)[len(*s)-1].([]byte)
		if !ok {
			return false
		}
		*s = (*s)[:len(*s)-1]
		derSigs[i] = derSig[:len(derSig)-1] // remove the sighash type
	}

	*s = (*s)[:len(*s)-1] // pop off the 0

	points := make([]ecc.Point, n)   // points
	sigs := make([]ecc.Signature, m) // sigs

	for i := 0; i < n; i++ {
		point, err := ecc.ParsePoint(pubKeys[i])
		if err != nil {
			log.Println("line 1531:", err)
			return false
		}
		points[i] = point
	}

	for i := 0; i < m; i++ {
		sig, err := ecc.ParseSignature(derSigs[i])
		if err != nil {
			log.Println("line 1540:", err)
			return false
		}
		sigs[i] = sig
	}

	// check that all the signatures are valid
	for _, sig := range sigs {
		for len(points) > 0 {
			point := points[0]
			points = points[1:]

			ok, err := point.Verify(z, sig)
			if err != nil {
				log.Println("line 1554:", err)
				return false
			}

			if ok {
				break
			}
		}
	}

	*s = append(*s, EncodeNum(1))

	return true
}
```

1. n을 가져옵니다. n은 공개키의 개수입니다.
2. n개의 공개키를 가져옵니다.
3. m을 가져옵니다. m은 서명의 개수입니다.
4. m개의 서명을 가져옵니다.
5. 공개키와 서명을 파싱합니다.
6. 모든 서명이 유효한지 확인합니다.
7. 모든 서명이 유효하면 1을 스택에 넣습니다.

---

## 8.3 다중서명의 문제점

다중서명은 비밀키가 하나인 경우와 비교해 단일실패지점을 제거하지만 비효율적입니다. 먼저 공개키의 개수가 많아질수록 잠금 스크립트의 크기가 커집니다. 이에 가독성도 떨어지고 전달하기도 어렵습니다. 또한 이를 저장하기 위한 노드의 자원이 많이 필요합니다. 또한 잠금 스크립트의 크기가 매우 커질 수 있다는 것을 악용하여 다른 용도로 오용할 수 있습니다.

---

## 8.4 p2sh 스크립트

p2sh 스크립트는 긴 잠금 스크립트 문제를 해결하는 방법입니다. p2sh 스크립트는 잠금 스크립트의 해시값을 제시하고 원래의 긴 스크립트는 해제 시 드러납니다.

p2sh 스크립트는 리
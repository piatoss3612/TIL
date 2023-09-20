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

p2sh 스크립트는 긴 잠금 스크립트 문제를 해결하는 방법입니다. p2sh 스크립트는 긴 잠금 스크립트 대신 그 잠금 스크립트의 해시값을 제시하고 원래의 긴 스크립트는 해제 시 드러납니다. 따라서 pay-to-script-hash라고 부릅니다.

p2sh 스크립트는 특별한 규칙을 실행하는 패턴이 있습니다. 

```
<RedeemScript>
OP_HASH160
<Hash160(RedeemScript)>
OP_EQUAL
```

특별 규칙에 의해 만약 이 명령어들이 실행된 후 스택에 1이 남아있으면 리딤 스크립트를 파싱하고 스크립트 명령집합에 추가합니다. 이러한 패턴과 규칙은 BIP0016에서 정의되었습니다.

m of n 다중서명 잠금스크립트를 p2sh 스크립트에서는 리딤 스크립트라고 부릅니다. 내용은 다중서명의 잠금 스크립트와 동일합니다. p2sh 스크립트는 리딤 스크립트의 해시값을 사용합니다. 리딤 스크립트는 나중에 필요하므로 따로 보관해 둡니다.

그렇다면 리딤 스크립트는 어디에 보관해야 할까요? 리딤 스크립트는 대응하는 p2sh 주소를 만들 때 함께 만들어져 사용자가 직접 보관합니다. 이후 사용자가 해제 스크립트를 구성할 때 리딤 스크립트를 사용합니다. 만약 리딤 스크립트를 잃어버리면 자금을 사용할 수 없게 되므로 다시 만들기 쉽게 만드는 것이 좋습니다.

```
ScriptPubKey: OP_HASH160 <Hash160(RedeemScript)> OP_EQUAL
ScriptSig: OP_0 <Signature> <Signature> <RedeemScript>

Script: OP_0 <Signature> <Signature> <RedeemScript> OP_HASH160 <Hash160(RedeemScript)> OP_EQUAL
```

다중서명 스크립트와 마찬가지로 OP_0는 OP_CHECKMULTISIG 연산의 Off-by-one 버그를 해결하기 위해 필요한 원소입니다. OP_0는 0을 스택 위로 올립니다. 그리고 2개의 서명과 리딤 스크립트도 스택 위로 올라갑니다.

```
Script: OP_HASH160 <Hash160(RedeemScript)> OP_EQUAL
Stack: 0 <Signature> <Signature> <RedeemScript>
```

OP_HASH160 명령어는 스택 위에 있는 리딤 스크립트의 해시값을 구합니다. 

```
Script: <Hash160(RedeemScript)> OP_EQUAL
Stack: 0 <Signature> <Signature> <Hash160(RedeemScript)>
```


다음으로 <Hash160(RedeemScript)>가 스택 위로 올라가며 OP_EQUAL 명령어는 스택 위에 있는 두 개의 원소가 같으면 1을, 그렇지 않으면 0을 반환합니다. 

```
Script:
Stack: 0 <Signature> <Signature> 1
```

만약 BIP0016 이전의 비트코인 코어 소프트웨어라면 스택 위의 값이 1이므로 여기서 스크립트가 유효하다고 판단하고 종료합니다. 그러나 BIP0016 이후의 비트코인 코어 소프트웨어라면 리딤 스크립트를 파싱하고 스크립트 명령집합에 추가합니다. 

```
Script: OP_2 <pubkey1> <pubkey2> OP_2 OP_CHECKMULTISIG
Stack: 0 <Signature> <Signature>
```

OP_2는 숫자 2를 스택 위로 올리고 이어서 2개의 공개키와 2를 스택 위로 올립니다.

```
Script: OP_CHECKMULTISIG
Stack: 0 <Signature> <Signature> 2 <pubkey1> <pubkey2> 2
```

이제 OP_CHECKMULTISIG 명령어가 실행됩니다. OP_CHECKMULTISIG 명령어는 n + m + 3개의 원소를 가져와 유효한 서명의 개수가 일정 수 이상이면 1을, 그렇지 않으면 0을 반환합니다. 

```
Script:
Stack: 1
```

OP_CHECKMULTISIG 명령어가 실행된 후 스택에 1이 남아있으므로 스크립트가 유효하다고 판단하고 종료합니다.

---

## 8.5 p2sh 스크립트 코딩하기

### Script 구조체의 Verify 메서드 수정

```go
case []byte:
	stack = append(stack, cmd)

	// cmds 안에 3개의 명령어가 남아있고 BIP0016에서 규정한 특별 패턴에 해당하는 경우
	if len(cmds) == 3 {
		// cmds의 첫 번째 원소가 OP_HASH160, cmds의 두 번째 원소가 20바이트인 []byte 타입, cmds의 세 번째 원소가 OP_EQUAL인지 확인
		opCodeH160, ok1 := cmds[0].(int)
		h160, ok2 := cmds[1].([]byte)
		opCodeEqual, ok3 := cmds[2].(int)

		if ok1 && ok2 && ok3 && opCodeH160 == 0xa9 && len(h160) == 20 && opCodeEqual == 0x87 {
			cmds = cmds[3:] // cmds에서 3개의 명령어 제거

			if !OpHash160(&stack) {
				return false, errors.New("failed to evaluate OP_HASH160")
			}

			stack = append(stack, h160) // 스택에 h160 추가

			if !OpEqual(&stack) {
				return false, errors.New("failed to evaluate OP_EQUAL")
			}

			if !OpVerify(&stack) {
				return false, errors.New("failed to evaluate OP_VERIFY")
			}

			redeemScript := append(utils.EncodeVarint(len(cmd)), cmd...)

			script, _, err := Parse(redeemScript) // redeemScript 파싱
			if err != nil {
				return false, err
			}

			cmds = append(script.Cmds, cmds...) // cmds에 스크립트 명령어 집합 추가
		}
	}
```

### 8.5.1 다중서명 이외의 p2sh

p2sh 스크립트의 장점은 리딤 스크립트의 길이가 최대 520바이트까지 가능하다는 것입니다. 따라서 다중서명 이외에도 다양한 스크립트를 p2sh 스크립트로 만들 수 있는 유연성이 있습니다. 또한 UTXO 집합의 크기를 줄여줍니다. 예를 들어 2 of 3 다중서명을 사용하는 경우 3개의 공개키가 UTXO에 저장되어야 하지만 p2sh 스크립트를 사용하면 리딤 스크립트의 해시값만 저장하면 됩니다.

p2sh 스크립트는 또한 세그윗의 하위 호환도 가능하게 합니다.

### 8.5.2 p2sh 주소

p2sh 스크립트에서 사용되는 주소의 계산 방법은 p2pkh 주소를 구하는 방법과 유사합니다. 리딤 스크립트의 해시값을 구하고 이를 Base58로 인코딩하고 마지막에 체크섬을 붙입니다. p2sh 주소는 테스트넷인 경우 0xc4, 메인넷인 경우 0x05로 시작합니다.

#### 연습문제 8.2

20바이트의 hash160 값을 p2pkh 주소로 변환하는 h160_to_p2pkh_address 함수를 작성하시오.

```go
func H160ToP2pkhAddress(h160 []byte, testnet bool) string {
	var prefix byte
	if testnet {
		prefix = 0x6f
	} else {
		prefix = 0x00
	}
	return EncodeBase58Checksum(append([]byte{prefix}, h160...))
}
```

#### 연습문제 8.3

20바이트의 hash160 값을 p2sh 주소로 변환하는 h160_to_p2sh_address 함수를 작성하시오.

```go
func H160ToP2shAddress(h160 []byte, testnet bool) string {
	var prefix byte
	if testnet {
		prefix = 0xc4
	} else {
		prefix = 0x05
	}
	return EncodeBase58Checksum(append([]byte{prefix}, h160...))
}
```

### 8.5.3 p2sh 서명 검증

p2pkh 스크립트와 마찬가지로 서명해시를 찾는 과정이 p2sh 서명 검증 과정의 가장 큰 어려움입니다. p2sh 서명 검증 과정은 다음과 같습니다.

#### 1단계: 모든 해제 스크립트를 지운다

서명을 검증할 때 먼저 트랜잭션 안에 모든 해제 스크립트를 삭제합니다. 서명을 생성할 때도 마찬가지입니다.

#### 2단계: 삭제된 해제 스크립트 자리에 리딤 스크립트를 삽입한다

삭제된 해제 스크립트 자리에 리딤 스크립트를 삽입합니다. 잠금 스크립트를 삽입했던 p2pkh의 경우와는 다릅니다.

#### 3단계: 해시 유형을 덧붙인다

트랜잭션의 마자막에 4바이트 해시 유형을 덧붙입니다. 이 경우에는 해시 유형을 덧붙이는 과정이 p2pkh와 동일합니다.

#### 트랜잭션 검증 코딩하기

1. SigHash 메서드 수정

```go
// 트랜잭션의 서명해시를 반환하는 함수
// inputIndex는 서명해시를 만들 때 사용할 입력의 인덱스
// redeemScripts는 리딤 스크립트 목록
func (t Tx) SigHash(inputIndex int, redeemScripts ...*script.Script) ([]byte, error) {
	// 입력 인덱스가 트랜잭션의 입력 개수보다 크면 에러를 반환
	if inputIndex >= len(t.Inputs) {
		return nil, fmt.Errorf("input index %d greater than the number of inputs %d", inputIndex, len(t.Inputs))
	}

	s := utils.IntToLittleEndian(t.Version, 4) // 버전

	in, err := t.serializeInputsForSig(inputIndex, redeemScripts...) // 입력 목록, 입력의 인덱스와 리딤 스크립트 목록을 사용
	if err != nil {
		return nil, err
	}

	s = append(s, in...)

	out, err := t.serializeOutputs() // 출력 목록
	if err != nil {
		return nil, err
	}

	s = append(s, out...)

	s = append(s, utils.IntToLittleEndian(t.Locktime, 4)...) // 유효 시점

	s = append(s, utils.IntToLittleEndian(SIGHASH_ALL, 4)...) // SIGHASH_ALL (4바이트)

	h256 := utils.Hash256(s) // 해시를 생성

	return h256, nil // 해시를 반환
}

// 서명해시를 만들 때 사용할 입력 목록을 직렬화한 결과를 반환하는 함수
func (t Tx) serializeInputsForSig(inputIndex int, redeemScripts ...*script.Script) ([]byte, error) {
	inputs := t.Inputs

	result := utils.EncodeVarint(len(inputs)) // 입력 개수

	for i, input := range inputs {
		var scriptSig *script.Script // 해제 스크립트, 기본값은 nil

		if i == inputIndex { // 입력 인덱스가 inputIndex와 같으면
			if len(redeemScripts) > 0 { // 리딤 스크립트가 있으면
				scriptSig = redeemScripts[0] // 리딤 스크립트를 사용
			} else {
				scriptPubKey, err := input.ScriptPubKey(NewTxFetcher(), t.Testnet) // 이전 트랜잭션 출력의 잠금 스크립트를 가져옴
				if err != nil {
					return nil, err
				}

				scriptSig = scriptPubKey // 이전 트랜잭션 출력의 잠금 스크립트를 사용
			}
		}

		s, err := NewTxIn(input.PrevTx, input.PrevIndex, scriptSig, input.SeqNo).Serialize() // scriptSig를 사용하는 새로운 입력을 생성하고 직렬화
		if err != nil {
			return nil, err
		}

		result = append(result, s...) // 직렬화한 결과를 result에 추가
	}

	return result, nil // 직렬화한 결과를 반환
}
```

2. VerifyInput 메서드 수정

```go
// 트랜잭션의 입력을 검증하는 함수
func (t Tx) VerifyInput(inputIndex int) (bool, error) {
	if inputIndex >= len(t.Inputs) {
		return false, fmt.Errorf("input index %d greater than the number of inputs %d", inputIndex, len(t.Inputs))
	}

	input := t.Inputs[inputIndex] // 입력을 가져옴

	scriptSig := input.ScriptSig // 해제 스크립트

	scriptPubKey, err := input.ScriptPubKey(NewTxFetcher(), t.Testnet) // 이전 트랜잭션 출력의 잠금 스크립트를 가져옴
	if err != nil {
		return false, err
	}

	var redeemScripts []*script.Script // 리딤 스크립트 목록

	if script.IsP2shScriptPubkey(scriptPubKey.Cmds) { // 이전 트랜잭션 출력의 잠금 스크립트가 P2SH 스크립트인 경우
		rawRedeem, ok := scriptSig.Cmds[len(scriptSig.Cmds)-1].([]byte) // 해제 스크립트의 마지막 원소가 리딤 스크립트
		if !ok {
			return false, fmt.Errorf("last element should be the redeem script")
		}

		redeemScript, _, err := script.Parse(append([]byte{byte(len(rawRedeem))}, rawRedeem...)) // 리딤 스크립트 파싱
		if err != nil {
			return false, err
		}

		redeemScripts = append(redeemScripts, redeemScript)
	}

	z, err := t.SigHash(inputIndex, redeemScripts...) // 서명해시를 가져옴
	if err != nil {
		return false, err
	}

	combined := scriptSig.Add(scriptPubKey) // 해제 스크립트와 잠금 스크립트를 결합

	return combined.Evaluate(z) // 결합한 스크립트를 평가
}
```

3. 테스트

```go
package main

import (
	"chapter08/tx"
	"encoding/hex"
	"fmt"
)

func main() {
	testVerifyInput()
}

func testVerifyInput() {
	txBytes, _ := hex.DecodeString("0100000001868278ed6ddfb6c1ed3ad5f8181eb0c7a385aa0836f01d5e4789e6bd304d87221a000000db00483045022100dc92655fe37036f47756db8102e0d7d5e28b3beb83a8fef4f5dc0559bddfb94e02205a36d4e4e6c7fcd16658c50783e00c341609977aed3ad00937bf4ee942a8993701483045022100da6bee3c93766232079a01639d07fa869598749729ae323eab8eef53577d611b02207bef15429dcadce2121ea07f233115c6f09034c0be68db99980b9a6c5e75402201475221022626e955ea6ea6d98850c994f9107b036b1334f18ca8830bfff1295d21cfdb702103b287eaf122eea69030a0e9feed096bed8045c8b98bec453e1ffac7fbdbd4bb7152aeffffffff04d3b11400000000001976a914904a49878c0adfc3aa05de7afad2cc15f483a56a88ac7f400900000000001976a914418327e3f3dda4cf5b9089325a4b95abdfa0334088ac722c0c00000000001976a914ba35042cfe9fc66fd35ac2224eebdafd1028ad2788acdc4ace020000000017a91474d691da1574e6b3c192ecfb52cc8984ee7b6c568700000000")
	txObj, _ := tx.ParseTx(txBytes, false)

	ok, err := txObj.VerifyInput(0)
	fmt.Println(ok, err)
}
```
```bash
$ go run main.go
false failed to evaluate OP_CHECKMULTISIG
```

OP_CHECKMULTISIG 명령어가 실패하는 것을 확인했습니다. 왜 이럴까...

로그를 찍어보니 OP_CHECKMULTISIG 명령어가 실행될 때 n이 정수가 아니라고 합니다.

```bash
2023/09/20 17:15:32 line 1485: n is not an int
```

이 부분은 OP_CHECKMULTISIG 함수를 실행할 때 인코딩되어 바이트 슬라이스로 저장된 숫자를 int 타입으로 가져오려고 해서 발생한 문제였습니다.
그래서 아래와 같이 디코딩하는 과정을 추가했습니다.

```go
encN, ok := (*s)[len(*s)-1].([]byte) // 인코딩된 n
if !ok {
	return false
}
*s = (*s)[:len(*s)-1]

n := DecodeNum(encN) // n
```

그런데 서명 파싱하는 부분에서 패닉이 발생합니다... 서명 중 하나가 비어있는 바이트 슬라이스를 읽어오기 때문인데요

```bash
panic: runtime error: index out of range [0] with length 0

goroutine 1 [running]:
chapter08/ecc.ParseSignature({0xc0001225e8?, 0x21?, 0x4d?})
```

이 부분은 제가 m을 읽고 나서 스택에서 읽은 값을 제거하지 않아서 발생한 문제였습니다. 생각보다 쉽게 해결되었습니다.

이렇게 오류를 잡고 나서 다시 실행을 해보면, 다음과 같이 입력 검증을 성공하는 것을 확인할 수 있습니다.

```bash
$ go run main.go 
true <nil>
```
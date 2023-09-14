# chapter07 트랜잭션 검증과 생성

## 7.1 트랜잭션 검증

네트워크로 전송된 트랜잭션은 노드들에게 전파된다. 트랜잭션을 수신한 각 노드는 트랜잭션을 검증한다. 다음은 트랜잭션 검증을 위해 노드가 확인하는 사항들이다.

1. 트랜잭션의 각 입력이 참조하는 이전 트랜잭션 출력이 비트코인을 소비하지 않았는지 확인한다. (이중 지불 방지)
2. 입력 비트코인의 합이 출력 비트코인의 합보다 크거나 같은지 확인한다. (존재하는 비트코인만 소비하는지 확인)
3. 각 입력이 참조하는 트랜잭션 출력의 잠금 스크립트를 해제 스크립트로 해제할 수 있는지 확인한다. (서명이 올바른지 확인)

### 7.1.1 입력 비트코인 존재 확인

트랜잭션 자체만으로는 이중 지불 여부를 확인할 수 없다. 개인 수표만으로 은행 잔고 이상의 금액으로 발행되었는지 알 수 없는 것처럼. 이중 지불을 확인하기 위해서는 전체 트랜잭션 집합으로부터 계산된 UTXO 집합을 뒤져야 한다. 이 작업은 풀 노드에서 직접 실행할 수 있지만, 라이트 노드는 풀 노드에게 UTXO 집합을 요청하여 이중 지불 여부를 확인할 수 있다.

### 7.1.2 입력과 출력 비트코인 합계 확인

노드는 입력 비트코인의 존재를 확인할 뿐만 아니라 입력 비트코인의 합이 출력 비트코인의 합보다 크거나 같은지 확인한다. 이는 트랜잭션이 새로운 비트코인을 만들지 않고 기존 비트코인을 소비하는지 확인하는 것이다. 한 가지 예외는 코인베이스 트랜잭션으로 이는 9장에서 다룬다. 이 작업 또한 UTXO 집합을 찾아봐야 하므로 라이트 노드는 풀 노드에게 UTXO 집합을 요청하여 확인할 수 있다.

```go
package main

import (
	"chapter07/tx"
	"encoding/hex"
	"fmt"
)

func main() {
	checkFee()
}

func checkFee() {
	rawTx, _ := hex.DecodeString("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600")
	parsedTx, _ := tx.ParseTx(rawTx, false)

    // 입력 비트코인의 합: 42505594
    // 출력 비트코인의 합: 42465594
    // 수수료: 40000

    // 그러나 입력 비트코인의 합과 출력 비트코인의 합 모두 오버플로우가 발생(비트코인의 총 발행량 2100만 개를 초과)
    // 따라서 이는 유효한 트랜잭션이 아니다.

	fetcher := tx.NewTxFetcher()

	fee, err := parsedTx.Fee(fetcher)
	if err != nil {
		panic(err)
	}

	fmt.Println(fee >= 0)
}
```
```shell
$ go run main.go
true
```

만약 수수료가 음숫값이라면 입력 비트코인의 합이 출력 비트코인의 합보다 작다는 의미이다. 이러한 트랜잭션이 블록에 포함되면 새로운 비트코인이 발행되는 것이다. 이를 허용하면 안되므로 반드시 수수료를 확인하고 수수료가 음숫값이라면 트랜잭션을 거부해야 한다.

- 더 알아볼 것: 비트코인 오버플로 버그

### 7.1.3 서명 확인

트랜잭션의 입력이 참조하는 트랜잭션 출력의 잠금 스크립트를 해제 스크립트로 해제할 수 있는지 확인한다. 이 과정에서 공개키 P, 서명해시 z 그리고 서명 (r, s)가 필요한데, 공개키와 서명은 스크립트에 포함되어 있어서 쉽게 알아낼 수 있다. 

```go
package main

import (
	"chapter07/ecc"
	"encoding/hex"
	"fmt"
)

func main() {
	checkSig()
}

func checkSig() {
	sec, _ := hex.DecodeString("0349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278a")
	der, _ := hex.DecodeString("3045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed")
	z, _ := hex.DecodeString("27e0c5994dec7824e56dec6b2fcb342eb7cdb0d0957c2fce9882f715e85d81a6")

	point, _ := ecc.ParsePoint(sec)
	sig, _ := ecc.ParseSignature(der)

	ok, err := point.Verify(z, sig)
	if err != nil {
		panic(err)
	}

	fmt.Println(ok)
}
```
```shell
$ go run main.go
true
```

어려운 부분은 서명해시 z를 알아내는 것이다. 직렬화된 트랜잭션에 hash256 함수를 적용한 값을 서명해시로 사용할 수 있을 것 같지만, 이는 정확한 서명해시가 아니다. 직렬화된 트랜잭션의 해제 스크립트에는 서명 자체가 이미 포함되어 있어서 서명해시에 서명이 들어간다는 모순이 발생한다.

따라서 트랜잭션을 아래와 같은 단계로 변형해야 한다. 입력이 여러 개라면 각 입력에 대해 서명해시를 구해야 한다.

#### 1단계: 모든 해제 스크립트를 비운다.

- 서명을 검증할 때 먼저 트랜잭션 안에 모든 해제 스크립트를 삭제한다. 서명을 생성할 때도 마찬가지다.

#### 2단계: 삭제된 해제 스크립트 자리에 사용할 UTXO의 잠금 스크립트를 넣는다.

- 이전 트랜잭션 출력의 잠금 스크립트를 제거한 해제 스크립트 자리에 삽입한다. 잠금 스크립트를 찾기 위해 UTXO 집합을 뒤져야 할 것 같지만, 사실 서명 생성자의 공개키로 해제 스크립트가 만들어지므로 간단하게 찾을 수 있다.

#### 3단계: 해시 유형을 덧붙인다

- 해시 유형은 4바이트 리틀엔디언 정수로, 서명해시에 덧붙여서 서명해시를 만든다. 이 정보로 서명이 어떤 용도로 사용되는지 알 수 있다.

##### 해시 유형 종류

- SIGHASH_ALL: 현재의 입력과 다른 모든 입출력을 함께 인증한다는 의미 (가장 일반적인 해시 유형), 1을 4바이트 리틀엔디언으로 표현
- SIGHASH_SINGLE: 현재의 입력과 이에 대응되는 출력과 다른 모든 입력도 함께 인증한다는 의미, 3을 4바이트 리틀엔디언으로 표현
- SIGHASH_NONE: 현재의 입력과 다른 모든 입력을 인증한다는 의미, 2를 4바이트 리틀엔디언으로 표현
- SIGHASH_ANYONECANPAY: 그냥 이런 해시 유형이 있다는 것만 알아두자

> 인증한다는 의미는 서명해시를 구하기 위한 메시지에 인증되는 필드를 포함한다는 의미

SIGHASH_ALL로 서명을 생성할 경우 최종 트랜잭션의 모든 출력은 서명할 때의 출력과 동일해야 한다. 그렇지 않은 경우 서명이 무효화된다.

변경된 트랜잭션을 z로 계산하는 코드는 다음과 같다.

```go
package main

import (
	"chapter07/ecc"
	"chapter07/utils"
	"encoding/hex"
	"fmt"
	"math/big"
)

func main() {
	checkModifiedTx()
}

func checkModifiedTx() {
	modifiedTx, _ := hex.DecodeString("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000001976a914a802fc56c704ce87c42d7c92eb75e7896bdc41ae88acfeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac1943060001000000")

	h256 := utils.Hash256(modifiedTx)
	z := big.NewInt(0).SetBytes(h256)

	sec, _ := hex.DecodeString("0349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278a")
	der, _ := hex.DecodeString("3045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed")

	point, _ := ecc.ParsePoint(sec)
	sig, _ := ecc.ParseSignature(der)

	ok, err := point.Verify(z.Bytes(), sig)
	if err != nil {
		panic(err)
	}

	fmt.Println(ok)
}
```
```shell
$ go run main.go
true
```

#### 연습문제 7.1

- Tx 구조체의 SigHash 메서드를 작성하시오.

```go
// 트랜잭션의 서명해시를 반환하는 함수
func (t Tx) SigHash(inputIndex int) ([]byte, error) {
	// 입력 인덱스가 트랜잭션의 입력 개수보다 크면 에러를 반환
	if inputIndex >= len(t.Inputs) {
		return nil, fmt.Errorf("input index %d greater than the number of inputs %d", inputIndex, len(t.Inputs))
	}

	s := utils.IntToLittleEndian(t.Version, 4) // 버전

	in, err := t.serializeInputsForSig(inputIndex) // 입력 목록
	if err != nil {
		return nil, err
	}

	out, err := t.serializeOutputs() // 출력 목록
	if err != nil {
		return nil, err
	}

	s = append(s, in...)
	s = append(s, out...)
	s = append(s, utils.IntToLittleEndian(t.Locktime, 4)...)  // 유효 시점
	s = append(s, utils.IntToLittleEndian(SIGHASH_ALL, 4)...) // SIGHASH_ALL (4바이트)

	return utils.Hash256(s), nil // 해시를 반환
}

// 서명해시를 만들 때 사용할 입력 목록을 직렬화한 결과를 반환하는 함수
func (t Tx) serializeInputsForSig(inputIndex int) ([]byte, error) {
	inputs := t.Inputs

	result := utils.EncodeVarint(len(inputs)) // 입력 개수

	for i, input := range inputs {
		if i == inputIndex { // 입력 인덱스가 inputIndex와 같으면
			scriptPubKey, err := input.ScriptPubKey(NewTxFetcher(), t.Testnet) // 이전 트랜잭션 출력의 잠금 스크립트를 가져옴
			if err != nil {
				return nil, err
			}

			newInput := NewTxIn(input.PrevTx, input.PrevIndex, scriptPubKey, input.SeqNo) // 이전 트랜잭션 출력의 잠금 스크립트를 사용하는 새로운 입력을 생성
			s, err := newInput.Serialize()                                                // 새로운 입력을 직렬화
			if err != nil {
				return nil, err
			}

			result = append(result, s...) // 직렬화한 결과를 result에 추가
		} else { // 입력 인덱스가 inputIndex와 다르면
			s, err := input.Serialize() // 입력을 직렬화
			if err != nil {
				return nil, err
			}

			result = append(result, s...) // 직렬화한 결과를 result에 추가
		}
	}

	return result, nil // 직렬화한 결과를 반환
}
```

#### 연습문제 7.2

- Tx 구조체의 VerifyInput 메서드를 작성하시오.

```go
// 트랜잭션의 입력을 검증하는 함수
func (t Tx) VerifyInput(inputIndex int) (bool, error) {
	if inputIndex >= len(t.Inputs) {
		return false, fmt.Errorf("input index %d greater than the number of inputs %d", inputIndex, len(t.Inputs))
	}

	input := t.Inputs[inputIndex] // 입력을 가져옴

	scriptSig := input.ScriptSig                                       // 해제 스크립트
	scriptPubKey, err := input.ScriptPubKey(NewTxFetcher(), t.Testnet) // 이전 트랜잭션 출력의 잠금 스크립트를 가져옴
	if err != nil {
		return false, err
	}

	z, err := t.SigHash(inputIndex) // 서명해시를 가져옴
	if err != nil {
		return false, err
	}

	combined := scriptSig.Add(scriptPubKey) // 해제 스크립트와 잠금 스크립트를 결합

	return combined.Evaluate(z), nil // 결합한 스크립트를 평가
}
```

### 7.1.4 전체 트랜잭션 검증

```go
// 트랜잭션을 검증하는 함수
func (t Tx) Verify() (bool, error) {
	fee, err := t.Fee(NewTxFetcher()) // 수수료를 가져옴
	if err != nil {
		return false, err
	}

	// 수수료가 음수이면 유효하지 않은 트랜잭션
	if fee < 0 {
		return false, nil
	}

	// 트랜잭션의 입력을 검증
	for i := 0; i < len(t.Inputs); i++ {
		ok, err := t.VerifyInput(i)
		if err != nil {
			return false, err
		}

		if !ok {
			return false, nil
		}
	}

	return true, nil
}
```

풀 노드는 실제로는 더 많은 검증을 수행한다. 에를 들어 이중 지불 확인, 합의 규칙 확인, 트랜잭션의 유효 시점 확인 등이다. 하지만 현재 단계에서는 이정도로 충분하다.

---
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

## 7.2 트랜잭션 생성

트랜잭션 검증에 사용한 코드를 활용하여 검증 항목에 부합하는 유효한 트랜잭션을 생성할 수 있다. 요컨대 생성된 트랜잭션의 입력의 합은 출력의 합보다 크거나 같아야 하고, 잠금 스트립트를 해제 스크립트로 해제할 수 있어야 한다.

트랜잭션을 생성하려면 최소한 하나의 UTXO를 참조해야 한다.

### 7.2.1 트랜잭션 설계

트랜잭션을 설계하려면 다음 기본 질문에 답할 수 있어야 한다.

1. 비트코인을 어느 주소로 보낼 것인가?
2. 어느 UTXO를 사용할 것인가?
3. 수수료는 얼마로 할 것인가? (얼마나 빨리 처리되길 원하는가?)

### 7.2.2 트랜잭션 구성

트랜잭션을 구성하기 위해서는 먼저 Base58로 표현된 주소로부터 20바이트 해시를 얻어야 한다. 이 과정은 다음 함수를 사용하면 된다.

```go
var base58Alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz" // base58 인코딩에 사용할 문자열

// base58로 인코딩된 문자열을 바이트 슬라이스로 디코딩하는 함수
func DecodeBase58(s string) ([]byte, error) {
	result := big.NewInt(0) // 결과를 저장할 big.Int

	for _, c := range s {
		// base58Alphabet에서 c의 인덱스를 찾음
		charIndex := strings.IndexByte(base58Alphabet, byte(c))

		// 58을 곱하고 인덱스를 더함
		result.Mul(result, big.NewInt(58))
		result.Add(result, big.NewInt(int64(charIndex)))
	}

	combined := result.FillBytes(make([]byte, 25)) // 크기가 25인 바이트 슬라이스를 만들어 big.Int를 채움
	checksum := combined[len(combined)-4:]         // 마지막 4바이트는 체크섬

	// 체크섬 검사
	if !bytes.Equal(Hash256(combined[:len(combined)-4])[:4], checksum) {
		return nil, fmt.Errorf("bad address: %s %s", checksum, Hash256(combined[:len(combined)-4])[:4])
	}

	return combined[1 : len(combined)-4], nil // prefix(테스트넷 여부)를 제외하고 체크섬을 제외한 바이트 슬라이스를 반환
}
```

또한 20바이트 해시값을 잠금 스크립트로 변환하기 위해 다음 함수를 사용한다. 이 함수는 20바이트 해시값을 입력으로 받아서 p2pkh 스크립트를 반환한다.

```go
func NewP2PKHScript(h160 []byte) *Script {
	return New(
		0x76, // OP_DUP
		0xa9, // OP_HASH160
		h160, // 20바이트의 데이터
		0x88, // OP_EQUALVERIFY
		0xac, // OP_CHECKSIG
	)
}
```

이렇게 주어진 함수들을 활용하여 아래와 같이 트랜잭션을 구성할 수 있다.

```go
package main

import (
	"chapter07/script"
	"chapter07/tx"
	"chapter07/utils"
	"fmt"
)

func main() {
	checkGenTx()
}

func checkGenTx() {
	prevTx := "0d6fe5213c0b3291f208cba8bfb59b7476dffacc4e5cb66f6eb20a080843a299" // 이전 트랜잭션 ID
	prevIndex := 13                                                              // 이전 트랜잭션의 출력 인덱스
	txIn := tx.NewTxIn(prevTx, prevIndex, nil)                                   // 트랜잭션 입력 생성 (해제 스크립트는 비어있음)

	changeAmount := int(0.33 * 1e8)                                           // 출력 금액
	changeH160, _ := utils.DecodeBase58("mzx5YhAH9kNHtcN481u6WkjeHjYtVeKVh2") // 잠금 스크립트를 생성할 주소
	changeScript := script.NewP2PKHScript(changeH160)                         // p2pkh 잠금 스크립트 생성
	changeOutput := tx.NewTxOut(changeAmount, changeScript)                   // 트랜잭션 출력 생성

	targetAmount := int(0.1 * 1e8)                                            // 출력 금액
	targetH160, _ := utils.DecodeBase58("mnrVtF8DWjMu839VW3rBfgYaAfKk8983Xf") // 잠금 스크립트를 생성할 주소
	targetScript := script.NewP2PKHScript(targetH160)                         // p2pkh 잠금 스크립트 생성
	targetOutput := tx.NewTxOut(targetAmount, targetScript)                   // 트랜잭션 출력 생성

	txObj := tx.NewTx(1, []*tx.TxIn{txIn}, []*tx.TxOut{changeOutput, targetOutput}, 0, true) // 트랜잭션 생성
	fmt.Println(txObj)
}
```
```shell
$ go run main.go
tx: cd30a8da777d28ef0e61efe68a9f7c559c1d3e5bcd7b265c850ccb4068598d11
version: 1
inputs: [0d6fe5213c0b3291f208cba8bfb59b7476dffacc4e5cb66f6eb20a080843a299:13]
outputs: [33000000:OP_DUP OP_HASH160 d52ad7ca9b3d096a38e752c2018e6fbc40cdf26f OP_EQUALVERIFY OP_CHECKSIG  10000000:OP_DUP OP_HASH160 507b27411ccf7f16f10297de6cef3f291623eddf OP_EQUALVERIFY OP_CHECKSIG ]
locktime: 0
```

여기서 생성된 트랜잭션은 아직 유효하지 않다. 트랜잭션 입력에 대응되는 이전 트랜잭션 출력의 잠금 스크립트를 해제할 해제 스크립트가 비어있기 때문이다. 따라서 해제 스크립트를 생성해야 한다.


### 7.2.3 트랜잭션 해제 스크립트 생성

잠금 스크립트 안에 해싱된 공개키가 포함되어 있으므로, 이에 대응하는 비밀키를 사용하여 서명해시 z와 이에 대한 DER 형식의 서명을 생성할 수 있다.

```go
package main

import (
	"chapter07/ecc"
	"chapter07/script"
	"chapter07/tx"
	"encoding/hex"
	"fmt"
	"math/big"
)

func main() {
	checkGenScriptSig()
}

func checkGenScriptSig() {
	rawTx, _ := hex.DecodeString("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600")
	parsedTx, _ := tx.ParseTx(rawTx, false) // 트랜잭션 파싱

	z, _ := parsedTx.SigHash(0)                                         // 서명 해시 생성
	privateKey, _ := ecc.NewS256PrivateKey(big.NewInt(8675309).Bytes()) // 개인 키 생성
	point, _ := privateKey.Sign(z)                                      // 서명 생성
	der := point.DER()                                                  // 서명을 DER 형식으로 변환
	sig := append(der, byte(tx.SIGHASH_ALL))                            // DER 서명에 해시 타입을 추가
	sec := privateKey.Point().SEC(true)                                 // 공개 키를 압축된 SEC 형식으로 변환
	scriptSig := script.New(sig, sec)                                   // 해제 스크립트 생성
	parsedTx.Inputs[0].ScriptSig = scriptSig                            // 해제 스크립트를 트랜잭션 입력에 추가
	encoded, err := parsedTx.Serialize()                                // 트랜잭션 직렬화
	if err != nil {
		panic(err)
	}

	fmt.Println(hex.EncodeToString(encoded)) // 직렬화된 트랜잭션 출력
}
```

여기서 서명해시가 예제와 완전히 다른 문제 발생! 이로 인해 서명도, 직렬화된 트랜잭션도 완전히 다른 결과가 나온다...


```
expected z: 18037338614366229343027734445863508930887653120159589908930024158807354868134
actual z: 27e0c5994dec7824e56dec6b2fcb342eb7cdb0d0957c2fce9882f715e85d81a6
```

SigHash 메서드 쪽에 문제가 있어 보여서 확인해보니, 입력을 직렬화할 때 입력인덱스와 매칭되지 않는 입력들의 해제 스크립트를 비우지 않고 그대로 직렬화하고 있었다. 따라서 입력을 직렬화할 때 입력인덱스와 매칭되지 않는 입력들의 해제 스크립트를 비워주는 코드를 추가하였다.

```go
// Before
s, err := input.Serialize() // 입력을 직렬화
if err != nil {
return nil, err
}

// After
s, err := NewTxIn(input.PrevTx, input.PrevIndex, nil, input.SeqNo).Serialize() // 해제 스크립트가 비어있는 새로운 입력을 생성하고 직렬화
if err != nil {
	return nil, err
}
```

이렇게 변경하고 나도 해결이 안되네...

```
expected z: 18037338614366229343027734445863508930887653120159589908930024158807354868134
actual z: 27e0c5994dec7824e56dec6b2fcb342eb7cdb0d0957c2fce9882f715e85d81a6
```

---

아 '18037338614366229343027734445863508930887653120159589908930024158807354868134'가 big number였네요! z를 *big.Int 타입으로 변환해서 출력해 보니 동일한 값이 나왔습니다. 여기서 어리버리를 까버렸네요...

```
expected z: 18037338614366229343027734445863508930887653120159589908930024158807354868134
actual z: 18037338614366229343027734445863508930887653120159589908930024158807354868134
```

그런데 문제는 해결되지 않았습니다. 서명이 완전히 다른 값이 나오는 겁니다.

```
expected signature: Signature(7db2402a3311a3b845b038885e3dd889c08126a8570f26a844e3e4049c482a11,10178cdca4129eacbeab7c44648bf5ac1f9cac217cd609d216ec2ebc8d242c0a)
actual signature: Signature(58b1dabdbe559c98cb592ed9e80c4b4026d388e508f151c0507068ccfb66cc22, 33732f71be9b4567fdfb7388907c9f2c9be872e1fe43540e081408dd2be7a24a)
```

이 부분은 서명 메서드에서 로그를 찍어보니 k값이 완전히 다르게 나오는 것을 확인할 수 있었습니다. 하 진짜... 이런데서 문제가 생길 줄이야...

```
expected k: 31962261299247255223153268424202387061992322641623030136314162059063217934409
actual k: 64814951868233252747544664310574630482194884804266162042260186974060977004832
```

이 부분도 해결했습니다! k를 구하는 메서드에서 z와 비밀키를 *big.Int 타입으로 변환한 뒤에 FillBytes 메서드를 사용하여 32바이트를 채워주니 해결되었습니다.

```go
// Before
zBytes := z.Bytes()
secreteBytes := pvk.secret

// After
zBytes := z.FillBytes(make([]byte, 32))
secreteBytes := big.NewInt(0).SetBytes(pvk.secret).FillBytes(make([]byte, 32))
```

이 부분이 해결되니 나머지는 자연스럽게 해결되었습니다.

```
expected k: 31962261299247255223153268424202387061992322641623030136314162059063217934409
actual k: 31962261299247255223153268424202387061992322641623030136314162059063217934409

expected signature: Signature(7db2402a3311a3b845b038885e3dd889c08126a8570f26a844e3e4049c482a11,10178cdca4129eacbeab7c44648bf5ac1f9cac217cd609d216ec2ebc8d242c0a)
actual signature: Signature(7db2402a3311a3b845b038885e3dd889c08126a8570f26a844e3e4049c482a11, 10178cdca4129eacbeab7c44648bf5ac1f9cac217cd609d216ec2ebc8d242c0a)

expected serialized tx: 0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006a47304402207db2402a3311a3b845b038885e3dd889c08126a8570f26a844e3e4049c482a11022010178cdca4129eacbeab7c44648bf5ac1f9cac217cd609d216ec2ebc8d242c0a012103935581e52c354cd2f484fe8ed83af7a3097005b2f9c60bff71d35bd795f54b67feffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600
actual serialized tx: 0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006a47304402207db2402a3311a3b845b038885e3dd889c08126a8570f26a844e3e4049c482a11022010178cdca4129eacbeab7c44648bf5ac1f9cac217cd609d216ec2ebc8d242c0a012103935581e52c354cd2f484fe8ed83af7a3097005b2f9c60bff71d35bd795f54b67feffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600
```

근데 또 입력에 서명하는 메서드가 안되네요. 진짜 총체적 난국이다... 이 부분은 다음에 다시 해보겠습니다.

이거 문제가 뭔지 알 것 같습니다. 메인넷에서 생선된 트랜잭션인데 비밀키가 '8675309' 이따위로 허술하진 않을 거란 말이죠. 맞겠지? 이런 비밀키는 써서는 안되겠죠? 결론은 제 코드는 문제가 없다는 겁니다. 이 부분은 테스트넷 트랜잭션을 생성하면서 다시 검증해보겠습니다.

---

## 7.3 테스트넷 트랜잭션 생성 및 전파

테스트넷 트랜잭션을 생성하기 위해서는 일단 테스트넷 비트코인이 필요하다. 그리고 테스트넷 비트코인을 받기 위해서는 테스트넷 주소가 필요하다. 테스트넷 주소는 다음과 같이 생성할 수 있다.

```go
func checkGenTestnetTx() {
	secret := utils.LittleEndianToBigInt(utils.Hash256(utils.StringToBytes("piatoss rules the world")))
	privateKey, _ := ecc.NewS256PrivateKey(secret.Bytes())

	address := privateKey.Point().Address(true, true)
	fmt.Println(address)
}
```
```shell
$ go run main.go
mxbVdvxhfkjZPSB2eGSPAcUZJEYNPnL8XW
```

이제 구글에 testnet bitcoin faucet을 검색해보면 테스트 비트코인을 받을 수 있는 사이트를 찾을 수 있다. 그 중 한 곳에 들어가서 생성된 테스트넷 주소를 입력하면 테스트넷 비트코인을 받을 수 있다. 사용한 주소는 https://coinfaucet.eu/en/btc-testnet/ 이다.

이제 트랜잭션을 생성하면 되는데... 입력에 서명하는 메서드에서 오류가 발생한다.

```shell
2023/09/16 11:06:25 line 1448: invalid length
panic: failed to evaluate OP_CHECKSIG
```

서명의 길이가 잘못되었다는 오류가 발생한다. 아마도 해시 유형이 추가되어 있어서 길이 계산이 잘못된 것 같다.
길이를 비교하는 부분만 지우면 잘 동작한다. 일단 임시로 이렇게 진행해보자.

```go
func checkGenTestnetTx() {
	secret := utils.LittleEndianToBigInt(utils.Hash256(utils.StringToBytes("piatoss rules the world")))
	privateKey, _ := ecc.NewS256PrivateKey(secret.Bytes())

	address := privateKey.Point().Address(true, true)

	prevTx := "e770e0b481166da7d0d139c855e86633a12dbd4fa9b97f33a31fc9a458f8ddd7"
	prevIndex := 0
	txIn := tx.NewTxIn(prevTx, prevIndex, nil)

	balance := 1193538

	changeAmount := balance - (balance * 6 / 10) // 40% of balance
	changeH160, _ := utils.DecodeBase58(address)
	changeScript := script.NewP2PKHScript(changeH160)
	changeOutput := tx.NewTxOut(changeAmount, changeScript)

	targetAmount := balance * 6 / 10 // 60% of balance
	targetH160, _ := utils.DecodeBase58("mwJn1YPMq7y5F8J3LkC5Hxg9PHyZ5K4cFv")
	targetScript := script.NewP2PKHScript(targetH160)
	targetOutput := tx.NewTxOut(targetAmount, targetScript)

	txObj := tx.NewTx(1, []*tx.TxIn{txIn}, []*tx.TxOut{changeOutput, targetOutput}, 0, true)

	ok, err := txObj.SignInput(0, privateKey, true)
	if err != nil {
		panic(err)
	}

	fmt.Println(ok)

	serializedTx, err := txObj.Serialize()
	if err != nil {
		panic(err)
	}

	fmt.Println(hex.EncodeToString(serializedTx))
}
```
```shell
$ go run main.go
true
0100000001d7ddf858a4c91fa3337fb9a94fbd2da13366e855c839d1d0a76d1681b4e070e7000000006b4830450221009f1546297fb02e2385cb352bd999f52f31ff8ca2851f0df5b0fe168891c92516022043e7bccacbaca8863962132a957994c09b14e5eaceb47f5b95b074b95cc54ff3012103a7005a25ae9cf0ed9804d4d5f1b0bea6d7b8e901dd4bfa4e21d0914b7e195d74ffffffff02e8480700000000001976a914bb55f73b3c61e3c4e45bf2466a67109652cde9bf88ac5aed0a00000000001976a914ad346f8eb57dee9a37981716e498120ae80e44f788ac00000000
```

이렇게 입력에 서명을 한 뒤에 직렬화한 트랜잭션을 https://live.blockcypher.com/btc-testnet/pushtx/ 에서 전파하면 테스트넷에서 트랜잭션이 전파된다.
# chapter09 블록

비트코인에서는 동일한 트랜잭션 출력을 여러 번 사용하는 이중 지불 문제(double spending problem)가 발생할 수 있습니다. 이중 지불 문제가 발생했다는 것을 어떻게 확인할 수 있을까요? 트랜잭션의 검증을 통해서 트랜잭션이 유효한지 확인해볼 수 있겠지만, 트랜잭션이 처리되는 순서를 알 수 없다면 결국은 동일한 출력을 사용하는 다른 트랜잭션이 먼저 처리되면서 그 이외의 트랜잭션들은 무효 처리가 될 수 있습니다.

블록은 트랜잭션의 묶음이면서 트랜잭션의 순서를 정함으로써 이중 지불 문제의 해결 방안을 제시합니다. 트랜잭션의 순서를 정할 수 있다면 첫 번째 트랜잭션을 유효한 것으로 처리하고, 나머지는 무효 처리하여 이중 지불 문제를 해결할 수 있습니다.

트랜잭션의 순서를 정하기 위해서는 네트워크에 속해 있는 모든 노드의 합의가 필요합니다. 이를 위해서는 어쩔 수 없이 많은 통신 비용이 발생하게 됩니다. 트랜잭션이 발생할 때마다 합의를 내거나 오랜 시간 동안 트랜잭션을 쌓아놓고 한 번에 합의를 내는 것은 실용적이기 않기 때문에 비트코인에서는 10분마다 트랜잭션을 정산합니다.

이번 장에서는 블록을 파싱하고 작업증명을 살펴봅니다.

## 9.1 코인베이스 트랜잭션

코인베이스 트랜잭션은 블록마다 들어가는 첫 번째 트랜잭션이며 비트코인을 발행하는 유일한 트랜잭션입니다. 코인베이스 트랜잭션의 출력에는 보통 p2pkh 잠금 스크립트로 채굴자가 지정한 주소에 지급되는 채굴 보상과 트랜잭션 수수료를 잠가 놓습니다. 즉, 코인베이스 트랜잭션은 채굴자가 채굴 활동에 대한 보상으로 받는 비트코인을 생성하는 트랜잭션이라고 할 수 있습니다.

트랜잭션 구조는 몇 가지 특성을 제외하면 일반적인 트랜잭션과 동일합니다.

1. 정확히 하나의 입력을 가집니다.
2. 입력의 이전 트랜잭션 해시는 32바이트의 0으로 설정합니다.
3. 입력의 이전 트랜잭션 인덱스는 0xffffffff로 설정합니다.

### 연습문제 9.1

Tx 구조체의 IsCoinbase 메서드를 구현하세요.

```go
// 트랜잭션이 코인베이스 트랜잭션인지 여부를 반환하는 함수
func (t Tx) IsCoinbase() bool {
	return len(t.Inputs) == 1 && // 입력 개수가 1이고
		strings.EqualFold(t.Inputs[0].PrevTx, hex.EncodeToString(bytes.Repeat([]byte{0x00}, 32))) && // 이전 트랜잭션이 0x00으로 채워진 32바이트이고
		t.Inputs[0].PrevIndex == 0xffffffff // 이전 트랜잭션의 출력 인덱스가 0xffffffff인 경우 코인베이스 트랜잭션
}
```

### 9.1.1 해제 스크립트

코인베이스 트랜잭션은 새로운 비트코인을 생성하는 트랜잭션이기 때문에 이전 트랜잭션이 없습니다. 그러면 코인베이스 트랜잭션 입력의 해제 스크립트 자리에는 무엇이 들어가야 할까요?

코인베이스 트랜잭션의 해제 스크립트는 블록을 만드는 채굴자가 임의로 정할 수 있습니다. 크기는 최소 2바이트에서 최대 100바이트까지 가능합니다. BIP0034 이전의 해제 스크립트는 잠금 스크립트 없이 자체 실행이 유효하기만 하면 됩니다.

```go
func checkCoinbaseTxScriptSig() {
	rawScript, _ := hex.DecodeString("4d04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73")
	scriptSig, _, _ := script.Parse(rawScript)

	content, ok := scriptSig.Cmds[2].([]byte)
	if !ok {
		fmt.Println("Fail to parse scriptSig")
		return
	}

	fmt.Println(string(content)) // The Times 03/Jan/2009 Chancellor on brink of second bailout for banks
}
```
```bash
$ go run main.go 
The Times 03/Jan/2009 Chancellor on brink of second bailout for banks
```

### 9.1.2 BIP0034 제안서

BIP0034는 코인베이스 트랜잭션 해제 스크립트의 첫 번째 원소를 규정하고 있습니다. 이는 서로 다른 블록임에도 코인베이스 트랜잭션의 ID가 동일한 경우가 발생하는 문제를 해결하기 위한 제안서입니다. 코인베이스 트랜잭션의 해제 스크립트 첫 번째 원소는 블록의 높이를 나타내는 값이어야 합니다. 서로 다른 코인베이스 트랜잭션은 속해있는 블록의 높이가 각각 다르기 때문에 코인베이스 트랜잭션의 ID가 동일한 경우가 발생하지 않습니다.

다음은 코인베이스 트랜잭션의 높이를 파싱하는 방법입니다.

```go
func parseHeightFromCoinbaseTxScriptSig() {
	rawScript, _ := hex.DecodeString("5e03d71b07254d696e656420627920416e74506f6f6c20626a31312f4542312f4144362f43205914293101fabe6d6d678e2c8c34afc36896e7d9402824ed38e856676ee94bfdb0c6c4bcd8b2e5666a0400000000000000c7270000a5e00e00")
	scriptSig, _, _ := script.Parse(rawScript)

	fmt.Println(scriptSig)

	height, ok := scriptSig.Cmds[0].([]byte)
	if !ok {
		fmt.Println("Fail to parse scriptSig")
		return
	}
	fmt.Println(utils.LittleEndianToInt(height)) // 465879
}
```
```bash
$ go run main.go
465879
```

### 연습문제 9.2

Tx 클래스의 CoinbaseHeight 메서드를 구현하세요.

```go
// 코인베이스 트랜잭션의 해제 스크립트에 포함된 높이를 반환하는 함수
func (t Tx) CoinbaseHeight() (bool, int) {
	if !t.IsCoinbase() {
		return false, 0
	}

	// 코인베이스 트랜잭션의 해제 스크립트에서 높이를 가져옴
	scriptSig := t.Inputs[0].ScriptSig
	heightBytes, ok := scriptSig.Cmds[0].([]byte)
	if !ok {
		return false, 0
	}

	return true, utils.LittleEndianToInt(heightBytes) // 리틀엔디언으로 인코딩된 높이를 반환
}
```

---

## 9.2 블록 헤더


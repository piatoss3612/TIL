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

블록 헤더는 블록의 메타데이터를 담고 있습니다. 블록 헤더는 다음과 같은 정보를 담고 있습니다.

- 블록 버전
- 이전 블록의 해시
- 머클 루트
- 타임스탬프
- 비트값 (난이도)
- 논스값

블록 헤더는 80바이트의 고정된 크기를 가지고 있습니다. 블록 헤더는 11장에서 살펴볼 단순 지급 검증(SPV, Simple Payment Verification)을 위한 필수 정보를 담고 있습니다.

트랜잭션 ID처럼 블록 ID도 리틀엔디언으로 표현된 헤더의 hash256 해시값입니다. 블록 ID는 이어지는 다음 블록의 이전 블록 해시값으로 들어갑니다.

```go
func readBlockID() {
	rawBlockHeader, _ := hex.DecodeString("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d")
	blockHash := utils.Hash256(rawBlockHeader)

	blockID := hex.EncodeToString(utils.ReverseBytes(blockHash))
	fmt.Println(blockID) // 0000000000000000007e9e4c586439b0cdbe13b1370bdd9435d76a644d047523
}
```
```bash
$ go run main.go 
0000000000000000007e9e4c586439b0cdbe13b1370bdd9435d76a644d047523
```

지금까지 배운 것을 기초로 Block 구조체를 정의하고 메서드를 구현해보겠습니다.

```go
type Block struct {
	Version    int
	PrevBlock  string
	MerkleRoot string
	Timestamp  int
	Bits       int
	Nonce      int
}

func New(version int, prevBlock, merkleRoot string, timestamp, bits, nonce int) *Block {
	return &Block{
		Version:    version,
		PrevBlock:  prevBlock,
		MerkleRoot: merkleRoot,
		Timestamp:  timestamp,
		Bits:       bits,
		Nonce:      nonce,
	}
}
```

### 연습문제 9.3

Block 구조체의 Parse 메서드를 구현하세요.

```go
// 블록을 파싱하는 함수
func Parse(b []byte) (*Block, error) {
	if len(b) < 80 {
		return nil, errors.New("Block is too short")
	}

	buf := bytes.NewBuffer(b)

	version := utils.LittleEndianToInt(buf.Next(4))                    // 4바이트 리틀엔디언 정수
	prevBlock := hex.EncodeToString(utils.ReverseBytes(buf.Next(32)))  // 32바이트 리틀엔디언 해시
	merkleRoot := hex.EncodeToString(utils.ReverseBytes(buf.Next(32))) // 32바이트 리틀엔디언 해시
	timestamp := utils.LittleEndianToInt(buf.Next(4))                  // 4바이트 리틀엔디언 정수
	bits := utils.BytesToInt(buf.Next(4))                              // 4바이트 리틀엔디언 정수
	nonce := utils.BytesToInt(buf.Next(4))                             // 4바이트 리틀엔디언 정수

	return New(version, prevBlock, merkleRoot, timestamp, bits, nonce), nil
}
```

### 연습문제 9.4

Block 구조체의 Serialize 메서드를 구현하세요.

```go
// 블록을 직렬화하는 함수
func (b *Block) Serialize() ([]byte, error) {
	result := make([]byte, 0, 80)

	version := utils.IntToLittleEndian(b.Version, 4)     // version 4바이트 리틀엔디언
	prevBlockBytes, err := hex.DecodeString(b.PrevBlock) // 16진수 문자열을 []byte로 변환
	if err != nil {
		return nil, err
	}
	prevBlock := utils.ReverseBytes(prevBlockBytes)        // prevBlock 32바이트 리틀엔디언
	merkleRootBytes, err := hex.DecodeString(b.MerkleRoot) // 16진수 문자열을 []byte로 변환
	if err != nil {
		return nil, err
	}
	merkleRoot := utils.ReverseBytes(merkleRootBytes)    // merkleRoot 32바이트 리틀엔디언
	timestamp := utils.IntToLittleEndian(b.Timestamp, 4) // timestamp 4바이트 리틀엔디언
	bits := utils.IntToBytes(b.Bits, 4)                  // bits 4바이트 빅엔디언
	nonce := utils.IntToBytes(b.Nonce, 4)                // nonce 4바이트 빅엔디언

	totalLength := len(version) + len(prevBlock) + len(merkleRoot) + len(timestamp) + len(bits) + len(nonce)

	if totalLength > 80 {
		return nil, errors.New("The size of block is too big")
	}

	result = append(result, version...)
	result = append(result, prevBlock...)
	result = append(result, merkleRoot...)
	result = append(result, timestamp...)
	result = append(result, bits...)
	result = append(result, nonce...)

	return result, nil
}
```

### 연습문제 9.5

Block 구조체의 Hash 메서드를 구현하세요.

```go
// 블록의 해시를 계산하는 함수
func (b *Block) Hash() ([]byte, error) {
	s, err := b.Serialize()
	if err != nil {
		return nil, err
	}
	return utils.ReverseBytes(utils.Hash256(s)), nil
}
```

### 9.2.1 블록 버전

블록 버전은 블록을 생성하는 비트코인 소프트웨어 기능의 집합을 나타냅니다. 블록 버전 2는 소프트웨어가 BIP0034에 대한 지원(코인베이스 트랜잭션에 블록 높이를 넣음)을 추가했음을 의미합니다. 블록 버전 3은 BIP0066에 대한 지원(엄격한 DER 인코딩 시행)을 추가했음을 의미합니다. 블록 버전 4는 BIP0065에 대한 지원(OP_CHECKLOCKTIMEVERIFY 사용을 규정)을 추가했음을 의미합니다. 

이런 식으로 버전을 매기는 방식은 블록 버전이 하나씩 올라갈 때마다 소프트웨어의 기능 준비 상황을 하나씩 네트워크에 전파하는 문제가 있습니다. 이러한 문제를 완화하기 위해서 한 번에 서로 다른 기능의 준비 상황이 29개까지 표시되어 전파되는 BIP0009가 제안되었습니다.

BIP0009의 작동 방식은 다음과 같습니다.
1. 채굴자는 자신의 채굴 소프트웨어가 BIP0009 규정을 따른다는 것을 나타내기 위해 블록 버전 필드 4바이트 중 처음 3비트를 001로 설정합니다.
2. 나머지 29비트는 어떤 기능이 준비되어 있는지를 나타내기 위해 1비트씩 사용하여 29개의 기능에 대한 준비 상황을 표시합니다.

기능 준비 여부는 비교적 간단하게 확인할 수 있습니다.

```go
func readBlockVersionBIP9() {
	rawBlockHeader, _ := hex.DecodeString("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d")

	b, _ := block.Parse(rawBlockHeader)

	fmt.Println("BIP9:", b.Version>>29 == 0x001) // 처음 3비트가 001이면 BIP9 활성화
	fmt.Println("BIP91:", b.Version>>4&1 == 1)   // 4번째 비트가 1이면 BIP91 활성화
	fmt.Println("BIP141:", b.Version>>1&1 == 1)  // 2번째 비트가 1이면 BIP141 활성화
}
```
```bash
$ go run main.go 
BIP9: true
BIP91: false
BIP141: true
```

### 연습문제 9.6

Block 구조체의 Bip9 메서드를 구현하세요.

```go
func (b Block) Bip9() bool {
	return b.Version>>29 == 0x001
}
```

### 연습문제 9.7

Block 구조체의 Bip91 메서드를 구현하세요.

```go
func (b Block) Bip91() bool {
	return b.Version>>4&1 == 1
}
```

### 연습문제 9.8

Block 구조체의 Bip141 메서드를 구현하세요.

```go
func (b Block) Bip141() bool {
	return b.Version>>1&1 == 1
}
```

### 9.2.2 이전 블록 해시값

모든 블록(제네시스 블록 제외)은 이전 블록의 해시값을 가지고 있습니다. 해시값이 충돌나는 경우는 없으므로 이전 블록의 해시값으로 블록을 특정할 수 있고 블록의 순서를 알 수 있으며 블록을 연결하여 체인을 만들 수 있습니다.

### 9.2.3 머클 루트

머클루트는 블록 내 순서에 따라 나열된 모든 트랜잭션을 32바이트 해시값으로 변환한 값입니다. 머클 트리에 대한 자세한 내용은 11장에서 살펴보겠습니다.

### 9.2.4 타임스탬프

타임스탬프는 유닉스 형식으로 표현된 4바이트 값입니다. 이 값은 두 가지 목적으로 사용됩니다. 첫 번째는 트랜잭션의 록타임과 비교하여 트랜잭션이 활성화되는 시점을 알아내기 위해 사용합니다. 두 번째는 비트값을 재계산하는 데 사용합니다.

### 9.2.5 비트값

비트값은 작업증명과 관련된 필드입니다. 이와 관련해서는 9.3절에서 자세히 살펴봅니다.

### 9.2.6 논스값

논스(Nonce, Number used only ONCE)값은 작업증명을 위해 채굴자가 임의로 지정하는 값입니다. 논스값을 조정하여 블록의 해시값이 특정 조건을 만족하도록 합니다.

---

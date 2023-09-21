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

## 9.3 작업증명

작업증명(PoW, Proof of Work)은 탈중앙화 방식의 비트코인 채굴을 가능하게 하고, 전체 네트워크 보안을 유지하는 핵심 기능입니다. 작업증명은 특정한 조건을 만족하는 매우 희소한 숫자를 찾는 과정입니다. 채굴자들은 저마다 특정 조건을 만족할 때까지 논스값을 조정하여 블록 헤더의 해시값을 계속해서 계산합니다. 만약 블록 헤더의 해시값이 특정 조건을 만족하면 블록을 생성한 채굴자는 다른 채굴자들에게 블록을 전파하고 검증을 요청합니다. 검증이 완료되면 채굴자는 블록을 네트워크에 전파하여 블록체인에 추가합니다. 이러한 과정을 거치면서 채굴자는 채굴에 대한 보상으로 비트코인을 받게 됩니다. 비트코인을 보상으로 받을 수 있기 때문에 많은 채굴자들이 블록을 생성하기 위해 경쟁하고 있으며, 이는 곧 소수의 채굴자가 네트워크를 독점하는 것을 방지하고 전체 네트워크를 안전하게 유지하는 것으로 이어집니다.

특정 조건은 다음과 같습니다.

> 블록 헤더의 해시값이 특정 조건을 만족하는 숫자보다 작아야 한다.

### 9.3.1 채굴자의 해시값 생성 방법

특정 조건을 만족하는 해시값을 찾기 위해 채굴자는 블록 헤더의 모든 필드를 직렬화한 후 해시값을 계산합니다. 이 때 변경할 수 있는 값들이 있습니다. 대표적으로 논스값이 있습니다. 논스값은 채굴자가 임의로 지정할 수 있는 값입니다. 그러나 논스값은 4바이트로 크기가 작기 때문에 최신 채굴 장비 몇 대면 금방 확인할 수 있기 때문에 작업증명을 찾기에는 충분한 크기가 아닙니다.

다른 방법들로는 코인베이스 트랜잭션을 변경하여 새로운 머클루트를 만들거나, 버전 필드를 변경할 수 있습니다.

### 9.3.2 목푯값

목푯값은 특정 조건을 만족하는 해시값을 찾기 위한 기준값입니다. 즉, 블록 헤더의 해시값이 목푯값보다 작아야 합니다. 목푯값은 블록 헤더의 비트값으로 계산된 256비트 숫자입니다.

블록헤더의 비트값은 4바이트 크기로, 처음 세 바이트를 계수로 사용하고 마지막 바이트를 지수로 사용합니다. 비트값은 다음과 같은 식으로 계산됩니다.

```go
func calcTargetFromBits() {
	bits, _ := hex.DecodeString("e93c0118")
	exp := big.NewInt(0).SetBytes([]byte{bits[len(bits)-1]}) // 지수
	coef := utils.LittleEndianToBigInt(bits[:len(bits)-1])   // 계수

	target := big.NewInt(0).Mul(coef, big.NewInt(0).Exp(big.NewInt(256), big.NewInt(0).Sub(exp, big.NewInt(3)), nil)) // 계수 * 256^(지수-3) = 목푯값

	fmt.Println(hex.EncodeToString(target.FillBytes(make([]byte, 32)))) // 0000000000000000013ce9000000000000000000000000000000000000000000
}
```
```bash
$ go run main.go
0000000000000000013ce9000000000000000000000000000000000000000000
```

이 목푯값보다 작은 해시값을 찾는 것이 작업증명의 목표입니다. 이 목푯값은 얼마나 찾기 어려울까요? (내용 추가 필요)

이 블록 헤더의 해시값이 작업증명을 만족하는지 다음과 같이 확인할 수 있습니다.

```go
func calcTargetFromBits() {
	bits, _ := hex.DecodeString("e93c0118")
	exp := big.NewInt(0).SetBytes([]byte{bits[len(bits)-1]}) // 지수
	coef := utils.LittleEndianToBigInt(bits[:len(bits)-1])   // 계수

	target := big.NewInt(0).Mul(coef, big.NewInt(0).Exp(big.NewInt(256), big.NewInt(0).Sub(exp, big.NewInt(3)), nil)) // 계수 * 256^(지수-3) = 목푯값

	fmt.Println(hex.EncodeToString(target.FillBytes(make([]byte, 32)))) // 0000000000000000013ce9000000000000000000000000000000000000000000

	rawBlockHeader, _ := hex.DecodeString("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d")
	proof := utils.LittleEndianToBigInt(utils.Hash256(rawBlockHeader))

	fmt.Println(proof.Cmp(target) < 0) // proof가 target보다 작으면 Cmp는 -1을 반환
}
```
```bash
$ go run main.go
true
```

### 연습문제 9.9

block 패키지의 helper.go 파일에 BitsToTarget 함수를 구현하세요.

```go
func BitsToTarget(b []byte) *big.Int {
	exp := utils.BytesToBigInt(b[len(b)-1:])         // 지수
	coef := utils.LittleEndianToBigInt(b[:len(b)-1]) // 계수

	return big.NewInt(0).Mul(coef, big.NewInt(0).Exp(big.NewInt(256), big.NewInt(0).Sub(exp, big.NewInt(3)), nil)) // 계수 * 256^(지수-3) = 목푯값
}
```

### 9.3.3 난이도

난이도는 목푯값만으로는 가늠하기 어려운 작업증명의 난이도를 표현하기 위한 값입니다. 목푯값이 작다면 해시값을 구하는 것이 어려울 것이므로 난이도는 목푯값에 반비례하도록 정의하면 서로 다른 난이도 사이의 비교가 쉬울 것입니다. 참고로 최초 블록의 난이도는 1이었습니다. 2023년 9월 21일 기준으로는 57119871304635(https://blockchair.com/ko/bitcoin/charts/difficulty)입니다... 어마어마하네요.

난이도 계산 수식은 다음과 같습니다.
```
difficulty = 0xffff * 256^(0x1d - 3) / target
```
```go
func calcDifficulty() {
	bits, _ := hex.DecodeString("e93c0118")
	target := block.BitsToTarget(bits)

	// difficulty = 0xffff * 256^(0x1d - 3) / target
	difficulty := big.NewFloat(0).Mul(big.NewFloat(0xffff), big.NewFloat(0).Quo(big.NewFloat(0).SetInt(new(big.Int).Exp(big.NewInt(256), big.NewInt(0).Sub(big.NewInt(0x1d), big.NewInt(3)), nil)), big.NewFloat(0).SetInt(target))) // 0xffff * 256^(0x1d - 3) / target

	fmt.Println(difficulty.Text('f', -1)) // 888171856257.3206
}
```
```bash
$ go run main.go 
888171856257.3206
```

### 연습문제 9.10

Block 구조체의 Difficulty 메서드를 구현하세요.

```go
// 블록의 난이도를 계산하는 함수
func (b Block) Difficulty() *big.Float {
	target := BitsToTarget(utils.IntToBytes(b.Bits, 4))
	return big.NewFloat(0).Mul(big.NewFloat(0xffff), big.NewFloat(0).Quo(big.NewFloat(0).SetInt(new(big.Int).Exp(big.NewInt(256), big.NewInt(0).Sub(big.NewInt(0x1d), big.NewInt(3)), nil)), big.NewFloat(0).SetInt(target))) // 0xffff * 256^(0x1d - 3) / target
}
```

### 9.3.4 작업증명 유효성 확인

블록 헤더의 hash256 해시값을 계산하고 이를 리틀엔디언 정수로 읽어서 목푯값과 비교하여 작업증명을 확인할 수 있습니다. 이 때 블록 헤더의 해시값이 목푯값보다 작아야 합니다.

### 연습문제 9.11

Block 구조체의 CheckProofOfWork 메서드를 구현하세요.

```go
// 작업증명의 유호성을 검증하는 함수
func (b Block) CheckProofOfWork() (bool, error) {
	hash, err := b.Hash() // 블록의 해시를 계산
	if err != nil {
		return false, err
	}

	target := BitsToTarget(utils.IntToBytes(b.Bits, 4))    // 목푯값 계산
	proof := new(big.Int).SetBytes(utils.ReverseBytes(hash)) // 블록의 해시를 little endian으로 변환한 뒤 big.Int로 변환

	return proof.Cmp(target) == -1, nil // 블록의 해시가 목푯값보다 작으면 true, 크거나 같으면 false 반환
}
```

### 9.3.5 난이도 조정

비트코인에서는 2016개의 블록이 생성되는 시간을 난이도 조정 기간(difficulty adjustment period)라고 합니다. 이 기간 동안 생성된 블록의 개수가 2016개가 되도록 난이도를 조정합니다. 이 때 난이도를 조정하는 방법은 다음과 같습니다.

```
time_differential = (난이도 조정 기간의 마지막 블록 타임스탬프) - (난이도 조정 기간의 첫 번째 블록 타임스탬프)

if time_differential > 8주:
    new_target = previous_target * 8주 / 2주
else if time_differential < 3.5일:
    new_target = previous_target * 3.5일 / 2주
else:
    new_target = previous_target * time_differential / 2주
```

만약 time_differential이 2주보다 크다면 목푯값이 커지면서 난이도가 쉬워지고, time_differential이 2주보다 작다면 목푯값이 작아지면서 난이도가 어려워집니다.
이 때 time_differential의 최댓값은 8주, 최솟값은 3.5일로 설정하여 목푯값은 최소 1/4로, 최대 4배로 제한됩니다. 

각 블록이 평균적으로 10분 마다 생성된다면, 2016개의 블록을 생성하기 위해서 2주의 시간이 걸립니다. 따라서 네트워크의 해시 파워가 크고 작은 것에 상관없이 2주의 시간이 걸리도록 난이도를 조정하는 방향으로 설계되었습니다.

이 공식은 다음과 같이 코딩할 수 있습니다.

```go
func calcNewTarget() {
	rawLastBlock, _ := hex.DecodeString("00000020fdf740b0e49cf75bb3d5168fb3586f7613dcc5cd89675b0100000000000000002e37b144c0baced07eb7e7b64da916cd3121f2427005551aeb0ec6a6402ac7d7f0e4235954d801187f5da9f5")
	rawFirstBlock, _ := hex.DecodeString("000000201ecd89664fd205a37566e694269ed76e425803003628ab010000000000000000bfcade29d080d9aae8fd461254b041805ae442749f2a40100440fc0e3d5868e55019345954d80118a1721b2e")

	lastBlock, _ := block.Parse(rawLastBlock)
	firstBlock, _ := block.Parse(rawFirstBlock)

	timeDiff := lastBlock.Timestamp - firstBlock.Timestamp

	twoWeek := 60 * 60 * 24 * 14

	if timeDiff > twoWeek*4 {
		timeDiff = twoWeek * 4
	} else if timeDiff < twoWeek/4 {
		timeDiff = twoWeek / 4
	}

	newTarget := big.NewInt(0).Div(big.NewInt(0).Mul(lastBlock.Target(), big.NewInt(int64(timeDiff))), big.NewInt(int64(twoWeek))).FillBytes(make([]byte, 32))

	fmt.Println(hex.EncodeToString(newTarget))
}
```
```bash
$ go run main.go
0000000000000000007615000000000000000000000000000000000000000000
```

다음 목푯값을 계산하고 나면 이를 비트값으로 변환하여 블록 헤더에 넣어줍니다. 목푯값을 비트값으로 변환하는 연산은 다음과 같습니다.

```go
// 목푯값을 비트로 변환하는 함수
func TargetToBits(target *big.Int) []byte {
	rawBytes := target.Bytes() // 목푯값을 []byte로 변환, 앞에 0은 제외됨

	var exp int
	var coef []byte

	// 만약 rawBytes가 1로 시작하면 음수가 되므로 변환해줌
	if rawBytes[0] > 0x7f {
		exp = len(rawBytes) + 1                      // 0x00을 추가했으므로 지수는 1 증가
		coef = append([]byte{0x00}, rawBytes[:2]...) // 0x00을 추가해줌
	} else {
		exp = len(rawBytes) // 지수
		coef = rawBytes[:3] // 계수
	}

	return append(utils.ReverseBytes(coef), byte(exp)) // 계수를 리틀엔디언으로 변환하고 지수를 뒤에 붙임
}
```

### 연습문제 9.12

```go
func calcNewTargetAndConvertToBits() {
	rawFirstBlock, _ := hex.DecodeString("02000020f1472d9db4b563c35f97c428ac903f23b7fc055d1cfc26000000000000000000b3f449fcbe1bc4cfbcb8283a0d2c037f961a3fdf2b8bedc144973735eea707e1264258597e8b0118e5f00474")
	rawLastBlock, _ := hex.DecodeString("000000203471101bbda3fe307664b3283a9ef0e97d9a38a7eacd8800000000000000000010c8aba8479bbaa5e0848152fd3c2289ca50e1c3e58c9a4faaafbdf5803c5448ddb845597e8b0118e43a81d3")

	firstBlock, _ := block.Parse(rawFirstBlock)
	lastBlock, _ := block.Parse(rawLastBlock)

	timeDiff := lastBlock.Timestamp - firstBlock.Timestamp

	twoWeek := 60 * 60 * 24 * 14

	if timeDiff > twoWeek*4 {
		timeDiff = twoWeek * 4
	} else if timeDiff < twoWeek/4 {
		timeDiff = twoWeek / 4
	}

	newTarget := big.NewInt(0).Div(big.NewInt(0).Mul(lastBlock.Target(), big.NewInt(int64(timeDiff))), big.NewInt(int64(twoWeek)))

	newBits := block.TargetToBits(newTarget)

	fmt.Println(hex.EncodeToString(newBits)) // 80df6217
}
```
```bash
$ go run main.go
80df6217
```

### 연습문제 9.13

block 패키지의 helper.go 파일에 CalculateNewBits 함수를 구현하세요.

```go
func CalculateNewBits(prevBits []byte, timeDiff int64) []byte {
	if timeDiff > TWO_WEEK*4 {
		timeDiff = TWO_WEEK * 4
	} else if timeDiff < TWO_WEEK/4 {
		timeDiff = TWO_WEEK / 4
	}

	newTarget := big.NewInt(0).Div(big.NewInt(0).Mul(BitsToTarget(prevBits), big.NewInt(timeDiff)), big.NewInt(TWO_WEEK))

	return TargetToBits(newTarget)
}
```
# Chapter06 표현식

## 표현식 언어

- 러스트는 표현식 언어(expression language)이다.

#### if 표현식

```rust
let x = 5;
let y = if x == 5 { 10 } else { 15 };
assert_eq!(y, 10);
```

> 러스트는 모든 게 표현식이므로, C의 삼항 연산자와 같은 특별한 표현식이 필요 없다.

---

## 블록과 세미콜론

### 블록 

- 가장 일반적인 종류의 표현식으로 값을 산출하며, 값이 필요한 모든 곳에 쓰일 수 있다.

### 세미콜론

```rust
let msg = {
    let x = 5; // let 선언문: 항상 세미콜론을 붙여야 한다.

    hello(); // 표현식 + 세미콜론: 호출된 함수의 반환값이 드랍된다.

    foo() // 세미콜론이 없는 표현식: 호출된 함수의 반환값이 `msg`에 할당된다.
}; 
```

---

## 선언

- 지역변수 선언
- 아이템 선언 (fn, struct, use 등)

---

## if와 match

### if

```rust
let x = if y == 5 { 10 } else { 15 };
```

- 조건을 괄호 안에 넣지 않는다.
- else if와 else는 선택사항이다.
- if 표현식의 모든 블록은 반드시 같은 타입을 가져야 한다.

### match

```rust
match code {
    0 => println!("OK"),
    1 => println!("Wires Tangled"),
    2 => println!("User Asleep"),
    _ => println!("Unrecognized Error {}", code),
}
```

- switch문과 비슷하나 더 유연하다.
- `_`는 와일드카드 패턴이다. 모든 값에 매칭되며 반드시 마지막에 위치해야 한다.
- 위에서부터 우선순위가 높다.
- 러스트는 주어진 값들을 각 패턴과 비교하며, 패턴이 매칭되면 해당 패턴의 표현식을 실행한다. 이 과정에서 적어도 한 가지 패턴이 매칭되어야 한다. 그렇지 않으면 컴파일러는 에러를 발생시킨다.
- match 표현식의 모든 갈래는 반드시 같은 타입을 가져야 한다.

---

## if let

```rust
if let pattern = expr {
    block1
} else {
    block2
}
```

- 주어진 expr이 pattern과 매칭되면 block1이 실행되고, 그렇지 않으면 block2가 실행된다.
- match로도 같은 효과를 낼 수 있으므로 굳이 사용할 필요는 없다.

---

## 루프

```rust
while condition {
    block
}

while let pattern = expr {
    block
}

loop {
    block
}

for pattern in iterable {
    block
}
```

- while이나 for는 루프의 값이 항상 ()인 반면, loop는 필요할 경우 값을 반환할 수 있다.
- loop는 무한 루프를 작성할 때 쓴다.
- for 루프는 iterable 표현식을 평가한 뒤에 그 결과로 얻은 이터레이터의 개별 값에 대해서 한 번씩 block을 평가한다.

```rust
for i in 0..20 {
    println!("{}", i);
}
```

- `..` 연산자는 범위를 나타낸다. `0..20`은 0부터 19까지의 범위를 나타낸다. `..=`는 범위의 끝을 포함한다.
- for 루프로 값을 반복 처리할 때는 러스트의 이동 의미론에 따라 값이 이동되거나 복사된다.

```rust
let strings: Vec<String> = error_messages();
for s in strings { // 각 String 값은 s로 이동된다.
    println!("{}", s);
}
println!("{} error(s)", strings.len()); // 오류: 이동된 값에 접근
```

- 이동 의미론을 피하려면 참조자를 사용한다.

```rust
let strings: Vec<String> = error_messages();
for s in &strings { // 각 String 값은 참조자로 빌려진다.
    println!("{}", s);
}
println!("{} error(s)", strings.len()); // 참조자를 사용하므로 이동되지 않는다.
```

---

## 루프의 제어 흐름

- 루프의 제어 흐름을 바꾸는 표현식은 `break`, `continue`, `return`이 있다.

### break

```rust
let answer = loop {
    // next_line()은 입력에서 한 줄을 읽어서 반환한다.
    if let Some(line) = next_line() { // 입력이 있으면
        if line.starts_with("answer: ") { // 입력이 "answer: "로 시작하면
            break line; // 루프를 빠져나가고 line을 반환한다.
        }
    } else {
        break "answer: nothing"; // 입력이 없으면 루프를 빠져나가고 "answer: nothing"을 반환한다.
    }
}
```

- 루프 안에 있는 모든 break 표현식은 반드시 같은 타입을 가져야 한다.

### continue

```rust
for line in input_lines {
    let trimmed = trim_comments_and_whitespace(line);
    if trimmed.is_empty() {
        continue; // 루프의 맨 처음으로 돌아가 다음 반복을 시작한다.
    }
    ...
}
```

### 레이블

```rust
'search:
for room in apartment {
    for spot in room.hiding_spots() {
        if spot.contains("salmon") {
            println!("Salmon spotted! in room {}", room.number());
            break 'search; // 레이블을 지정하여 루프를 빠져나간다.
        }
    }
}
```

- 레이블은 `'`로 시작하며, 레이블이 지정된 루프를 빠져나가는 break 표현식에 사용된다.
- 레이블은 continue 표현식에도 사용할 수 있다.

---

## return 표현식

- return 표현식은 현재 함수를 빠져나와 호출부에 값을 반환한다. 값이 없는 return은 return ()의 축약형이다.
- 마지막 표현식이 세미콜론으로 끝나지 않으면, 그 표현식의 값이 함수의 반환값이 된다.

```rust
fn f() {
    return; // return ()과 같다.
}
```

---

## 러스트에 loop가 있는 이유

### 러스트 컴파일러의 제어 흐름 분석

- 함수의 모든 경로가 예정된 반환 타입의 값을 반환하는지 검사한다. 이를 제대로 수행하기 위해 함수의 끝에 도달하는 것이 가능한지 아닌지 알아야 한다.
- 지역변수가 초기화되지 않은 채로 쓰이는지 검사한다. 여기에는 함수의 모든 경로를 검사해서 지역변수가 초기화되지 않은 채로 쓰이는 경로가 있는지 검사하는 작업이 수반된다.
- 도달할 수 없는 코드에 대해서 경고를 내보낸다.

> 이러한 제어 흐름 분석을 가리켜 `흐름을 고려한 분석`이라고 한다.

- 러스트의 흐름을 고려한 분석은 루프 조건을 아예 검사하지 않는 대신, 단순히 프로그램의 모든 조건이 참이나 거짓일 수 있다고 가정한다. 이로 인해 러스트는 안전한 프로그램을 일부 거부하기도 한다.
- loop 표현식은 이러한 문제를 '니가 진짜로 원하는 게 뭐야'식으로 해결한다.
- loop 표현식처럼 정상적으로 끝나지 않는 표현식에는 `!`라는 특수한 타입이 배정되고 타입 일치에 관한 규칙에서 면제된다.

```rust
fn serve_forever(socket: ServerSocket, handler: ServerHandler) -> ! {
    socket.listen();
    loop {
        let s = socket.accept();
        handler.handle(s);
    }
}
```

- 위와 같은 함수를 `일탈 함수(diverging function)`라고 한다.

---

## 함수와 메서드 호출

- 러스트는 대부분의 다른 언어와 마찬가지로 함수 호출과 메서드 호출 문법이 같다.
- 러스트는 값과 레퍼런스를 명확히 구분하는데 `.` 연산자는 이런 규칙을 조금 완화해 준다.
- 타입 연관 함수를 호출할 때는 `::` 연산자를 사용한다.

```rust
let mut numbers = Vec::new(); // 타입 연관 함수 호출
```

- 러스트 문법의 한 가지 특이한 점은 Vec<T>와 같은 제너릭 타입의 타입 연관 함수를 호출할 때는 타입 인자를 사용하지 않는다는 점이다.

```rust
let mut numbers = Vec<i32>::with_capacity(1000); // 비교와 관련된 오류가 발생
let ramp = (0..n).collect<Vec<i32>>(); // 비교와 관련된 오류가 발생
```

- 표현식에서 `<`가 비교 연산자로 해석되기 때문에 위와 같은 코드는 오류를 발생시킨다.
- 타입 인자를 명시적으로 지정하려면 `::<...>` 구문을 사용한다.

```rust
let mut numbers = Vec::<i32>::with_capacity(1000);
let ramp = (0..n).collect::<Vec<i32>>();
```

- 타입 인자를 명시적으로 지정하지 않으면 컴파일러가 타입 인자를 추론한다.

```rust
let mut numbers = Vec::with_capacity(1000); // 타입 인자를 명시적으로 지정하지 않음
let ramp = (0..n).collect(); // 타입 인자를 명시적으로 지정하지 않음
```

---

## 필드와 요소

- 스트럭트와 튜플의 필드에 접근할 때는 `.` 연산자를 사용한다.
- 다만 스트럭트는 필드 이름을 사용하고, 튜플은 0부터 시작하는 인덱스를 사용한다. (game.block, coords.1)
- 배열, 슬라이스, 벡터의 요소에 접근할 때는 `[]` 연산자를 사용한다. (numbers[0])
- 이 세 가지 표현식은 배정문 왼편에 올 수 있다고 해서 L-value라고 부른다.

### .. 연산자

```rust
.. // 전체 범위
a .. // a에서 끝까지
.. b // 처음부터 b까지 (b는 포함되지 않음)
a .. b // a에서 b까지 (b는 포함되지 않음)
..= b // 처음부터 b까지 (b 포함)
a ..= b // a에서 b까지 (b 포함)
```

- 범위는 시작값을 포함하고 있는 경우에만 반복 처리가 가능하다.
- 배열을 자를 때는 여섯 가지 형식이 모두 유효하다.

---

## 레퍼런스 연산자

- `&`와 `&mut` 연산자는 레퍼런스를 만든다.
- `*` 연산자는 레퍼런스를 역참조한다.

---

## 산술, 비트별, 비교, 논리 연산자

- 오버플로를 검사하지 않는 `wrapping_*`와 같은 메서드가 있다.
- 정수를 나눌 때 `checked_div`와 같은 메서드를 사용하면 패닉에 빠지지 않고 `None`을 반환한다.
- 단항 연산자 `-`는 수의 부호를 뒤집는다. 부호 없는 정수에는 적용할 수 없다.
- 비트별 연산자 `&`, `|`, `^`, `!`는 정수 타입에만 적용할 수 있다. `!`는 논리 부정 연산자이다.
- 러스트에서 비트별 연산자는 비교 연산자보다 우선순위가 높다.

---

## 배정

- `=` 연산자는 러스트에서 배정 연산자이다. 러스트에서는 기본적으로 변수를 변경할 수 없기 때문에 많이 쓰이지 않는다.
- `+=`, `-=`, `*=`, `/=`, `%=`, `<<=`, `>>=`, `&=`, `|=`, `^=` 연산자는 배정 연산자이다.
- 러스트는 `++`, `--` 연산자를 지원하지 않는다.

---

## 타입 캐스팅

- `as` 연산자는 타입 캐스팅을 수행한다.

### 허용되는 캐스팅 종류

- 정수 타입에서 정수 타입으로 캐스팅
    - 좁은 타입으로 캐스팅 -> 잘려나감
    - 넓은 타입으로 캐스팅 -> 부호가 있으면 부호 확장, 부호가 없으면 0으로 확장
- 부동소수점 타입에서 정수 타입으로 캐스팅: 0에 가까운 쪽으로 반올림
- bool, char, enum 타입의 값은 어떤 정수 타입으로든 캐스팅 가능 (반대는 불가능)
- u8은 char 타입으로 캐스팅 가능

### 자동 변환되는 일부 타입

- &String -> &str
- &Vec<T> -> &[T]
- &Box<T> -> &T

- 이 변환들은 기본 제공 트레이트 `Deref`를 구현한 타입에 대해서만 적용된다고 해서 `Dref 강제 변환`이라고 부른다.
- Dref 강제 변환의 목적은 Box 같은 스마트 포인터 타입을 최대한 실제 값처럼 행동하도록 만드는 것이다. (자세한 내용은 13장 참고)

---

## 클로저

- 세로 막대(|)로 둘러싸인 인자 목록과 그 뒤에 오는 표현식으로 구성된다.

```rust
let is_even = |x| x % 2 == 0;
```

- 필요에 따라 클로저의 인수 타입과 반환 타입을 명시할 수 있다.

```rust
let is_even = |x: i32| -> bool { x % 2 == 0 };
```

- 클로저 호출은 함수와 동일하다.

```rust
assert_eq!(is_even(4), true);
```
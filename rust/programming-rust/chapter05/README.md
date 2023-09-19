# Chapter05 레퍼런스

레퍼런스는 자신이 가리키는 대상보다 절대로 더 오래 살아 있으면 안된다. 이를 강조하기 위해 레퍼런스를 만드는 걸 두고 값을 `빌려 온다`고 표현한다. 빌려온 것은 반드시 돌려줘야 한다.

## 값의 레퍼런스

```rust
use ::std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    // table의 소유권을 가져오면서 table의 값을 비움
    for (artist, works) in table {
        println!("works by {}:", artist);
        // works의 소유권을 가져오면서 works의 값을 비움
        for work in works {
            println!("  {}", work);
        }
        // works는 미초기화 상태가 됨
    }
    // table은 미초기화 상태가 됨
}

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    
    ...

    show(table); // table의 소유권을 넘겨줌
}
```

그저 출력만 했을 뿐인데 `table`과 `works`의 소유권을 가져와서 비워버렸다. 이를 방지하기 위해 레퍼런스를 사용해야 한다.

레퍼런스에는 두 가지가 있다.

1. 공유된 레퍼런스(shared reference): `&T` 형태로 표현하며, `T`의 값을 빌려온다. 참조하는 대상을 읽을 수만 있고, 변경할 수는 없다. 레퍼런스를 여러 개 만들어도 된다. 공유된 레퍼런스는 Copy 타입이다.
2. 변경할 수 있는 레퍼런스(mutable reference): `&mut T` 형태로 표현하며, `T`의 값을 빌려온다. 참조하는 대상을 읽고 쓸 수 있다. 레퍼런스를 하나만 만들 수 있다. 변경할 수 있는 레퍼런스는 Copy 타입이 아니다.

공유된 레퍼런스와 변경할 수 있는 레퍼런스를 굳이 구분하는 이유는 컴파일 시점에 `멀티플 리더 또는 싱글 라이터` 규칙을 시행하기 위해서라고 볼 수 있다. 이 규칙은 빌려온 값의 소유자에게도 적용된다. 공유된 레퍼런스가 존재하는 동안에는 참조하는 대상을 변경할 수 없다. 마찬가지로 변경할 수 있는 레퍼런스가 존재하는 동안에는 참조하는 대상에 대한 독점적인 접근 권한을 가진다. 이처럼 공유와 변경을 구분하는 것은 러스트의 메모리 안전성을 지키는 데 가장 중요한 요소 중 하나이다.

앞의 예제를 레퍼런스를 사용해 수정해보자.

```rust
use ::std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    // table의 레퍼런스를 가져오면서 table의 소유권을 가져오지 않음
    // HashMap의 공유된 레퍼런스를 반복 처리할 때는 각 요소에 대해서도 공유된 레퍼런스를 만들도록 정의되어 있음
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    
    ...

    show(&table); // table의 레퍼런스를 넘겨줌
}
```

다음은 변경할 수 있는 레퍼런스를 사용한 예제이다.

```rust
fn sort_works(table: &mut Table) {
    for (_, works) in table {
        works.sort(); // works의 변경할 수 있는 레퍼런스를 만들어 works를 정렬
    }
}

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    
    ...

    sort_works(&mut table); // table의 변경할 수 있는 레퍼런스를 넘겨줌
}
```

---

## 레퍼런스 다루기

### C++ 레퍼런스와 러스트 레퍼런스의 차이점

C++에서는 레퍼런스가 변환에 의해 암묵적으로 생성되고 역참조도 암묵적으로 이루어진다. 러스트에서는 레퍼런스를 명시적으로 생성하고 역참조도 명시적으로 이루어진다.

```rust
let x = 10;
let r = &x; // 명시적으로 레퍼런스를 생성
assert(*r == 10); // 명시적으로 역참조
```

그런데 앞선 예제에서 해시맵의 레퍼런스를 가져와 출력할 때 역참조 없이도 값을 가져올 수 있었다. 이건 왜 그럴까?

러스트에서는 레퍼런스가 아주 폭넓게 쓰이므로 `.` 연산자는 암묵적으로 역참조를 수행한다. 

```rust
struct Anime {
    name: &'static str,
    bechdel_pass: bool,
}

let aria = Anime {
    name: "Aria: The Animation",
    bechdel_pass: true,
};
let anime_ref = &aria;
assert_eq!(anime_ref.name, "Aria: The Animation"); // 암묵적으로 역참조
assert_eq!((*anime_ref).name, "Aria: The Animation"); // 명시적으로 역참조
```

또한 `.` 연산자는 메서드를 호출할 때 피연산자의 레퍼런스를 생성하여 전달할 수도 있다. Vec의 sort 메서드가 그 예이다.

```rust
let mut v = vec![1973, 1968];
v.sort(); // v의 레퍼런스를 생성하여 sort에 전달
(& mut v).sort(); // 위와 동일하게 동작, 코드가 길고 복잡해짐
```

### 레퍼런스 배정하기

레퍼런스를 변수에 배정하면 그 변수는 새 위치를 가리키게 된다.

```rust
let x = 10;
let y = 20;
let mut r = &x;

if b {
    r = &y;
}

assert!(*r == 10 || *r == 20);
```

이 코드에서 레퍼런스 r은 처음에 x를 가리키다가 b가 참이면 y를 가리킨다. 반면 C++는 한 번 초기화되고 나면 절대로 다른 것을 가리킬 수 없다.

### 레퍼런스의 레퍼런스

러스트에서는 레퍼런스의 레퍼런스를 만들 수 있다.

```rust
struct Point {
    x: i32,
    y: i32,
}
let point = Point { x: 1000, y: 729 };
let r = &point;
let rr = &r;
let rrr = &rr;
    
assert_eq!(rrr.y, 729);
```

`rrr.y`가 729를 반환하는 이유는 `.` 연산자가 레퍼런스가 최종적으로 가리키는 대상을 찾을 때까지 연결된 레퍼런스를 따라가기 때문이다.

### 레퍼런스 비교하기

```rust
let x = 10;
let y = 10;

let rx = &x;
let ry = &y;

let rrx = &rx;
let rry = &ry;

assert!(rrx <= rry);
assert!(rrx == rry);
```

러스트의 비교 연산자도 `.` 연산자와 마찬가지로 레퍼런스가 최종적으로 가리키는 대상을 비교한다.

만일 두 레퍼런스가 같은 메모리를 가리키는지 알고 싶다면, `std::ptr::eq` 함수를 사용해야 한다.

```rust
assert!(!std::ptr::eq(rx, ry));
```

비교할 때 대상이 되는 피연산자는 반드시 같은 타입이어야 하며, 레퍼런스의 경우도 마찬가지다.

```rust
assert!(rx == rrx); // 타입 불일치: &i32 != &&i32
assert!(rx == *rrx); // 문제 없음: &i32 == &i32
```

### 레퍼런스는 절대로 널이 될 수 없다

러스트의 레퍼런스는 절대로 널이 될 수 없다.

러스트에서는 무언가의 레퍼런스일 수도 있고 아닐 수도 있는 값이 필요할 때 `Option<&T>` 타입을 사용한다. 이 타입은 `Some(&T)` 또는 `None` 둘 중 하나의 값을 가질 수 있다.

### 임의의 표현식을 가리키는 레퍼런스 빌려 오기

러스트는 어떤 종류의 표현식이든 가리지 않고 거기서 산출되는 값의 레퍼런스를 빌려올 수 있다.

```rust
fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}
// factorial(6)을 익명변수에 배정한 후 그 변수의 레퍼런스를 r에 배정
// 익명변수의 수명은 r의 수명과 같다.
let r = &factorial(6); 
assert_eq!(r + &1009, 1729); // 1009를 보관하기 위해 생성된 임시변수는 assert_eq! 문이 끝날 때까지만 유효하다.
```

### 슬라이스 레퍼런스와 트레이트 객체

러스트를 2가지 종류의 팻 포인터(fat pointer)를 더 가지고 있다. 팻 포인터는 어떤 값의 주소와 그 값을 사용하는 데 필요한 추가 정보를 갖는 2워크 크기의 값이다.

1. 슬라이스 레퍼런스: `&[T]` 형태로 표현하며, `T` 타입의 값들의 슬라이스를 빌려온다. 슬라이스의 시작 주소와 길이를 갖는 팻 포인터이다.
2. 트레이트 객체: 트레이트를 구현하고 있는 값의 레퍼런스다. 트레이트 객체는 트레이트의 메서드 호출에 필요한 정보인 값의 주소와 그 값에 알맞는 트레이트 구현체의 주소를 갖는 팻 포인터이다.

이렇게 추가 정보를 가진다는 것을 제외하면 슬라이스 레퍼런스와 트레이트 객체는 일반적인 레퍼런스와 비슷하다.
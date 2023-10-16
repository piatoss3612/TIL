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
2. 변경할 수 있는 레퍼런스(mutable reference): `&mut T` 형태로 표현하며, `mut T`의 값을 빌려온다. 참조하는 대상을 읽고 쓸 수 있다. 레퍼런스를 하나만 만들 수 있다. 변경할 수 있는 레퍼런스는 Copy 타입이 아니다.

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

---

## 레퍼런스 안전성

### 수명

- 러스트는 프로그램에 있는 모든 레퍼런스 타입을 대상으로 **수명**을 부여한다.
- 수명이란 레퍼런스가 유효한 구간을 의미한다. 수명은 컴파일 시점에만 존재하는 가상의 개념이다.
- 레퍼런스의 수명은 레퍼런스가 가리키는 대상의 수명과 같거나 더 짧아야 한다.

### 함수 인수로 레퍼런스를 전달하기

#### 레퍼런스를 인수로 받는 함수

```rust
static mut STASH: &i32; // 전역 변수 STASH
fn f(p: &i32) {
    STASH = p; // STASH에 p의 레퍼런스를 저장
}
```

#### 위 코드의 문제점

- 모든 static 변수는 초기화가 되어야 한다.
- `f` 함수는 `p`의 레퍼런스를 `STASH`에 저장한다. 이는 `p`의 수명이 `STASH`의 수명보다 길어야 한다는 것을 의미한다.
- 변경할 수 있는 static은 스레드 안전하지 않다. 따라서 unsafe 블록 안에서만 사용할 수 있다.

#### 수정된 코드

```rust
static mut STASH: &i32 = &128;
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}
```

- `p`의 수명을 `'static`으로 지정했다. 이는 `p`의 수명이 `'static`과 같거나 더 길어야 한다는 것을 의미한다.

```rust
#[test]
fn test_static_lifetime() {
    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);

    // let x = 10;
    // f(&x);
}
```

- `f`는 수명이 `'static`인 레퍼런스만 받을 수 있다. 따라서 `WORTH_POINTING_AT`의 레퍼런스를 전달할 수 있다.
- `x`는 수명이 `'static`이 아니므로 `f`에 전달할 수 없다.

> 함수 시그니처는 본문의 행동을 드러낸다. 이는 함수 호출의 안전성을 보장하는 데 중요한 역할을 한다.

### 레퍼런스 반환하기

- 러스트는 함수의 인수가 하나 뿐이고 인수와 반환값의 타입이 모두 레퍼런스일 때, 이 둘의 수명이 같다고 가정한다.

### 레퍼런스를 갖는 스트럭트

```rust
#[test]
fn test_static_field_lifetime() {
    struct S {
        r: &'static i32,
    }

    let s;
    {
        static X: i32 = 10;
        s = S { r: &X };
    }

    assert_eq!(*s.r, 10);

    struct S2<'a> {
        r: &'a i32,
    }

    struct D {
        s: S2<'static>,
    }

    struct D2<'a> {
        s: S2<'a>,
    }
}
```

### 고유한 수명 매개 변수

```rust
#[test]
fn test_unique_lifetime() {
    // struct S<'a> {
    //     x: &'a i32,
    //     y: &'a i32,
    // }

    // let x = 10;
    // let r;
    // {
    //     let y = 20;
    //     {
    //         let s = S { x: &x, y: &y }; // s의 x와 y의 수명이 동일해야 하는데 다름
    //         r = s.x;
    //     }
    // }

    // println!("{}", r);

    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s: S<'_, '_> = S { x: &x, y: &y }; // s의 x와 y의 수명이 달라도 됨
            r = s.x;
        }
    }

    println!("{}", r);
}
```

### 수명 매개변수 생략하기

- 코드가 아주 단순할 때는 수명 매개변수를 생략할 수 있다.
- 함수가 어떤 타입의 메서드이면서 self 매개변수를 레퍼런스로 받는다면, self의 수명이 반환값에 배정된다.

```rust
struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }

    fn find_by_prefix2<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}
```

---

## 공유 vs 변경

- 공유된 레퍼런스는 살아 있는 동안 참조 대상을 읽기 전용으로 설정해 두므로 참조 대상에 배정하거나 해당 값을 다른 곳으로 옮길 수 없다.

### 공유에 관한 규칙

- 공유된 접근은 읽기 전용 접근이다
- 변경할 수 있는 접근은 배타적인 접근이다

> 이러한 규칙으로 인해 unsafe를 사용하지 않는 동시적 러스트 프로그램은 구조적으로 데이터 경합(race condition)이 발생하지 않는다.

### 객체의 바다와 맞서기

- 러스트는 가비지 컬렉션을 사용하는 프로그램이 서로가 서로에게 의존하는 것과 달리, 포인터, 소유권, 데이터 흐름이 시스템 전반에서 한 방향으로 흐르는 것을 선호한다. (Rc같은 스마트 포인터 타입을 사용하지 않는한)
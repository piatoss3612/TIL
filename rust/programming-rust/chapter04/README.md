# Chapter04 소유와 이동

1. 가비지 컬렉션을 사용하여 메모리를 관리하는 경우
    - 대상을 잃은 포인터가 생기는 것을 방지
    - 그러나 객체에 대한 통제를 잃는다.
2. 엔지니어가 메모리를 관리하는 경우
    - 객체에 대한 통제를 유지
    - 그러나 대상을 잃은 포인터가 생길 수 있다.

러스트는 프로그램이 구사할 수 있는 포인터의 용법에 제약을 두는 기발한 방법으로 이 문제를 해결한다.

## 소유

러스트에서는 소유의 개념이 언어 자체에 내장되어 있으며, 컴파일 시점 검사를 통해 검증된다. 따라서 소유권과 관련된 어떤 것도 런타임 비용을 발생시키지 않는다.

### 소유권 규칙

1. 모든 값은 자신의 수명을 결정하는 소유자가 존재한다.
2. 오직 하나의 소유자만이 그 값의 소유자가 될 수 있다.
3. 소유자가 해제(드롭)될 때 그가 소유한 값도 드롭된다.

```rust
fn print_hello_world() {
    let mut s = String::from("hello"); // s는 String 값의 소유자
    s.push_str(", world!"); // String 값에 문자열을 추가
    println!("{}", s); // hello, world!
} // s가 스코프를 벗어나면 String 값이 드롭된다.
```

s의 버퍼의 포인터, 길이, 용량을 가리키는 세 개의 워드가 print_hello_world 함수의 스택 프레임에 저장된다.
버퍼는 힙에 저장되어 있으며, 문자열 데이터를 가지고 있다.
s는 버퍼를 소유하고 있으며, 변수 s가 함수의 끝에서 스코프를 벗어나면 String이 드롭되고, 버퍼도 해제된다.

Box 타입은 소유의 또 다른 예이다. Box는 힙에 저장된 값에 대한 포인터이며, Box가 스코프를 벗어나면 그 값도 드롭된다.
Box::new(v) 호출은 힙 공간을 할당하고, 값 v를 그리로 옮긴 뒤, 그 힙 공간을 가리키는 포인터를 반환한다.

```rust
{
    let point = Box::new((0.625, 0.5)); // 튜플을 박스로 감싼다.
    let label = format!("{:?}", point); // 포맷 매크로를 사용한다.
    assert_eq!(label, "(0.625, 0.5)");
} // point와 label이 스코프를 벗어나면 드롭된다.
```

변수가 자신의 값을 소유하듯이 스트럭트는 자신의 필드를 소유한다. 마찬가지로 튜플, 배열, 벡터 역시 자신의 요소를 소유한다.

```rust
fn print_struct() {
    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = Vec::new(); // composers는 Vec<Person> 값의 소유자
    composers.push(Person {
        // Person 구조체는 자신의 필드의 소유권을 가진다.
        name: "Palestrina".to_string(), // 문자열 필드는 String 값의 소유권을 가진다.
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });

    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
} // composers가 스코프를 벗어나면 Vec<Person> 값이 드롭된다. 그리고 관련된 모든 값들도 드롭된다.
```

문자열 필드는 String 값의 소유권을 가지고 있으며, Person 구조체는 자신의 필드의 소유권을 가진다. 그리고 Vec<Person> 값은 자신의 요소의 소유권을 가진다. 이렇듯 소유자와 이들이 소유한 값은 트리 형태를 이룬다. 각 트리의 최종 루트는 변수다. 이 변수가 스코프를 벗어나면 트리의 모든 값이 드롭된다.

러스트는 C나 C++이 free와 delete를 호출하는 것과 달리, 소유자가 스코프를 벗어나거나 벡터에서 요소를 제거하는 등의 방법을 통해 값이 드롭되는 것을 보장한다.

소유의 개념은 때로는 너무 엄격해서 사용하기가 어렵다. 이에 러스트는 다음과 같은 방법을 통해서 소유의 개념을 확장한다.

1. 값을 한 소유자에게서 다른 소유자로 옮길 수 있다.
2. 정수, 부동소수점 수, 문자 같은 아주 단순한 타입들은 소유 규칙의 적용 대상에서 제외된다. 이런 타입을 Copy 타입이라고 한다.
3. 표준 라이브러리를 통해 러스트는 Rc<T>와 Arc<T> 같은 타입을 제공한다. 이 타입들은 여러 소유자를 허용한다.
4. 값의 레퍼런스를 빌릴 수 있다. 레퍼런스는 한정된 수명을 가진 소유권 없는 포인터이다.

---

## 이동

러스트에서는 값을 변수에 할당하거나, 값을 함수에 전달하거나, 값을 함수에서 반환하는 등의 연산이 일어날 때마다 값이 복사되지 않고 이동한다. 이때 원래 주인의 소유권은 새로운 주인으로 이동하고 원래 변수는 미초기화 상태가 된다.

```rust
let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()]; // s는 Vec<String> 값의 소유자
let t = s; // s의 소유권이 t로 이동한다.
let u = s; // s의 소유권이 이미 t로 이동했으므로 미초기화 상태인 s를 사용할 수 없다. 따라서 컴파일 에러가 발생한다.
```

러스트는 이동을 통해 할당 비용을 최소화하고, 레퍼런스 카운팅이나 가비지 컬렉션의 도움 없이도 메모리의 해제 시점을 추적할 수 있다.

만약 복사본이 필요하다면 clone 메서드를 사용하면 된다. clone 메서드는 벡터의 모든 요소들에 대해 깊은 복사를 수행한다.

```rust
let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
let t = s.clone(); // s의 복사본을 만든다.
let u = s.clone(); // s의 복사본을 만든다.
```

### 이동으로 처리되는 그 밖의 연산들

1. 값을 이미 초기화된 변수에 할당하는 경우

```rust
let mut s = "Govinda".to_string();
s = "Siddhartha".to_string(); // 기존의 String 값은 드롭된다.
```

2. 미초기화 상태의 변수에 값을 할당하는 경우

```rust
let mut s = "Govinda".to_string();
let t = s; // s의 소유권이 t로 이동한다.
s = "Siddhartha".to_string(); // 미초기화 상태인 s에 새로운 String 값이 할당된다.
```

이 외에도 러스트는 값이 쓰이는 거의 모든 곳에서 이동 의미론(move semantics)을 적용한다.

이동에 대해 명심해야 할 것이 두 가지 있다.

1. 이동의 적용 대상은 값 자체이다. 그 값이 가리키는 힙 공간은 이동의 영향을 받지 않는다.
2. 러스트 컴파일러가 이 모든 것을 꿰뚫어 보고 그에 알맞는 코드를 생성해 낼 수 있다.

### 이동 제어 흐름

1. if 표현식의 조건절에서

```rust
let x = vec![10, 20, 30];
if c {
    f(x); // x의 소유권이 f로 이동할 수도 있고
} else {
    g(x); // g로 이동할 수도 있다.
}
h(x); // 조건절이 끝나면 x는 미초기화 상태가 된다. 따라서 h는 미초기화 상태인 x를 사용할 수 없다.
```

2. 루프 안에서

```rust
let x = vec![10, 20, 30];
while f() {
    g(x); // x는 첫 번째 반복에서 g로 이동하고 미초기화 상태가 된다. 따라서 두 번째 반복에서는 g를 호출할 수 없다.
}
```
```rust
let mut x = vec![10, 20, 30];
while f() {
    g(x); // x는 g로 이동하고 미초기화 상태가 된다.
    x = h(); // h의 반환값이 x로 이동한다. 따라서 다음 반복에서는 g를 호출할 수 있다.
}
e(x); // x에 h()의 반환값이 할당되었으므로 x는 미초기화 상태가 아니다.
```

### 색인을 써서 접근하는 콘텐트의 이동

모든 종류의 값 소유자가 미초기화 상태가 될 수 있는 것은 아니다. 

```rust
let mut v = Vec::new();
for i in 101 .. 106 {
    v.push(i.to_string());
}

let third = v[2]; // 불가능. v[2]의 소유권을 third로 이동하게 되면 v[2]는 미초기화 상태가 되어야 하는데 이 정보를 추적할 수 없다.
```

이런 경우 러스트는 색인을 써서 접근하는 콘텐트의 이동을 허용하지 않는다.

벡터의 요소를 밖으로 옮기고 싶다면 어떻게 해야 할까? 다음과 같이 세 가지 정도의 방법이 있다.

```rust
let mut v = Vec::new();
for i in 101..106 {
    v.push(i.to_string());
}

// 1. 벡터 끝에 있는 값을 꺼낸다.
let fifth = v.pop().expect("vector empty!");
assert_eq!(fifth, "105");

// 2. 주어진 색인에 있는 값을 벡터 밖으로 옮기고, 마지막 요소를 그 자리로 옮긴다.
let second = v.swap_remove(1);
assert_eq!(second, "102");

// 3. 꺼내려는 값을 다른 값과 맞바꾼다.
let third = std::mem::replace(&mut v[2], "substitute".to_string());
assert_eq!(third, "103");

assert_eq!(v, vec!["101", "104", "substitute"]);
```

컴파일러가 추적할 수 없는 소유자의 값을 옮겨야 할 때는 소유자의 타입을 값 보유 여부가 동적으로 추적되는 것으로 바꾸는 게 좋다.

```rust
struct Person {
    name: Option<String>,
    birth: i32,
}

let mut composers = Vec::new();
composers.push(Person {
    name: Some("Palestrina".to_string()),
    birth: 1525,
});

// let first_name = composers[0].name; // 이렇게 하면 cannot move out of index 발생
let first_name = std::mem::replace(&mut composers[0].name, None);
assert_eq!(first_name, Some("Palestrina".to_string()));
assert_eq!(composers[0].name, None);

// 이런 식으로 Option을 사용하는 경우가 많아 관련된 메서드가 존재한다.
let first_name = composers[0].name.take(); // take 메서드는 Option 값을 꺼내고 그 자리에 None을 놓는다.
assert_eq!(first_name, None);
```

---
# Chapter10 이늄과 패턴

## 이늄

```rust
enum Ordering {
    Less,
    Equal,
    Greater,
}
```

- 이늄은 후보값들의 집합으로, 이 때 후보 값은 베리언트(variant) 또는 생성자(constructor)라고 부른다.
- 앞서 살펴본 `Ordering`은 표준 라이브러리에 정의된 이늄이다.

```rust
use std::cmp::Ordering;

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
```

- 아래와 같이 이늄 자체와 베리언트를 모두 가져올 수도 있다.

```rust
use std::cmp::Ordering::{self, *};

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Less
    } else if n > m {
        Greater
    } else {
        Equal
    }
}
```

- 경우에 따라서는 이늄에 사용할 값을 지정해줄 수도 있다. 기본적으로는 0부터 시작하여 1씩 증가하는 값이 사용된다.

```rust
enum Ordering {
    Less = -1,
    Equal = 0,
    Greater = 1,
}
```

- 일반적으로 이늄은 가장 작은 크기의 정수 타입으로 표현할 수 있는 크기를 가진다. (1바이트)
- 이늄을 정수로 캐스팅할 수 있지만, 반대로 정수를 이늄으로 캐스팅할 수는 없다.

- 이늄도 스트럭트와 마찬가지로 메소드를 가질 수 있다.

### 데이터를 갖는 이늄

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

- V4와 V6는 튜플 베리언트이다. 각각 4개의 u8 타입의 값을 갖는 튜플과 String 타입의 값을 갖는 튜플이다.
- 이늄은 스트럭트 베리언트를 가질 수도 있다.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
}
```

### 이늄의 메모리 구조

- 이늄은 작은 정수 태그(어떤 생성자를 사용했고 어떤 필드를 갖는지에 대한 정보)와 생성자에 따라 달라지는 데이터로 구성된다.
- 하지만 레이아웃이 정해져 있지 않기 때문에, 최적화의 여지를 남겨둔다.

### 이늄을 이용한 리치 데이터 구조

- 이늄은 리치 데이터 구조를 표현하는 데에도 사용할 수 있다.

```rust
use std::collections::HashMap;

enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}
```

### 제네릭 이늄

- 이늄은 제네릭 타입 파라미터를 가질 수 있다.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

- 제네릭 이늄의 문법은 제네릭 스트럭트와 동일하다.
- 한 가지 특이한 것은 T가 Box이거나 기타 스마트 포인터 타입일 경우, 러스트가 태그를 제거할 수 있다는 것이다.

```rust
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

fn binary_tree() {
    let jupiter_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));

    let mercury_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));

    let mars_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: mercury_tree,
    }));

    let venus_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Venus",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));

    let uranus_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Uranus",
        left: BinaryTree::Empty,
        right: venus_tree,
    }));

    let tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: uranus_tree,
    }));
}
```

- 이늄의 문제는 필드에 직접 접근할 수 없다는 것이다. 필드에 접근하기 위해서는 패턴 매칭을 사용해야 한다.

## 패턴

- 이늄 베리언트의 필드에 접근하기 위해서는 패턴 매칭을 사용해야 한다.

```rust
match tree {
    BinaryTree::Empty => println!("empty"),
    BinaryTree::NonEmpty(ref node) => println!("non empty: {}", node.element),
}
```

### 패턴에 쓰이는 리터럴, 변수, 와일드카드

- 패턴에는 리터럴, 변수, 와일드카드를 사용할 수 있다.

```rust
match n {
    0 => println!("zero"),
    1 => println!("one"),
    _ => println!("other"),
}

match n {
    0 => println!("zero"),
    1 => println!("one"),
    n => println!("other: {}", n),
}

match calendar = match settings.get_string("calendar") {
    "gregorian" => Calendar::Gregorian,
    "chinese" => Calendar::Chinese,
    "ethiopic" => Calendar::Ethiopic,
    other => return Err(format!("'{}' is not a valid calendar", other)),
}
```

- `n`과 `other`는 범용 패턴이다. 이는 어떤 값이든 매칭시킬 수 있다는 것을 의미한다.
- `_`는 와일드카드 패턴이다. 이는 어떤 값이든 매칭시키지만, 매칭된 값을 사용하지 않는다는 것을 의미한다.
- 와일드카드 패턴은 모든 경우와 매칭되기 때문에, 맨 마지막에 사용해야 한다.

### 튜플 패턴과 스트럭트 패턴

- 튜플 패턴을 튜플을 매칭시키는 데에 사용할 수 있다.

```rust
match pair {
    (0, 0) => println!("origin"),
    (0, y) => println!("x axis, y = {}", y),
    (x, 0) => println!("y axis, x = {}", x),
    (x, y) => println!("({}, {})", x, y),
}
```

- 스트럭트 패턴은 스트럭트를 매칭시키는 데에 사용할 수 있다.

```rust
match point {
    Point { x: 0, y: 0 } => println!("origin"),
    Point { x: 0, y } => println!("x axis, y = {}", y),
    Point { x, y: 0 } => println!("y axis, x = {}", x),
    Point { x, y } => println!("({}, {})", x, y),
}
```

### 배열 패턴과 슬라이스 패턴

- 배열 패턴은 배열을 매칭시키는 데에 사용할 수 있다.

```rust
match pair {
    [0, 0] => println!("origin"),
    [0, y] => println!("x axis, y = {}", y),
    [x, 0] => println!("y axis, x = {}", x),
    [x, y] => println!("({}, {})", x, y),
}
```

- 슬라이스 패턴은 슬라이스를 매칭시키는 데에 사용할 수 있다.

```rust
match names {
    [] => println!("empty list"),
    [a] => println!("one element list: {}", a),
    [a, b] => println!("two element list: {} and {}", a, b),
    [a, .., z] => println!("some elements: {} and {}", a, z),
}
```

### 레퍼런스 패턴

- 매칭된 값의 일부를 빌려오기 위해서는 레퍼런스 패턴을 사용할 수 있다.

```rust
match account {
    Account { ref name, ref language, .. } => {
        ui.greet(name, language);
        ui.show_settings(&account); // name과 language의 소유권은 여전히 account에 있음
    }
}
```

- ref mut을 사용하면 가변 레퍼런스를 얻을 수 있다.

- `&`로 시작하는 패턴은 레퍼런스를 매칭시키는 데에 사용할 수 있다.

```rust
match sphere.center() {
    &Point3D { x, y, z } => println!("center at ({}, {}, {})", x, y, z),
}
```

- `&`이 아니라 `*`를 사용해 값을 불러와야 하지 않을까 싶지만 **패턴과 표현식은 서로 반대의 관계**이기 때문에 `&`를 사용해야 한다.
- 표현식 `(x, y)`는 두 값을 튜플로 만들지만, 패턴 `(x, y)`는 튜플을 매칭해서 두 값으로 쪼갠다. 마찬가지로 표현식 `&`는 레퍼런스를 만들지만, 패턴 `&`는 레퍼런스를 매칭시킨다.

### 매치 가드

- 매치 가드는 매칭된 값에 추가적인 조건을 걸 수 있게 해준다.

```rust
match pair {
    (x, y) if x == y => println!("equal"),
    (x, y) if x + y == 0 => println!("equal zero"),
    (x, _) if x % 2 == 0 => println!("first is even"),
    _ => println!("no match"),
}
```

### 여러 가능성 매칭하기

- `|`를 사용해 여러 가능성을 매칭시킬 수 있다.

```rust
match n {
    0 | 1 => println!("small"),
    2 | 3 => println!("medium"),
    _ => println!("large"),
}
```

- 또는 `..=`를 사용해 범위를 매칭시킬 수도 있다.

```rust
match n {
    0..=9 => println!("small"),
    10..=99 => println!("medium"),
    _ => println!("large"),
}
```

### @ 패턴으로 바인딩하기

- `@` 패턴을 사용해 매칭된 값을 바인딩할 수 있다.

```rust
match pair {
    p @ (0, 0) => println!("origin: {:?}", p),
    p @ (0, y) => println!("x axis: {:?}", p),
    p @ (x, 0) => println!("y axis: {:?}", p),
    p @ (x, y) => println!("other: {:?}", p),
}
```
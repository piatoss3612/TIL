# Chapter09 스트럭트

## 이름 있는 필드로 된 스트럭트

```rust
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize)
}
```

- 스트럭트를 포함한 모든 타입의 이름을 지을 때는 캐멀 케이스를 사용한다.

```rust
// 스트럭트 표현식
let width = 1024;
let height = 576;
let image = GrayscaleMap {
    pixels: vec![0; width * height],
    size: (width, height)
};

// 축약 표현식
let pixels = vec![0; width * height];
let size = (width, height);
let image = GrayscaleMap { pixels, size };
```

- 스트럭트의 필드에 접근하기 위해서는 `.`을 사용한다.
- 다른 모든 아이템과 마찬가지로 스트럭트는 비공개이며 필드도 비공개이다. 따라서 외부에서 접근할 수 없다.
- 스트럭트와 필드를 공개하려면 `pub` 키워드를 사용한다.

```rust
pub struct GrayscaleMap {
    pub pixels: Vec<u8>,
    pub size: (usize, usize)
}
```

- 이름 있는 필드로 된 스트럭트 값을 만들 때 같은 타입의 다른 스트럭트를 써서 생략된 필드를 채울 수 있다.
- 이 때 Copy 타입의 필드는 복사되고, 그렇지 않은 타입의 필드는 이동된다.

```rust
struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Copy, Clone)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b // 나머지 필드는 b에서 가져온다.
    };

    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1 // 나머지 필드는 broom1에서 가져온다.
    };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

#[test]
fn test_chop() {
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };

    let (hokey1, hokey2) = chop(hokey);

    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.health, 100);
    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.health, 100);
}
```

## 튜플형 스트럭트

- 필드의 이름이 없는 스트럭트를 튜플형 스트럭트라고 한다.
- 튜플형 스트럭트에 담긴 값은 요소라고 부르며, 각 요소에 접근하는 방법은 튜플과 같다.

```rust
struct Bounds(usize, usize);

let image_bounds = Bounds(1024, 768);
assert_eq!(image_bounds.0 * image_bounds.1, 786432);
```

- 튜플형 스트럭트의 필드는 비공개이며, 공개하려면 `pub` 키워드를 사용한다.

```rust
pub struct Bounds(pub usize, pub usize);
```

- 튜플형 스트럭트를 생성하는 표현식은 함수 호출과 같다.

> 튜플형 스트럭트는 패턴 매칭으로 요소를 찾을 때 유용하고, 이름 있는 필드로 된 스트럭트는 가독성이 좋다.

- 튜플형 스트럭트는 뉴타입(newtype) 패턴을 구현할 때 유용하다.
- 뉴타입은 하나의 구성 요소만을 가진 것으로, 엄격한 타입 체크를 위해 사용한다.

```rust
struct Inches(i32);
```

## 유닛형 스트럭트

- 필드가 없는 스트럭트를 유닛형 스트럭트라고 한다.
- 이는 유닛형 타입 `()`와 마찬가지로 메모리를 차지하지 않는다.
- 유닛형 스트럭트는 트레이트와 같이 쓸 때 유용하다. (11장에서 자세히 다룬다.)

## 스트럭트 레이아웃

```rust
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize)
}
```

- 러스트는 스트럭트의 필드나 요소가 메모리에 어떤 순서로 배치되는지에 대해 아무것도 규정하지 않는다.
- 한 가지 확실한 것은 러스트가 필드의 값을 스트럭트의 메모리 블록 안에 직접 저장한다는 것이다. (pixels 벡터가 소유한 힙 할당 버퍼만 자체적으로 저장한다.)
- 이는 각 필드의 값을 각자의 메모리 블록에 저장하는 java, python, c++과 같은 언어와 다르다.
- #[repr(C)] 어노테이션을 사용하면 c나 c++과 호환되는 레이아웃을 강제할 수 있다. (23장에서 자세히 다룬다.)

## impl로 메서드 정의하기

- `impl` 블록은 fn 정의체의 집합으로, 각 정의는 스트럭트의 메서드가 된다.
- `impl` 블록 안에 정의된 함수는 특정 타입과 연관되어 있다고 하여 `연관 함수`라고 부른다. 반대로 `연관 함수`가 아닌 함수는 `자유 함수`라고 부른다.
- `연관 함수`는 `자유 함수`와 달리 `self` 파라미터를 갖는다. 이는 메서드를 호출할 때 메서드가 속한 스트럭트의 인스턴스를 암시적으로 전달받는다는 것을 의미한다.
- `self`, `&self`, `&mut self`는 각각 스트럭트의 인스턴스, 스트럭트의 불변 참조, 스트럭트의 가변 참조를 의미한다.

### self를 Box, Rc, Arc로 넘기기

```rust
let mut bq = Box::new(Queue::new());

// Queue::push()는 &mut self를 받지만 bq는 Box<Queue> 타입이다.
// 러스트는 Box로부터 &mut self를 빌려오므로 Queue::push()를 호출할 수 있다.
bq.push('0');
```

### 타입 연관 함수

- `타입 연관 함수`는 `self` 파라미터를 갖지 않는다.
- 주로 생성자로 사용된다.

```rust
impl Queue {
    pub fn new() -> Queue {
        Queue { older: Vec::new(), younger: Vec::new() }
    }
}
```

## 연관 상수

- 타입 그 자체와 연관된 상수를 정의할 수 있다.

```rust
impl Queue {
    const INITIAL_SIZE: usize = 10;
}
```

- 연관 상수는 다음과 같이 직접 호출할 수 있다.

```rust
let n = Queue::INITIAL_SIZE;
```

- 연관 상수의 타입은 자신이 연관된 타입과 꼭 같을 필요는 없다.
- 이를 이용해 타입에 ID나 이름을 부여할 수 있다.

```rust
impl Queue {
    const INITIAL_SIZE: usize = 10;
    const ID: u64 = 0xDEADBEEF;
}
```

## 제네릭 스트럭트

- 제네릭 스트럭트는 템플릿처럼 동작하여 원하는 타입을 받아들일 수 있다.

```rust
struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>
}
```

- `<T>`의 `T`는 `타입 매개변수로 임의의 요소 타입 T를 받아들인다는 것을 의미한다.
- 예를 들어 `Queue<String>`은 `older`와 `younger` 필드가 `Vec<String>` 타입을 갖는다.
- 타입 매개변수는 `impl` 블록 안에서도 사용할 수 있다.

```rust
impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue { older: Vec::new(), younger: Vec::new() }
    }
    fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}
```

- 연관 함수 호출의 경우 `::<>`(터보피시) 표기법을 써서 타입 매개변수를 명시할 수 있다.

```rust
let mut q = Queue::<char>::new();
```

## 수명 매개변수를 갖는 제네릭 스트럭트

- 스트럭트 타입이 레퍼런스를 갖는 경우 수명 매개변수를 사용해야 한다.

```rust
struct Extreama<'elt> {
    greatest: &'elt i32,
    least: &'elt i32
}
```

## 상수 매개변수를 갖는 제네릭 스트럭트

```rust
struct Polynomial<const N: usize> {
    coefficients: [f64; N]
}
```

## 스트럭트 타입에 공통 트레이트 구현하기

- 기본적으로 새롭게 정의된 스트럭트는 아무런 트레이트도 구현하지 않는다.
- 따라서 복사, 비교, 해시, 디버깅 등의 기능을 사용할 수 없다.
- 이를 해결하기 위해 `derive` 어노테이션을 사용하여 사전 정의된 트레이트를 구현할 수 있다.

```rust
#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64
}
```

- 스트럭트에 트레이트가 자동으로 구현될 수 있으려면, 스트럭트의 모든 필드가 해당 트레이트를 구현하고 있어야 한다.

## 내부 가변성

- `내부 가변성`: 스트럭트의 필드가 불변 참조를 갖더라도 필드의 값을 변경할 수 있다는 것을 의미한다.
- 이를 위해 `Cell`과 `RefCell` 타입을 사용한다.
- `Cell<T>`는 T타입의 비공개 값 하나만을 갖는 스트럭트다. Cell이 특별한 이유는 Cell 자체에 대한 mut 접근 권한이 없더라도 내부 값을 변경할 수 있기 때문이다.

* `Cell::new(value)`: 새로운 Cell 인스턴스를 생성한다.
* `cell.set(value)`: Cell 인스턴스의 값을 변경한다. 이 메서드는 self를 mut가 아닌 레퍼런스로 받는다.
* `cell.get()`: Cell 인스턴스의 값을 가져온다.

- Cell의 get 메서드가 값의 복사본을 반환하기 때문에 Copy 트레이트가 구현되어 있지 않은 타입에 대해서는 사용할 수 없다.
- 이러한 경우 RefCell을 사용한다.
- `RefCell<T>`는 자신이 가진 T값의 레퍼런스를 빌려오는 것을 허용한다.

* `RefCell::new(value)`: 새로운 RefCell 인스턴스를 생성한다.
* `refcell.borrow()`: RefCell 인스턴스의 값을 빌려온다. 값이 이미 변경할 수 있도록 차용되어 있으면 런타임에 패닉이 발생한다.
* `refcell.borrow_mut()`: RefCell 인스턴스의 값을 가변으로 빌려온다.
* `refcell.try_borrow()`, `refcell.try_borrow_mut()`: borrow, borrow_mut과 같지만 패닉 대신 Result를 반환한다.

```rust
use std::cell::RefCell;

let ref_cell = RefCell::<String>::new("hello".to_string());

let r = ref_cell.borrow();
let count = r.len();
assert_eq!(count, 5);

let mut w = ref_cell.borrow_mut();
w.push_str(" world");
```

- 상단의 코드는 mut 레퍼런스가 독점적이라는 규칙을 깨버리지만, 컴파일러는 이를 감지하지 못한다.
- RefCell은 실행 시점에 레퍼런스 규칙을 검사하기 때문에 런타임에 패닉이 발생한다.

> 셀과 셀을 포함하는 모든 타입은 스레드 안전하지 않다. 스레드 안전성을 갖는 내부 가변성은 19장에서 `Mutex`를 다룰 때 자세히 다룬다.
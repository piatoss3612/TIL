# Chapter08 크레이트와 모듈

## 크레이트

러스트 프로그램의 구성 요소. 하나의 완전하고 응집력있는 단위.

추이적인 의존성(transitive dependency) : 크레이트가 다른 크레이트에 의존하고, 그 크레이트가 또 다른 크레이트에 의존하는 것.
의존성 그래프(dependency graph) : 크레이트 간의 의존성 관계의 모음.

### 에디션

- 에디션은 러스트의 버전을 지정하여 호환성을 유지하면서 언어를 발전시키는 방법이다.
- 기존의 언어 체계가 변경되는 경우, 새로운 에디션을 발표하여 기존의 코드와 호환성을 유지하면서 새로운 언어 체계를 도입하기 위해 사용한다. (ex. async/await 키워드의 추가)
- `Cargo.toml` 파일의 `[package]` 섹션에 `edition` 키를 추가하여 에디션을 지정할 수 있다.

```toml
[package]
edition = "2021"
```

- 새 코드는 가능하면 최신 에디션을 사용하여 작성하는 것이 바람직하며, `cargo new` 명령으로 생성되는 프로젝트는 기본적으로 최신 에디션을 사용한다.

### 빌드 프로필

- 일반적으로는 기본 설정을 사용하면 되지만, 프로파일러를 사용하여 프로그램의 성능을 측정해야 하는 경우가 있다.
- 프로파일러를 사용하려면 최적화와 디버그 심볼이 모두 필요하다. 이는 다음과 같이 `Cargo.toml` 파일에 설정할 수 있다.

```toml
[profile.release]
debug = true
```

## 모듈

- 크레이트가 프로젝트 간의 코드 공유라면, 모듈은 프로젝트 내부의 코드 구성 단위이다.
- 모듈은 러스트의 네임스페이스로, 함수, 타입, 상수 등을 담는 컨테이너 역할을 한다.

```rust
mod foo {
    pub struct Bar {
        pub baz: i32,
    }

    pub fn bar() {}
}
```

- 모듈은 `아이템`의 집합체다. `아이템`은 이름이 있는 기능을 말한다. (함수, 구조체, 열거형, 트레잇 등)
- 모듈은 `pub` 키워드를 사용하여 외부에 공개할 수 있다. (기본적으로 비공개)

### 중첩된 모듈

- 모듈은 중첩될 수 있다.

```rust
mod foo {
    pub mod bar {
        pub fn baz() {}
    }
}
```

- 중첩된 모듈 안에 있는 아이템을 다른 크레이트에서 사용하려면, 아이템과 바깥쪽 모듈 모두 `pub` 키워드를 사용하여 공개해야 한다.

- 아이템을 부모 모듈에서만 사용하려면, pub(super) 키워드를 사용한다.

```rust
mod foo {
    pub mod bar {
        pub(in crate::foo) struct Baz { // foo 모듈 내부에서만 사용 가능
            pub qux: i32,
        }
    }

    use bar::Baz; // OK
}

use foo::bar::Baz; // Error: struct `Baz` is private
```

### 분리된 파일에 있는 모듈

- 모듈은 여러 파일에 나누어 작성할 수 있다.

```rust
mod foo;
```

```rust
// foo.rs
pub fn bar() {}
```

### 경로와 가져오기

- `::` 연산자를 사용하여 모듈의 아이템을 참조할 수 있다.

```rust
fn main() {
    let mut s1 = 5;
    let mut s2 = 3;

    if s1 > s2 {
        std::mem::swap(&mut s1, &mut s2);
    }

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

- 그러나 매번 `::` 연산자를 사용하는 것은 번거롭다. 이를 해결하기 위해 `use` 키워드를 사용한다.

```rust
use std::mem;

fn main() {
    let mut s1 = 5;
    let mut s2 = 3;

    if s1 > s2 {
        mem::swap(&mut s1, &mut s2);
    }

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

- `use` 키워드는 모듈의 아이템을 현재 스코프로 가져온다. (모듈의 아이템을 공개해야 한다.)
- `super` 키워드는 부모 모듈을 가리킨며, `crate` 키워드는 현재 모듈을 포함하는 크레이트를 가리킨다.
- `self` 키워드는 현재 모듈을 가리킨다.
- `as` 키워드를 사용하여 아이템의 이름을 변경할 수 있다.
- `절대 경로`는 외부 크레이트의 아이템을 참조할 때 사용한다. (ex. ::image::Pixels) `절대 경로`를 사용하여 동일한 이름의 아이템을 구분할 수 있다.

### 표준 프렐류드

- 각 모듈은 백지상태로 시작한다. 그러나 완전히 비어있는 것은 아니다. 러스트는 표준 프렐류드(standard prelude)를 제공한다.
- 표준 프렐류드에는 std 라이브러리나 자주 쓰이는 Vec, Result, Option 등의 타입이 포함되어 있다.

### use 선언을 public으로 만들기

- use 선언은 pub 키워드를 사용하여 외부에 공개할 수 있다.

### 스트럭트 필드를 pub으로 만들기

- 모듈 바깥에서 스트럭트의 필드를 직접 접근하려면, 필드를 pub으로 만들어야 한다.

```rust
mod foo {
    pub struct Bar {
        pub baz: i32,
    }
}
```

### 스태틱과 상수

- 상수는 `const` 키워드를 사용하여 선언한다. 상수는 반드시 타입을 명시해야 한다.

```rust
const MAX_POINTS: u32 = 100_000;
```

- 스태틱은 `static` 키워드를 사용하여 선언한다. 상수와 마찬가지로 타입을 명시해야 한다. 스태틱은 런타임에 변경 가능하다.
- 그러나 `static`은 배타적 접근이 보장되지 않아 스레드 간에 안전하지 않다. 따라서 안전한 코드에서 `static`에 `mut` 키워드를 사용해서는 안된다.

## 프로그램을 라이브러리로 바꾸기

```toml
[package]
name = "fern-sim"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

1. `src/main.rs` 파일을 `src/lib.rs`로 변경한다.
2. `src/lib.rs` 파일에 `pub` 키워드를 사용하여 라이브러리로 공개할 아이템을 지정한다.

- `Cargo.toml` 파일은 전혀 수정할 필요가 없다. `cargo build`는 기본적으로 파일을 보고 무엇을 빌드할지 결정한다. (src/main.rs가 있으면 바이너리, src/lib.rs가 있으면 라이브러리)

## src/bin 디렉토리

- 프로그램을 라이브러리와 같은 크레이트 안에 넣을 경우, `src/bin` 디렉토리에 실행 파일을 넣을 수 있다.

1. 아래 코드를 `src/bin/efern.rs` 파일로 저장한다.

```rust
use fern_sim::{run_simulation, Fern};

fn main() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.001,
    };

    run_simulation(&mut fern, 1000);
    println!("final fern size: {}", fern.size);
}
```

2. `cargo build` 및 `cargo run` 명령을 실행해본다.

```bash
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
```

```bash
$ cargo run 또는 cargo run --bin efern
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/efern`
final fern size: 2.7169239322355985
```

- 이처럼 라이브러리와 실행 파일을 같은 크레이트에 넣을 수 있다.

## 어트리뷰트

- 어트리뷰트는 컴파일러에게 추가적인 정보를 제공하는 방법이다.
- 어트리뷰트는 `#` 기호로 시작한다.
- 예를 들어, 러스트가 캐멀 케이스가 아닌 타입 이름을 경고하지 않도록 하려면 다음과 같이 어트리뷰트를 사용한다.

```rust
#[allow(non_camel_case_types)]
pub struct my_type {
    // ...
}
```

- `#[cfg]` 어트리뷰트를 사용하여 조건부 컴파일을 할 수 있다.
- `#[inline]` 어트리뷰트를 사용하여 인라인 함수를 만들 수 있다.
- 일부 어트리뷰트는 전체 크레이트에 적용되며, `#![cfg]`와 같이 `!` 기호를 사용한다.

## 테스트와 문서화

- `cargo test`는 프로젝트 안의 모든 테스트를 실행한다.
- 특정 테스트만 실행하려면 `cargo test <test_name>` 명령을 사용한다.
- 테스트는 주로 `assert!`, `assert_eq!` 매크로를 사용하여 작성한다.
    - `assert!(expr)`은 `expr`이 참이면 통과하고, 거짓이면 실패한다.
    - `assert_eq!(left, right)`은 `left`와 `right`가 같으면 통과하고, 다르면 실패한다.
- `assert!`와 `assert_eq!`은 일반 코드에서도 불변성을 검증하는 데 사용할 수 있지만, 릴리즈 빌드에도 포함이 되므로, 성능에 영향을 줄 수 있다. 따라서 `debug_assert!`와 `debug_assert_eq!`를 사용하여 릴리즈 빌드에는 포함되지 않도록 하는 것이 좋다.
- 오류 상황을 테스트하려면 해당 테스트에 `should_panic` 어트리뷰트를 추가한다. (컴파일러가 패닉을 무시하도록 allow(unconditional_panic) 어트리뷰트를 추가해야 한다.)

```rust
#[test]
#[allow(unconditional_panic, unused_must_use)]
#[should_panic(expected = "divide by zero")]
fn test_divide_by_zero() {
    1 / 0;
}
```

- 테스트는 `Result<(), E>` 타입을 반환할 수 있다.

```rust
use std::num::ParseIntError;

#[test]
fn main() -> Result<(), ParseIntError> {
    i32::from_str_radix("1024", 10)?;
    Ok(())
}
```

- 테스트 모듈은 `#[cfg(test)]` 어트리뷰트를 사용하여 테스트 빌드에만 포함되도록 할 수 있다.

```rust
#[cfg(test)]
mod tests {
    fn roughly_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn trig_works() {
        use std::f64::consts::PI;
        assert!(roughly_equal(PI.sin(), 0.0));
    }
}
```

- 러스트는 멀티 스레드 테스트를 지원한다. 이를 비활성화하려면 `cargo test -- --test-threads=1` 명령을 사용한다.

### 통합 테스트

- `src` 디렉토리와 나란히 존재하는 `tests` 디렉토리에 통합 테스트를 작성할 수 있다.
- `cargo test`를 실행하면 cargo는 각 통합 테스트를 개별적으로 컴파일한 다음, 라이브러리와 러스트 테스트 도구를 링크하여 독립된 크레이트로 만든다.

### 문서화

- 러스트는 문서화를 위한 도구를 제공한다. `cargo doc` 명령을 사용하여 문서를 생성할 수 있다.

```bash
$ cargo doc --no-deps --open
```

- `--no-deps` 옵션은 의존성 라이브러리의 문서를 생성하지 않도록 한다.
- `--open` 옵션은 문서를 생성한 후 웹 브라우저로 열어준다.

- 러스트는 `///`로 시작하는 주석을 문서화 주석(documentation comment)으로 취급한다.
- 마찬가지로 `//!`로 시작하는 주석은 모듈이나 크레이트의 문서화 주석으로 취급한다.
- 러스트 주석은 마크다운 문법을 사용할 수 있다. 그 중에서도 특이한 것은 `leaves::Leaf`와 같이 러스트 아이템 경로를 사용할 수 있다는 점이다.
- 또한 검색을 위해 `#[doc(keyword = "foo")]`와 같이 별칭을 지정할 수 있다.
- 외부 파일의 내용을 문서화 주석에 포함시키려면 `include_str!` 매크로를 사용한다. (ex. #![doc = include_str!("../README.md")])
- 텍스트 중간에 코드를 삽입하려면 ` ```rust`와 같이 코드 블록을 사용한다.

### 독 테스트

- 러스트는 문서화 주석에서 코드를 가져와 테스트를 만들 수 있다. 이를 독 테스트(doc test)라고 한다.

```rust
use std::ops::Range;

/// 두 범위가 겹치면 true를 반환한다.
///
///     assert_eq!(fern_sim::ranges::overlap(0..7, 3..10), true);
///     assert_eq!(fern_sim::ranges::overlap(1..5, 101..105), false);
///
/// 두 범위 중 하나라도 비어 있으면 겹치지 않는다고 판단한다.
///
///     assert_eq!(fern_sim::ranges::overlap(0..0, 0..10), false);
///
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end && r1.start < r2.end && r2.start < r1.end
}
```

```bash
$ cargo test --doc
   Compiling fern-sim v0.1.0 (/home/piatoss/TIL/rust/programming-rust/chapter08/example)
    Finished test [unoptimized + debuginfo] target(s) in 0.04s
   Doc-tests fern-sim

running 2 tests
test src/ranges.rs - ranges::overlap (line 10) ... ok
test src/ranges.rs - ranges::overlap (line 5) ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s
```

- 코드 블록의 첫 줄에 `#` 기호를 추가하면 해당 코드 블록은 컴파일되지 않는다.
- `no_run` 어노테이션을 사용하면 코드 블록은 컴파일되지만 실행되지는 않는다.
- `ignore` 어노테이션을 사용하면 해당 테스트는 무시된다.

## 의존성 지정하기

* 바로 버전을 지정하는 방법

```toml
[dependencies]
image = "0.6.1"
```

* `crate.io`에 게시되지 않은 크레이트를 사용하는 법 (git 저장소 - `rev`, `branch`, `tag` 지정 가능)

```toml
[dependencies]
image = { git = "", rev = "" }
```

* 디렉토리에 있는 크레이트를 사용하는 법

```toml
[dependencies]
image = { path = "vendor/image" }
```

### 버전

- 호환성 규칙은 **유의적 버전 관리(SemVer)**에 따른다.
    - 0.0 버전은 초기 개발 단계를 의미하며 다른 어떤 버전과도 호환되지 않는다.
    - 0.x 버전은 0.x 시리즈의 모든 버전과 호환되지만, 0.y 버전과는 호환되지 않는다.
    - 1.0 버전부터는 주 버전이 새로 나올 경우에만 호환성이 깨진다. 따라서 버전 2.x를 요구하는 크레이트는 1.x와 호환되지 않는다.

### Cargo.lock

- `Cargo.lock` 파일은 의존성 그래프를 고정시킨다. 즉, `Cargo.toml` 파일에 버전을 명시하지 않아도 `Cargo.lock` 파일에 명시된 버전을 사용한다.
- `Cargo.lock` 파일은 `Cargo.toml` 파일에 직접 버전을 변경하거나, `cargo update` 명령을 사용하여 갱신할 수 있다.

## crates.io에 크레이트 게시하기

* `cargo package` 명령을 사용하여 크레이트를 패키지로 만들 수 있다.
    - `cargo package --list` 명령을 사용하여 패키지에 포함된 파일을 확인할 수 있다.
    - `Cargo.toml` 파일에 `[package]` 섹션을 추가하여 패키지의 메타데이터를 지정해야 한다.

* 패키지를 게시하려면 `cargo login` 명령을 사용하여 crates.io에 로그인해야 한다.
* `cargo publish` 명령을 사용하여 패키지를 게시할 수 있다.

## 워크스페이스

- 워크스페이스는 여러 크레이트를 하나의 프로젝트로 묶는 방법이다.
- 워크스페이스는 `Cargo.toml` 파일에 `[workspace]` 섹션을 추가하여 지정한다.

```toml
[workspace]
members = [
    "fern-sim",
    "fern-sim-cli",
]
```

- 이렇게 하면 각 크레이트에 `target` 디렉토리가 생기지 않고, 워크스페이스의 `target` 디렉토리에 생성된다.
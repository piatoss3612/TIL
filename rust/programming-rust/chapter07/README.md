# Chapter07 오류 처리

## 패닉

- 프로그램 자체에 있는 버그로 인해 더 이상 진행할 수 없을 때 발생하는 오류
    - 배열의 인덱스 범위를 벗어난 접근
    - 정수를 0으로 나누는 연산
    - Err이 되어버린 Result에 대고 .expect()를 호출하는 행위
    - 단언문 실패
    > 이 문제들의 공통점은 프로그래머의 실수로 인해 발생한다는 것이다.

- 패닉이 발생하면 스택을 해제하거나 프로세스를 중단할 수 있다.

### 해제

```rust
fn main() {
    pirate_share(1, 0);
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}
```
```bash
thread 'main' panicked at 'attempt to divide by zero', src/main.rs:7:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- `RUN_BACKTRACE=1` 환경 변수를 설정하면 스택 추적을 볼 수 있다.

```bash
stack backtrace:
   0: rust_begin_unwind
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/std/src/panicking.rs:593:5
   1: core::panicking::panic_fmt
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/panicking.rs:67:14
   2: core::panicking::panic
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/panicking.rs:117:5
   3: example::pirate_share
             at ./src/main.rs:7:5
   4: example::main
             at ./src/main.rs:2:5
   5: core::ops::function::FnOnce::call_once
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

- rust의 패닉은 여타 언어의 패닉과 달리 바로 프로세스를 종료시키지 않고 정해진 동작(변수 해제 등)을 수행한 후 종료시킨다.

- 패닉은 스레드별로 발생하기 때문에 한 스레드가 패닉에 빠져도 다른 스레드는 계속 동작할 수 있다.
- `std::panic::catch_unwind`를 사용하면 패닉을 잡아낼 수 있다.

### 중단

- 만일 첫 번째 패닉을 정리하는 중간에 두 번째 패닉이 발생하면 러스트는 스택 해제를 시도하지 않고 프로세스를 중단시킨다.
- 만일 `-C panic=abort` 플래그를 사용하면 첫 번째 패닉이 발생했을 때 프로세스를 바로 종료시킨다.

---

## Result

- `Result<T, E>`는 `Ok(T)`와 `Err(E)` 두 가지의 값을 가질 수 있다.
- Result 타입은 실패 가능성을 암시적으로 표현하는 타입이다.
- 러스트는 Result 타입에 대해 모종의 오류 처리 코드 작성을 요구한다.

### 오류 잡기

1. match 표현식
2. is_ok(), is_err() 메서드: 결과가 Ok인지 Err인지 확인
3. ok(), err() 메서드: 결과가 Ok이면 Option<T>를 반환하고 Err이면 Option<E>를 반환
4. unwrap_or(fallback) 메서드: 결과가 Ok이면 Ok의 값을 반환하고 Err이면 fallback을 반환하며, 오류는 버린다.
5. unwrap_or_else(fallback_fn) 메서드: 결과가 Ok이면 Ok의 값을 반환하고 Err이면 fallback_fn의 반환값을 반환하며, 오류는 버린다.
6. unwrap() 메서드: 결과가 Ok이면 Ok의 값을 반환하고 Err이면 패닉을 발생시킨다.
7. expect(msg) 메서드: 결과가 Ok이면 Ok의 값을 반환하고 Err이면 msg를 출력하며 패닉을 발생시킨다.
8. as_ref(), as_mut() 메서드: Result<T, E>를 Result<&T, &E> 또는 Result<&mut T, &mut E>로 변환한다.

### Result 타입 별칭

- `type Result<T> = Result<T, std::io::Error>`와 같이 별칭을 지정할 수 있다. 이렇게 하면 `Result<T>`를 사용할 때마다 `std::io::Error`를 명시할 필요가 없다.

```rust
pub type Result<T> = Result<T, std::io::Error>;
```

### 오류 출력하기

- println!(), writeln!(): 오류를 출력
- err.to_string(): 오류를 문자열로 변환
- err.source(): 오류의 원인이 있을 경우 원인을 반환

```rust
use std::error::Error;
use std::io::{Write, stderr};

// 오류에 관한 모든 정보를 출력한다.
fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}
```

### 오류 전파하기

- 오류를 매번 처리하기 보다는 호출부에서 한 번에 처리하는 것이 좋다.
- `?` 연산자를 사용하면 오류를 호출부로 전파할 수 있다.

```rust
let weather = get_weather(location)?;
```

- 함수가 성공 결과를 반환하면 T 타입의 값을 반환하고, 오류가 발생하면 호출부에 오류를 전파한다.
- `?` 연산자는 Result 타입에서만 사용할 수 있다.

### 여러 오류 타입 다루기

- 표준 라이브러리의 모든 오류 타입은 `Box<dyn std::error::Error + Send + Sync + 'static>`을 구현한다. 이를 통해 여러 오류 타입을 하나의 Result 타입으로 묶을 수 있다.

```rust
type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;
```

- `GenericResult`를 반환하는 함수를 호출할 때 특정 유형으로 된 오류만 처리하고 나머지는 그냥 전파하길 원한다면 제너릭 메서드 `error.downcast_ref::<E>()`를 사용하면 된다.

```rust
loop {
    match compile_project() {
        Ok(()) => return Ok(()),
        Err(err) => {
            if let Some(err) = err.downcast_ref::<ProjectError>() {
                match err {
                    ProjectError::Io(err) => {
                        if err.kind() == ErrorKind::NotFound {
                            return Err(err.into());
                        }
                    }
                }
            }
            return Err(err);
        }
    }
}
```

### '발생할 리 없는' 오류 다루기

- unwrap(), expect() 메서드는 오류가 발생할 리 없는 경우에만 사용해야 한다. 그렇지 않으면 패닉이 발생할 수 있다.


### 오류 무시하기

- `let _ = ...`와 같이 `_`를 사용하면 오류를 무시할 수 있다.

```rust
let _ = writeln!(stderr(), "error: {}", err);
```

### main()에서 오류 처리하기

- main 함수의 반환 타입 시그니처를 `Result<T, E>`로 지정하면 main 함수에서도 오류를 처리할 수 있다.

```rust
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // ...
}
```

### 사용자 정의 오류 타입 선언하기

```rust
#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl std::error::Error for JsonError {
    // ...
}
```

- thiserror 크레이트를 사용하면 더 쉽게 사용자 정의 오류 타입을 선언할 수 있다.

```rust
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{message:} ({line:}, {column:})")]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}
```
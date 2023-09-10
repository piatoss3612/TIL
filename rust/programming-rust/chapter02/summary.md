# chapter02 러스트 둘러보기

## rsstup과 cargo

### rust 설치

- https://rustup.rs/ 또는 https://www.rust-lang.org/tools/install 에서 설치

### 설치 확인

```bash
$ cargo --version
cargo 1.72.0 (103a7ff2e 2023-08-15)
$ rustc --version
rustc 1.72.0 (5680fa18f 2023-08-23)
$ rustdoc --version
rustdoc 1.72.0 (5680fa18f 2023-08-23)
```

### 각 명령어의 역할

1. cargo: 컴파일 및 패키지 관리
2. rustc: 러스트 컴파일러 (일반적으로 cargo를 통해 사용)
3. rustdoc: 러스트 문서화 도구 (일반적으로 cargo를 통해 사용)

### rust 패키지 생성

```bash
$ cargo new hello
     Created binary (application) `hello` package
```
```bash
$ cd hello
$ ls -la
total 24
drwxr-xr-x  4 piatoss piatoss 4096 Sep  9 21:10 .
drwxr-x--- 37 piatoss piatoss 4096 Sep  9 21:10 ..
drwxr-xr-x  6 piatoss piatoss 4096 Sep  9 21:10 .git
-rw-r--r--  1 piatoss piatoss    8 Sep  9 21:10 .gitignore
-rw-r--r--  1 piatoss piatoss  174 Sep  9 21:10 Cargo.toml
drwxr-xr-x  2 piatoss piatoss 4096 Sep  9 21:10 src
```

- `Cargo.toml`: 패키지의 메타데이터를 담고 있는 파일
- `.git` 및 `.gitignore`: git 저장소 관련 파일
- `src`: 소스코드가 담겨있는 디렉토리

### rust 패키지 실행

- rustc로 패키지를 컴파일한 다음, 실행 파일을 실행

```bash
$ cargo run
   Compiling hello v0.1.0 (/home/piatoss/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/hello`
Hello, world!
```

- 실행 파일은 `target` 디렉토리에 생성됨

```bash
$ ls -l target/debug/
total 4512
drwxr-xr-x 2 piatoss piatoss    4096 Sep  9 21:12 build
drwxr-xr-x 2 piatoss piatoss    4096 Sep  9 21:12 deps
drwxr-xr-x 2 piatoss piatoss    4096 Sep  9 21:12 examples
-rwxr-xr-x 2 piatoss piatoss 4599640 Sep  9 21:12 hello
-rw-r--r-- 1 piatoss piatoss      72 Sep  9 21:12 hello.d
drwxr-xr-x 3 piatoss piatoss    4096 Sep  9 21:12 incremental
```

---

## 러스트 함수

- rust의 문법은 의도적으로 다른 언어들과 비슷하게 만들어졌다.

```rust
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
```
- `fn`: 함수 선언
- `gcd`: 함수 이름
- `mut`: 가변성
- `->`: 반환 타입
- `assert!`: 매크로 호출
- `while`, `if`, `let`: 키워드
- `n`, `m`, `t`: 변수
- `u64`: 타입
- `;`: 문장 종료
- 마지막 문장의 `n`은 반환값

---

## 단위 테스트 작성해 돌려보기

```rust
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
```

- `#[test]`: 테스트 함수임을 나타내는 어트리뷰트
- `assert_eq!`: 매크로 호출

```bash
$ cargo test
   Compiling hello v0.1.0 (/home/piatoss/TIL/rust/programming-rust/chapter02/hello)
    Finished test [unoptimized + debuginfo] target(s) in 0.77s
     Running unittests src/main.rs (target/debug/deps/hello-d97a9b16105badb2)

running 1 test
test test_gcd ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

## 명령줄 인수 다루기

```rust
use std::env;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
```

- `use`: 모듈 or 트레이트 가져오기
- `Vec::new()`: 빈 벡터 생성
- `env::args()`: 명령줄 인수를 담은 이터레이터 반환
- `skip(1)`: 첫 번째 인수는 프로그램 이름이므로 건너뜀
- `u64::from_str(&arg)`: 문자열을 숫자로 변환
- `expect("error parsing argument")`: 변환 실패 시 에러 메시지 출력
- `eprintln!`: 표준 에러 출력
- `std::process::exit(1)`: 프로그램 종료
- `&numbers[1..]`: 벡터의 두 번째 이후 요소들을 참조
- `*m`: 참조된 값을 복사

```bash
$ cargo run 5 10 15 20
   Compiling hello v0.1.0 (/home/piatoss/TIL/rust/programming-rust/chapter02/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/hello 5 10 15 20`
The greatest common divisor of [5, 10, 15, 20] is 5
```

---

## 웹 서비스 만들기

### 크레이트 불러오기

```toml
[package]
name = "actix-gcd"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
actix-web = "1.0.8"
serde = { version = "1.0", features = ["derive"] }
```

- dependencies에 필요한 크레이트를 추가
- actix-web: 웹 서버 프레임워크
- serde: 직렬화/역직렬화 라이브러리 (옵션 기능을 사용하기 위해 features 추가)

- `cargo build` 또는 `cargo run`을 실행하면 자동으로 크레이트를 다운로드 받음

### 웹 서버 구현

```rust
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Starting server on http://localhost:3000");

    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .await
        .expect("error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button type="submit">Compute GCD</button>
        </form>
        "#,
    )
}

use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response = format!(
        "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }

        m = m % n;
    }

    n
}
```

---

## 동시성

### mandelbrot 예제

- 전체 코드 설명은 무슨 소리를 하는지 하나도 모르겠으므로 나중에 다시 읽어보기로...

1. `cargo build`로 빌드한 뒤 실행 (비동시적 실행)

```bash
$ time ./target/debug/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20

real    0m56.200s
user    0m56.187s
sys     0m0.013s
```

2. `cargo build --release`로 빌드한 뒤 실행 (비동시적 실행)

```bash
$ time ./target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20

real    0m3.943s
user    0m3.933s
sys     0m0.011s
```

3. `cargo build --release`로 빌드한 뒤 실행 (동시적 실행)

```bash
$ time ./target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20

real    0m1.260s
user    0m4.241s
sys     0m0.000s
```

---
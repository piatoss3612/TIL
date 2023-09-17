# 1. Hello World

## Generate binary

```bash
$ rustc src/main.rs
```

## Run binary

```bash
$ ./main
Hello, world!
```

## Activity

- Add a new line with a second println! macro

```rust
fn main() {
    println!("Hello, world!");
    println!("I'm a Rustorance!");
}
```

```bash
$ rustc src/main.rs
$ ./main
Hello, world!
I'm a Rustorance!
```
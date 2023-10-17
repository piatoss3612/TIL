fn main() {}

#[test]
fn test_if_expr() {
    let x = 5;
    let y = if x == 5 { 10 } else { 15 };
    assert_eq!(y, 10);
}

#[test]
fn test_for_ownership_move() {
    let strings: Vec<String> = vec!["Hello".to_string(), "World".to_string()];
    // for s in strings { // 값이 이동됨
    //     println!("{}", s);
    // }

    // println!("{}", strings.len()); // 오류: 값이 이동됨

    for s in strings.clone() {
        // 복사본을 만들어서 사용
        println!("{}", s);
    }

    println!("{}", strings.len()); // 정상: 값이 복사됨

    for s in &strings {
        // 참조자를 사용
        println!("{}", s);
    }

    println!("{}", strings.len()); // 정상: 참조자를 사용했기 때문에 값이 이동되지 않음
}

#[test]
fn test_float_to_int() {
    let x = -1.9;
    let y = x as i32;
    println!("{}", y);
}

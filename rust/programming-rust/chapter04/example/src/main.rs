fn main() {
    print_hello_world();
    print_struct();
}

fn print_hello_world() {
    let mut s = String::from("hello"); // s는 String 값의 소유자
    s.push_str(", world!"); // String 값에 문자열을 추가
    println!("{}", s); // hello, world!
} // s가 스코프를 벗어나면 String 값이 드롭된다.

#[test]
fn test_box() {
    {
        let point = Box::new((0.625, 0.5)); // 튜플을 박스로 감싼다.
        let label = format!("{:?}", point); // 포맷 매크로를 사용한다.
        assert_eq!(label, "(0.625, 0.5)");
    } // point와 label이 스코프를 벗어나면 드롭된다.
}

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

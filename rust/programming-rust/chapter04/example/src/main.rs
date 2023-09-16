use std::vec;

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

#[test]
fn test_move() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()]; // s는 Vec<String> 값의 소유자
                                                                               // let t = s; // s의 소유권이 t로 이동한다.
                                                                               // let u = s; // s의 소유권이 이미 t로 이동했으므로 미초기화 상태인 s를 사용할 수 없다. 따라서 컴파일 에러가 발생한다.

    let t = s.clone(); // s의 복사본을 만든다.
    let u = s.clone(); // s의 복사본을 만든다.

    assert_eq!(s, t);
    assert_eq!(t, u);

    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string(); // 기존의 String 값은 드롭된다.

    assert_eq!(s, "Siddhartha");

    let mut s = "Govinda".to_string();
    let t = s; // s의 소유권이 t로 이동한다.
    s = "Siddhartha".to_string(); // 미초기화 상태인 s에 새로운 String 값이 할당된다.

    assert_eq!(t, "Govinda");
    assert_eq!(s, "Siddhartha");

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
    let first_name = composers[0].name.take();
    // assert_eq!(first_name, None);
}

#[test]
fn test_struct_copy() {
    #[derive(Copy, Clone)]
    struct Label {
        number: u32,
    }

    fn print(l: Label) {
        println!("STAMP: {}", l.number);
    }

    let l = Label { number: 3 };
    print(l);
    print(l);
}

#[test]
fn test_reference_count() {
    use std::rc::Rc;

    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!(
        "{} are quite chewy, almost bouncy, and largely flavorless",
        u
    );
}

use ::std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    // table의 레퍼런스를 가져오면서 table의 소유권을 가져오지 않음
    // HashMap의 공유된 레퍼런스를 반복 처리할 때는 각 요소에 대해서도 공유된 레퍼런스를 만들도록 정의되어 있음
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_, works) in table {
        works.sort();
    }
}

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    sort_works(&mut table);
    show(&table);
}

#[test]
fn test_reference() {
    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    }

    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation"); // 암묵적으로 역참조
    assert_eq!((*anime_ref).name, "Aria: The Animation"); // 명시적으로 역참조

    let mut v = vec![1973, 1968];
    v.sort();
    (&mut v).sort(); // 명시적으로 역참조

    let x = 10;
    let y = 20;
    let mut r = &x;

    if true {
        r = &y;
    }

    assert!(*r == 10 || *r == 20);

    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 1000, y: 729 };
    let r = &point;
    let rr = &r;
    let rrr = &rr;

    assert_eq!(rrr.y, 729);

    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(!std::ptr::eq(rx, ry));

    // assert!(rx == rrx); // 타입 불일치: &i32 != &&i32
    assert!(rx == *rrx); // 문제 없음: &i32 == &i32

    fn factorial(n: usize) -> usize {
        (1..n + 1).product()
    }
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);
}

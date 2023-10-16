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

static mut STASH: &i32 = &128;
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

#[test]
fn test_static_lifetime() {
    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);

    // let x = 10;
    // f(&x);
}

#[test]
fn test_static_field_lifetime() {
    struct S {
        r: &'static i32,
    }

    let s;
    {
        static X: i32 = 10;
        s = S { r: &X };
    }

    assert_eq!(*s.r, 10);

    struct S2<'a> {
        r: &'a i32,
    }

    struct D {
        s: S2<'static>,
    }

    struct D2<'a> {
        s: S2<'a>,
    }
}

#[test]
fn test_unique_lifetime() {
    // struct S<'a> {
    //     x: &'a i32,
    //     y: &'a i32,
    // }

    // let x = 10;
    // let r;
    // {
    //     let y = 20;
    //     {
    //         let s = S { x: &x, y: &y }; // s의 x와 y의 수명이 동일해야 하는데 다름
    //         r = s.x;
    //     }
    // }

    // println!("{}", r);

    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s: S<'_, '_> = S { x: &x, y: &y }; // s의 x와 y의 수명이 달라도 됨
            r = s.x;
        }
    }

    println!("{}", r);
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }

    fn find_by_prefix2<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}

#[test]
fn test_reference2() {
    let mut x = 10;
    let r1 = &x;
    let r2 = &x; // 공유된 레퍼런스는 여러 번 빌려와도 문제 없음

    // x += 10; // 오류: `x`는 공유된 레퍼런스가 존재하므로 변경할 수 없음
    // let m = &mut x; // 오류: `x`는 공유된 레퍼런스가 존재하므로 변경할 수 있는 레퍼런스를 만들 수 없음

    let mut y = 20;
    let m1 = &mut y;

    // let m2 = &mut y; // 오류: 변경할 수 있는 레퍼런스는 하나만 만들 수 있음
    // let z = y; // 오류: 변경할 수 있는 레퍼런스가 존재하므로 `y`의 소유권을 가져갈 수 없음

    let mut w = (107, 109);
    let r = &w;
    let r0 = r.0; // 공유된 레퍼런스를 통해 다시 공유된 레퍼런스를 만들 수 있음

    // let m1 = &mut r.1; // 오류: `w`는 공유된 레퍼런스가 존재하므로 변경할 수 있는 레퍼런스를 만들 수 없음

    let mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0; // 변경할 수 있는 레퍼런스를 통해 다시 변경할 수 있는 레퍼런스를 만들 수 있음

    *m0 = 137;
    let r1 = &m.1; // 변경할 수 있는 레퍼런스를 통해 공유된 레퍼런스를 만들 수 있음

    // v.1; // 오류: 금지되어 있는 다른 경로를 통한 접근
}

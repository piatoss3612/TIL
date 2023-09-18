# Chapter05 레퍼런스

레퍼런스는 자신이 가리키는 대상보다 절대로 더 오래 살아 있으면 안된다. 이를 강조하기 위해 레퍼런스를 만드는 걸 두고 값을 `빌려 온다`고 표현한다. 빌려온 것은 반드시 돌려줘야 한다.

## 값의 레퍼런스

```rust
use ::std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    // table의 소유권을 가져오면서 table의 값을 비움
    for (artist, works) in table {
        println!("works by {}:", artist);
        // works의 소유권을 가져오면서 works의 값을 비움
        for work in works {
            println!("  {}", work);
        }
        // works는 미초기화 상태가 됨
    }
    // table은 미초기화 상태가 됨
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
    
    ...

    show(table); // table의 소유권을 넘겨줌
}
```

그저 출력만 했을 뿐인데 `table`과 `works`의 소유권을 가져와서 비워버렸다. 이를 방지하기 위해 레퍼런스를 사용해야 한다.

레퍼런스에는 두 가지가 있다.

1. 공유된 레퍼런스(shared reference): `&T` 형태로 표현하며, `T`의 값을 빌려온다. 참조하는 대상을 읽을 수만 있고, 변경할 수는 없다. 레퍼런스를 여러 개 만들어도 된다. 공유된 레퍼런스는 Copy 타입이다.
2. 변경할 수 있는 레퍼런스(mutable reference): `&mut T` 형태로 표현하며, `T`의 값을 빌려온다. 참조하는 대상을 읽고 쓸 수 있다. 레퍼런스를 하나만 만들 수 있다. 변경할 수 있는 레퍼런스는 Copy 타입이 아니다.

공유된 레퍼런스와 변경할 수 있는 레퍼런스를 굳이 구분하는 이유는 컴파일 시점에 `멀티플 리더 또는 싱글 라이터` 규칙을 시행하기 위해서라고 볼 수 있다. 이 규칙은 빌려온 값의 소유자에게도 적용된다. 공유된 레퍼런스가 존재하는 동안에는 참조하는 대상을 변경할 수 없다. 마찬가지로 변경할 수 있는 레퍼런스가 존재하는 동안에는 참조하는 대상에 대한 독점적인 접근 권한을 가진다. 이처럼 공유와 변경을 구분하는 것은 러스트의 메모리 안전성을 지키는 데 가장 중요한 요소 중 하나이다.

앞의 예제를 레퍼런스를 사용해 수정해보자.

```rust
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

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    
    ...

    show(&table); // table의 레퍼런스를 넘겨줌
}
```

다음은 변경할 수 있는 레퍼런스를 사용한 예제이다.

```rust
fn sort_works(table: &mut Table) {
    for (_, works) in table {
        works.sort(); // works의 변경할 수 있는 레퍼런스를 만들어 works를 정렬
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
    
    ...

    sort_works(&mut table); // table의 변경할 수 있는 레퍼런스를 넘겨줌
}
```

---
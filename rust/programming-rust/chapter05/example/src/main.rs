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

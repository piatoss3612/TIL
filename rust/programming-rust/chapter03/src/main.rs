use std::vec;

fn main() {
    let a = true;
    if a {}
}

#[test]
fn test_as() {
    assert_eq!(10_i8 as i16, 10_i16);
    assert_eq!(2525_u16 as i16, 2525_i16);

    assert_eq!(-1_i16 as i32, -1_i32); // 음수를 보다 큰 타입으로 변환할 때는 빈 공간을 부호로 채운다.
    assert_eq!(65535_u16 as i32, 65535_i32); // 양수를 보다 큰 타입으로 변환할 때는 빈 공간을 0으로 채운다.

    // 보다 작은 타입으로 변환할 때는 값이 잘린다.
    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u16 as i32, 65535_i32);

    // 부호가 없는 타입 <-> 부호가 있는 타입
    assert_eq!(-1_i8 as u8, 255_u8); // -1은 11111111이므로 부호가 없는 타입으로 변환하면 255가 된다.
    assert_eq!(255_u8 as i8, -1_i8); // 255는 11111111이므로 부호가 있는 타입으로 변환하면 -1이 된다.
}

#[test]
fn test_calc() {
    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b101101_u8.count_ones(), 4);
}

#[test]
fn test_checked() {
    assert_eq!(10_u8.checked_add(20), Some(30)); // 더하기 연산의 결과가 타입의 범위를 넘지 않으면 Some(결과값)을 반환한다.
    assert_eq!(100_u8.checked_add(200), None); // 더하기 연산의 결과가 타입의 범위를 넘으면 None을 반환한다.
    assert_eq!((-128_i8).checked_div(-1), None); // i8 타입의 최소값을 -1로 나누면 타입의 범위를 넘어서므로 None을 반환한다.
}

#[test]
fn test_wrapping() {
    assert_eq!(100_u16.wrapping_mul(200), 20000); // 곱하기 연산의 결과가 타입의 범위를 넘지 않으면 결과값을 반환한다.
    assert_eq!(500_u16.wrapping_mul(500), 53392); // 곱하기 연산의 결과가 타입의 범위를 넘으므로 결괏값을 2^16으로 나눈 나머지를 반환한다.

    assert_eq!(500_i16.wrapping_mul(500), -12144); // 부호가 있는 타입을 대상으로 곱하기 연산을 수행하면 결과값이 음수로 순환할 수 있다.

    assert_eq!(5_i16.wrapping_shl(17), 10); // 16비트 크기의 정수에 대해 17비트만큼 왼쪽으로 시프트하면 1 비트만큼 왼쪽으로 시프트한 것과 같다.
}

#[test]
fn test_saturating() {
    assert_eq!(32760_i16.saturating_add(10), 32767); // 더하기 연산의 결과가 타입의 범위를 넘었으므로 타입의 최대값을 반환한다.
    assert_eq!((-32760_i16).saturating_sub(10), -32768); // 빼기 연산의 결과가 타입의 범위를 넘었으므로 타입의 최소값을 반환한다.
}

#[test]
fn test_overflowing() {
    assert_eq!(255_u8.overflowing_sub(2), (253, false)); // 빼기 연산의 결과가 타입의 범위를 넘지 않으면 (결과값, false)를 반환한다.
    assert_eq!(255_u8.overflowing_add(2), (1, true)); // 더하기 연산의 결과가 타입의 범위를 넘으면 (순환 결과값, true)를 반환한다.

    // 16비트 크기의 정수에 대해 17비트만큼 왼쪽으로 시프트하면 1 비트만큼 왼쪽으로 시프트한 것과 같으며
    // 이동 거리가 타입의 비트 수보다 크므로 true를 반환한다.
    assert_eq!(5_u16.overflowing_shl(17), (10, true));
}

#[test]
fn test_float_type() {
    assert!((-1. / f32::INFINITY).is_sign_negative()); // 부호가 있는 부동소수점 음수를 무한대로 나누면 부호가 있는 부동소수점 음수가 된다.
    assert_eq!(-f32::MIN, f32::MAX); // 부호가 있는 부동소수점 수의 최소값에 -1을 곱하면 최대값이 된다.

    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // 제곱근을 구한 후 다시 제곱하면 원래 값이 된다.
    assert_eq!((-1.01f64).floor(), -2.); // -1.01을 내림하면 -2.0이 된다.
}

#[test]
fn test_bool() {
    assert_eq!(true as i32, 1); // true를 i32 타입으로 변환하면 1이 된다.
    assert_eq!(false as i32, 0); // false를 i32 타입으로 변환하면 0이 된다.
}

#[test]
fn test_char() {
    assert_eq!('*' as i32, 42); // '*'를 i32 타입으로 변환하면 42가 된다.
    assert_eq!('ಠ' as u16, 0xca0); // 'ಠ'를 u16 타입으로 상위 비트가 잘려서 0xca0이 된다.
    assert_eq!('ಠ' as i8, -0x60); // 'ಠ'를 i8 타입으로 상위 비트가 잘려서 -0x60이 된다.

    assert_eq!('*'.is_alphabetic(), false); // '*'는 알파벳이 아니다.
    assert_eq!('β'.is_alphabetic(), true); // 'β'는 알파벳이다.
    assert_eq!('8'.to_digit(10), Some(8)); // '8'은 10진수로 8이다.
    assert_eq!('ಠ'.len_utf8(), 3); // 'ಠ'는 UTF-8로 3바이트이다.
    assert_eq!(char::from_digit(2, 10), Some('2')); // 10진수 2는 '2'이다.
}

#[test]
fn test_tuple() {
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
}

#[test]
fn test_array() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        let mut j = i * i;
        while j < 10000 {
            sieve[j] = false;
            j += i;
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}

#[test]
fn test_vector() {
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210); // 벡터의 모든 요소를 곱한 결과가 210이다.

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030); // 벡터의 모든 요소를 곱한 결과가 30030이다.

    let rows = 3;
    let cols = 5;
    let mut table = vec![vec![0; cols]; rows]; // 3행 5열의 2차원 벡터를 생성한다.
    table[2][3] = 42;
    assert_eq!(table[2][3], 42); // 2행 3열의 요소가 42이다.

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();

    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);

    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 4);

    let mut v = vec![10, 20, 30, 40, 50];
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);

    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);

    let mut v = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(v.pop(), Some("Glass Gem"));
    assert_eq!(v.pop(), Some("Snow Puff"));
    assert_eq!(v.pop(), None);
}

#[test]
fn test_string() {
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let poodles = "ಠ_ಠ";

    assert_eq!(oodles, "oodles");
    assert_eq!(poodles.len(), 7);
    assert_eq!(poodles.chars().count(), 3);

    let temp = format!("{}°{:02}′{:02}″N", 24, 5, 23);
    assert_eq!(temp, "24°05′23″N");

    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");

    assert!("ONE".to_lowercase() == "one");

    assert!("peanut".contains("nut"));
    assert_eq!("ಠ_ಠ".replace("ಠ", "■"), "■_■");
    assert_eq!("    clean\n".trim(), "clean");

    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }

    type Bytes = Vec<u8>;
    let b: Bytes = From::from("bors".as_bytes());

    assert_eq!(b, b"bors");
}

// fn build_vector() -> Vec<i16> {
//     // let mut v: Vec<i16> = Vec::<i16>::new(); // 반환값의 타입을 명시적으로 지정하여 i16 타입의 벡터를 반환한다.
//     let mut v = Vec::new(); // 반환값의 타입에 따라 타입을 추론하여 i16 타입의 벡터를 반환한다.
//     v.push(10);
//     v.push(20);
//     v
// }

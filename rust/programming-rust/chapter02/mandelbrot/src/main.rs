use num::Complex;

/// `c`가 망델브로 집합에 속하는지 아닌지를 판단하며, 결론 내리는 데 필요한 반복 횟수는
/// 최대 `limit`로 제한한다.
///
/// `c`가 망델브로 집합에 속하지 않으면 `Some(i)`를 반환한다. 여기서 `i`는 `c`가 원점을
/// 중심으로 반경이 2인 원을 벗어나는 데 걸린 반복 횟수이다. `c`가 망델브로 집합에 속하는 것으로
/// 판단되면(`c`가 limit 반복을 모두 마친 후에도 원 안에 있으면) `None`을 반환한다.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

use std::str::FromStr;

/// `s`를 좌표 쌍으로 파싱한다.
///
/// `s`는 정확히 <left><sep><right> 형태의 쌍으로 구성되어야 한다. <sep>는 구분자 문자이며,
/// 공백이 아니어야 한다. <left>와 <right>는 `T::from_str`로 파싱할 수 있어야 한다.
///
/// `s`가 올바른 형식이라면 `Some<(x, y)>`를 반환한다. 여기서 x와 y는 각각 <left>와 <right>에
/// 해당한다. `s`가 올바른 형식이 아니라면 `None`을 반환한다.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// 쉼표로 구분된 좌표 쌍을 파싱하여 `Complex<f64>`로 반환한다.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// 결과 이미지의 픽셀 좌표에 대응하는 복소평면 상의 점을 반환한다.
///
/// `bounds`는 픽셀 단위로 된 이미지의 가로 세로 크기이며, `pixel`은 이미지 내의 픽셀 좌표이다.
/// `upper_left`와 `lower_right`는 복소평면의 좌상단과 우하단을 나타낸다.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
        // 픽셀 좌표계는 왼쪽 위 모서리가 (0, 0)이고, 복소평면 좌표계는 왼쪽 아래 모서리가 (0, 0)이다.
        // 따라서 픽셀 좌표의 y값을 증가시키면 복소평면 좌표의 y값은 감소한다.
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}

/// 직사각형 모양의 망델브로 집합을 픽셀 버퍼에 렌더링한다.
///
/// `bounds` 인수는 한 바이트 안에 회색조로 된 픽셀 하나가 들어가는 `pixels` 버퍼의 폭과 높이를 갖는다.
/// `upper_left`와 `lower_right` 인수는 복소평면의 좌상단과 우하단을 나타낸다.
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;

/// `bounds` 크기의 `pixels` 버퍼를 `filename`으로 저장한다.
fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}

use ::std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    // render(&mut pixels, bounds, upper_left, lower_right);

    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
        .unwrap();
    }

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

extern crate iron;
#[macro_use]
extern crate mime;
extern crate router;
extern crate urlencoded;

extern crate crossbeam;
extern crate image;
extern crate num;

use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;

use image::png::PNGEncoder;
use image::ColorType;
use num::complex::Complex64;
use num::Complex;
use std::fs::File;
use std::io::Write;

// use std::io::Write;
use std::str::FromStr;

fn main() {
    // let mut numbers = Vec::new();

    // for arg in std::env::args().skip(1) {
    //     numbers.push(u64::from_str(&arg).expect("error parsing args"))
    // }

    // if numbers.len() == 0 {
    //     writeln!(std::io::stderr(), "Usage: gcd Number ...").unwrap();
    //     std::process::exit(1);
    // }

    // let mut d = numbers[0];
    // for m in numbers[1..].into_iter() {
    //     d = gcd(d, *m);
    // }

    // println!("The gcd of {:?} is {}", numbers, d);

    if 1 + 1 == 3 {
        start_web();
    }

    final_write_image();
}

fn final_write_image() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(std::io::stderr(), "Usage: ...").unwrap();
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing bounds");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper_left");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower_right");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    let threads = 8;
    if threads == 1 {
        render(&mut pixels, bounds, upper_left, lower_right);
    } else {
        let rows_per_band = bounds.1 / threads + 1;
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move || {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
    }

    write_image(&args[1], &pixels, bounds).expect("error writing file");
}

#[allow(dead_code)]
fn square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        x = x * x + c;
    }
}

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

fn escape_time(c: Complex64, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

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
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

fn parse_complex(s: &str) -> Option<Complex64> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("0.5,1.5"), Some(Complex { re: 0.5, im: 1.5 }));
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex64,
    lower_right: Complex64,
) -> Complex64 {
    let (w, h) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * w / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * h / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex64,
    lower_right: Complex64,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let p = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = match escape_time(p, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;

    Ok(())
}

// =============================== web =====================================

fn start_web() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");
    println!("Listening on http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    resp.set_mut(status::Ok);
    resp.set_mut(mime!(Text/Html; Charset=Utf8));
    resp.set_mut(
        r#"
hello
<b>world</b>!
<form action="/gcd" method="post">
<input type="text" name="n"/>
<input type="text" name="n"/>
<button type="submit">Submit</button>
</form>
"#,
    );

    Ok(resp)
}

fn post_gcd(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let form_data = match req.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            resp.set_mut(status::BadRequest);
            resp.set_mut(format!("error parsing data: {:?}\n", e));
            return Ok(resp);
        }
        Ok(map) => map,
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            resp.set_mut(status::BadRequest);
            resp.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(resp);
        }
        Some(nums) => nums,
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(e) => {
                resp.set_mut(status::BadRequest);
                resp.set_mut(format!("not a number: {:?}\n", e));
                return Ok(resp);
            }
            Ok(n) => {
                numbers.push(n);
            }
        }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    resp.set_mut(status::Ok);
    resp.set_mut(mime!(Text/Html; Charset=Utf8));
    resp.set_mut(format!("Numbers of {:?} is <b>{}</b>\n", numbers, d));
    return Ok(resp);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
}

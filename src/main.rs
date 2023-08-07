extern crate iron;
#[macro_use]
extern crate mime;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;

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

use std::collections::HashMap;
use std::io::{self, BufRead};

type Table = HashMap<String, Vec<String>>;

static mut STASH: &i32 = &128;

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn factorial(n: usize) -> usize {
    (1..n + 1).fold(1, |a, b| a * b)
}

fn main() {
    let mut table = Table::new();
    table.insert("AA".to_string(), vec!["aa4".to_string(), "aa2".to_string()]);
    table.insert("BB".to_string(), vec!["bb3".to_string(), "bb2".to_string()]);
    table.insert("CC".to_string(), vec!["cc1".to_string(), "cc2".to_string()]);

    // show(&table);
    // assert_eq!(table["AA"][0], "aa4");

    sort_works(&mut table);
    show(&table);

    let x = 10;
    let y = 20;
    let mut r = &x;
    let b = true;
    if b {
        r = &y;
    }
    assert!(*r == 10 || *r == 20);

    let q = factorial(6);
    assert_eq!(q, 720);
    let r = &factorial(6);
    assert_eq!(r + 1009, 1729);

    // let r;
    // {
    //     let x = 1;
    //     r = &x;
    // }
    // assert_eq!(*r, 1);

    unsafe {
        assert_eq!(*STASH, 128);
    }
    static SOME_STATIC: i32 = 1000;
    f(&SOME_STATIC);
    unsafe {
        assert_eq!(*STASH, 1000);
    }

    let x = 10;
    g(&x);
    let y;
    {
        let x = vec![21, 2, 64, 5, 6, 7, 98];
        y = *smallest(&x);
    }
    assert_eq!(y, 2);

    // let s;
    // {
    //     let x = 10;
    //     s = S { r: &x };
    // }
    // assert_eq!(*s.r, 10);

    let mut wave = Vec::new();
    let head = vec![0.0, 1.];
    let tail = [0., -1.];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    assert_eq!(wave, vec![0., 1., 0., -1.]);

    let mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0;
    *m0 = 137;
    let r1 = &m.1;
    assert_eq!(v.1, 139);
}

fn g<'a>(_p: &'a i32) {
    //
}

#[allow(dead_code)]
struct S<'a> {
    r: &'a i32,
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    return s;
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

#[allow(dead_code)]
// fn read_numbers(file: &mut dyn BufRead) -> Result<Vec<i64>, io::Error> {
fn read_numbers(file: &mut dyn BufRead) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}

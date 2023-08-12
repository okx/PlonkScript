#[allow(dead_code)]
fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

#[test]
fn test_test() {
    assert_eq!(build_vector(), vec![10, 20]);

    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);

    assert_eq!(2u32.pow(4), 16);
    assert_eq!((-4i8).abs(), 4);
    assert_eq!((7i8).count_ones(), 3);

    let mut arr1 = [1, 12, 4, 7, 8, 10];
    assert_eq!(arr1[3], 7);

    arr1.sort();
    assert_eq!(arr1[3], 8);

    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);
    v.push(8);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210 * 8);

    //slices
    let v = vec![0.0, 0.707, 1.0, 0.707];
    let a = [0.0, 0.707, 1.0, 0.707];
    // let sv: &[f64] = &v;
    // let sa = &a;
    print(&v);
    print(&a);

    // let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    // let t = s;
    // let u = s;

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    let first = std::mem::replace(&mut v[0], "sub".to_string());
    assert_eq!(first, "101");

    assert_eq!(v, vec!["sub", "102", "103", "104", "105"]);
}

#[allow(dead_code)]
fn print(n: &[f64]) {
    for e in n {
        println!("{}", e);
    }
}

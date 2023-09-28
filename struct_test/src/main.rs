type Size = (usize, usize);

struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

fn main() {
    let w = 1024;
    let h = 576;
    let image = GrayscaleMap {
        pixels: vec![0; w * h],
        size: (w, h),
    };
}

fn new_map(size: Size, pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Clone, Copy)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut b1 = Broom {
        height: b.height / 2,
        ..b
    };

    let mut b2 = Broom {
        name: b1.name.clone(),
        ..b1
    };

    b1.name.push_str(" I");
    b2.name.push_str(" II");

    (b1, b2)
}

#[test]
fn test_chop(){
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.,200.,0.),
        intent: BroomIntent::FetchWater,
    };
    let (h1,h2) = chop(hokey);

    assert_eq!(h1.name, "Hokey I");
    assert_eq!(h2.name, "Hokey II");
}
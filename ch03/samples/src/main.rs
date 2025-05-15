fn m() {
    let a = 10;
    let b = 20;
    {
        let c = 30;
        let d = 40;
    }
}

fn n() {
    let e = 50;
    let f = 60;
}

struct H2O {}
struct O2 {}
struct H2 {}

fn burn(_h2_1: H2, _h2_2: H2, _o2: O2) -> (H2O, H2O) {
    (H2O {}, H2O {})
}
fn main() {
    let h2_1 = H2 {};
    let h2_2 = H2 {};
    let o2 = O2 {};

    let (h2o_1, h2o_2) = burn(h2_1, h2_2, o2);

    // 既に消費した分子は使えない
    // let (h2o_1, h2o_2) = burn(h2_1, h2_2, o2);
    
    struct Coin {}

    let a = Coin {};
    let b = a;
    let c = b;

    // let d = a;

    let a = 200;
    {
        let b = 10;
        // a = &b;
    }
    println!("{}", a);

    let a;
    {
        let b = 10;
        a = &b;
        println!("{}", a);
    }

    let a: i32 = 10;
    let b: &i32 = &a;

    square(b);
    Foo {x: &a};

    let mut x = 10;
    {
        let y = 20;
        add(&mut x, &y);
    }
    println!("{x}");

}

fn square<'a> (x: &'a i32) -> i32 {
    x * x
}

struct Foo<'a> {
    x: &'a i32,
}

fn add<'a> (x: &'a mut i32, y: &'a i32) {
    *x += *y;
}
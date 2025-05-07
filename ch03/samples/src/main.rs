fn main() {
    println!("Hello, world!");
}

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
struct O2 ()
struct H2 {}

fn burn(_h2_1: H2, _h2_2: H2, _o2: O2) -> (H2O, H2O) {
    (H2O {}, H2O {})
}

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

 let a;
 {
    let b = 10;
    a = &b;
 }
 println!("{}", a);
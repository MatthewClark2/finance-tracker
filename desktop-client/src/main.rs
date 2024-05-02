use currency::USD;

mod currency;

fn main() {
    let c1 = USD::new(0, 0);
    let c2 = USD::new(15, 31);
    let c3 = c1 - c2;
    println!("{c3}");
}

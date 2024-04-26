use currency::USD;

mod currency;

fn main() {
    let c1 = USD::new(0, 0);
    let c2 = USD::new(15, 31);
    let _c3 = c1.sub(&c2);
}

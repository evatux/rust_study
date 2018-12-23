use std::env;

fn fact(n: u64) -> u64 {
    match n {
        0 ... 1 => 1,
        _ => n * fact(n-1),
    }
}

fn main() {
    for arg in env::args().skip(1) {
        let n = arg.parse::<u64>().unwrap();
        let res = fact(n);
        println!("fact({}) = {}", n, res);
    }
}

#[test]
fn check() {
    assert_eq!(fact(0u64), 1u64);
    assert_eq!(fact(1u64), 1u64);
    assert_eq!(fact(4u64), 24u64);
}

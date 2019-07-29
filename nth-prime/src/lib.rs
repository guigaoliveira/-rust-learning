fn is_prime(number: u32) -> bool {
    let limit = (f64::from(number)).sqrt() as u32 + 1;
    !(2..limit).any(|i| number % i == 0)
}

pub fn nth(n: u32) -> u32 {
    let mut nth = 0;
    let mut i = 2;

    loop {
        if is_prime(i) {
            if nth == n {
                break;
            }
            nth += 1;
        }
        i += 1;
    }
    i
}

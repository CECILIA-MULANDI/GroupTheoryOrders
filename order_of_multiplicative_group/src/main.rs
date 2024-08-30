fn main() {
    let number = 9700;
    let factors = get_factors(number);
    println!("Prime factors of {}: {:?}", number,factors);
}

fn is_prime(n: i32) -> bool {
    match n {
        0 | 1 => false,
        _ => (2..=((n as f64).sqrt() as i32)).all(|i| n % i != 0),
    }
}

fn get_factors(n: i32) -> Vec<i32> {
    let limit = (n as f64).sqrt() as i32;
    let mut factors = Vec::new();
    for i in 2..=limit {
        if n % i == 0 {
            if is_prime(i) {
                factors.push(i);
            }
            let pair = n / i;
            if pair != i && is_prime(pair) {
                factors.push(pair);
            }
        }
    }
    factors
}

fn order_of_a_number(n: i32, factors: Vec<i32>) -> i32 {
    let mut product = 1.0;
    if is_prime(n) {
        product = (n - 1) as f64;
    } else {
        for i in factors {
            let k = 1.0 - 1.0 / i as f64;
            product *= k;
            println!("I is {i}");
        }
    }
    let order = product * n as f64;
    order as i32
}

fn list_group(n: i32) -> Vec<i32> {
    let mut group = Vec::new();
    if !is_prime(n) {
        for i in 0..n {
            if gcd(i, n) == 1 {
                group.push(i);
            }
        }
    } else {
        for i in 0..n {
            group.push(i);
        }
    }
    group
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

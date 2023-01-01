use rand::{seq::SliceRandom, thread_rng, Rng};


fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { 
        return a;
    }
    gcd(b, a%b)
}

fn prime_finder() -> usize {
    let test_number = rand::thread_rng().gen_range(10..100);
    for i in 2..test_number {
        if test_number % i == 0 {
            return prime_finder();
        }
    }
    test_number
}

fn possible_encryption_keys(phi: usize, n: usize) -> usize { 
    let mut pub_keys = Vec::new();
    for i in 2..phi {
        if gcd(i, phi) == 1 && gcd(i, n) == 1 {
            pub_keys.push(i);
        }
        if pub_keys.len() >= 100 {
            break;
        }
    }
    let mut rng = thread_rng();
    *pub_keys.choose(&mut rng).unwrap()
}

fn possible_private_keys(phi: usize, e: usize) -> usize {
    let mut priv_keys = Vec::new();
    let mut i = 2;
    while priv_keys.len() < 5 {
        if i * e % phi == 1 {
            priv_keys.push(i);
        }
        i += 1;
    }
    let mut rng = thread_rng();
    *priv_keys.choose(&mut rng).unwrap()
}

fn main() {
    let p = prime_finder();
    let q = prime_finder();
    let n = p*q;
    let phi = (p-1)*(q-1);
    let e = possible_encryption_keys(phi, n);
    let d = possible_private_keys(phi, e);
    println!("Public key: ({e}, {n})\nPrivate key: ({d}, {n})");
}

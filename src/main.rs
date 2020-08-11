use ramp::{Int, RandomInt};
use rand;
use primal;

fn find_prime(bits: usize, order_divisor: &Int) -> (Int, Vec<Int>) {
    // let mut i = 0;
    let k = order_divisor.bit_length();
    loop {
        // if i % 1000 == 0{
        //     println!("tried {} primes so far", i)
        // }
        // i += 1;
        let p = gen_prime(bits - (k as usize) - 7);
        for i in (2usize.pow(7))..(2usize.pow(8)) {
            let q = ((Int::from(i) * p.clone()) * order_divisor) + Int::one();

            if is_prime(&q) {
                println!("Found prime q = {}, with bit size {}", q, q.bit_length());
                println!("q = ({} * {} * {}) + 1", i, &p, &order_divisor);
                let mut prime_factors = vec![p];
                let mut order_div_factors = prime_factor(&(Int::from(i) * order_divisor));
                prime_factors.append(&mut order_div_factors);
                return (q, prime_factors)
            }
        }
    }
}

fn is_prime(p: &Int) -> bool {
    if !(simple_test(p, 2000)) {
        return false
    }
    if *p <= Int::from(17389) {
        return true
    }

    if !little_fermat(p, 5) {
        return false
    }

    if !miller_rabin(p, 5) {
        return false
    }

    true
}

fn simple_test(candidate: &Int, iters: usize) -> bool {
    for p in primal::Primes::all().take(iters) {
        let i = Int::from(p);
        if i == *candidate {
            return true
        }
        let (_, r) = candidate.divmod(&i);
        if r == Int::zero() {
            return false
        }
    }
    true
}

fn prime_factor(n: &Int) -> Vec<Int> {
    if *n <= Int::zero() {
        panic!("wow")
    }
    let mut result = vec![];
    let mut remaining = n.clone();
    loop {
        if remaining == Int::one() {
            break;
        }
        'inner: for p in primal::Primes::all().take(20000) {
            let (q, r) = remaining.divmod(&Int::from(p));
            if r == Int::zero() {
                result.push(Int::from(p));
                remaining = q;
                break 'inner;
            }
        }
    }

    return result
}

fn gen_prime(n: usize) -> Int {
    let mut rng = rand::thread_rng();
    loop {
        let mut big_int: Int = rng.gen_uint(n);
        big_int.set_bit(0, true);
        big_int.set_bit((n-1) as u32, true);
        if is_prime(&big_int) == true {
            return big_int;
        }
    }
}

fn little_fermat(candidate: &Int, iters: usize) -> bool{
    let mut rng = rand::thread_rng();
    let cand_minus_one = candidate - &Int::one();
    for _ in 0..iters {
        let r = rng.gen_uint_below(candidate);
        let result = r.pow_mod(&cand_minus_one, candidate);
        if !(result == Int::one()) {
            return false
        }
    }
    return true
}

fn miller_rabin(candidate: &Int, limit: usize) -> bool {
    let mut rng = rand::thread_rng();

    let one = Int::one();
    let two = Int::from(2);
    let cand_minus_one = candidate - &one;
    let cand_minus_two = candidate - &two;

    let (s,d) = rewrite(&cand_minus_one);

    'outer: for _ in 0..limit {
        let basis = rng.gen_int_range(&two, &cand_minus_two);
        let mut y = basis.pow_mod(&d, &candidate);

        if y == one || y == (cand_minus_one) {
            continue;
        } else {
            for _ in 0..s-1 {
                y = y.pow_mod(&two, &candidate);
                if y == one {
                    return false
                } else if y == cand_minus_one {
                    continue 'outer;
                }
            }
            return false;
        }
    }
    true
}

fn rewrite(n: &Int) -> (usize, Int) {
    let mut d = n.clone();
    let mut s = 0;

    while d.is_even() {
        d = d >> 1;
        s += 1;
    }
    return (s, d)
}

fn find_gen(prime: &Int, factors_: Vec<Int>) -> Int {
    let mut factors = factors_.clone();
    factors.dedup();
    let order = prime - Int::one();
    let mut gen = Int::from(2);
    loop {
        if factors.iter().all(|factor| {
            let k = order.clone() / factor.clone();
            if gen.pow_mod(&k, &prime) == Int::one() {
                return false
            }
            return true
        }) {
            return gen
        } else {
            gen = gen + Int::one();
        }
    }
}

fn main() {
    let mut order_div = Int::from(2).pow(10);
    order_div = order_div * Int::from(3).pow(9);
    let (prime, factors) = find_prime(128, &order_div);
    println!("factors: {:?}", factors);

    let gen = find_gen(&prime, factors);
    println!("generator: {}", gen);
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRIMES: [u64;32] =
        [4611686018427388039,
         4611686018427388073,
         4611686018427388081,
         4611686018427388091,
         4611686018427388093,
         4611686018427388097,
         4611686018427388157,
         4611686018427388181,
         4611686018427388207,
         4611686018427388247,
         4611686018427388273,
         4611686018427388279,
         4611686018427388289,
         4611686018427388291,
         4611686018427388319,
         4611686018427388331,
         4611686018427388349,
         4611686018427388361,
         4611686018427388387,
         4611686018427388429,
         4611686018427388447,
         4611686018427388463,
         4611686018427388477,
         4611686018427388513,
         4611686018427388519,
         4611686018427388601,
         4611686018427388609,
         4611686018427388699,
         4611686018427388721,
         4611686018427388787,
         4611686018427388793,
         4611686018427388853];

    #[test]
    fn test_rewrite() {
        let mut rng = rand::thread_rng();
        let two = Int::from(2);

        for _ in 0..1000 {
            let mut big_int: Int = rng.gen_uint(128);
            let (s, d) = rewrite(&big_int);
            assert_eq!(big_int, two.pow(s) * &d)
        }
    }

    #[test]
    fn test_primes() {
        for p in PRIMES.iter() {
            assert!(is_prime(&Int::from(*p)));
        }
    }

    #[test]
    fn test_prime_factor() {
        let mut rng = rand::thread_rng();
        let four = Int::from(4);
        let hundred_thousand = Int::from(100000);

        for _ in 0..100 {
            let r = rng.gen_int_range(&four, &hundred_thousand);
            let factors = prime_factor(&r);
            for f in factors.iter() {
                let isp = is_prime(f);
                if !isp {
                    println!("{:?}", f);
                }

                assert!(isp);
            }
            assert_eq!(r, factors.iter().fold(Int::one(), |p, f| p * f));
        }
    }
}

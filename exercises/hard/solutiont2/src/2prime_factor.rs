// src/prime_factor.rs
extern crate rand;
use rand::Rng;

// 计算 gcd
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Miller-Rabin 素性测试
fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31].iter() {
        if n % *p == 0 {
            return n == *p;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    for _ in 0..10 {
        let a = rand::thread_rng().gen_range(2..n - 1);
        let mut x = pow_mod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = pow_mod(x, 2, n);
            if x == n - 1 {
                break;
            }
        }
        if x != n - 1 {
            return false;
        }
    }
    true
}

// 快速幂取模
fn pow_mod(mut x: u128, mut y: u128, p: u128) -> u128 {
    let mut result = 1;
    while y > 0 {
        if y % 2 == 1 {
            result = (result * x) % p;
        }
        x = (x * x) % p;
        y /= 2;
    }
    result
}

// 使用 Pollard Rho 算法寻找因子
fn pollard_rho(p: u128) -> u128 {
    if p % 2 == 0 {
        return 2;
    }
    if is_prime(p) {
        return p;
    }

    let mut rng = rand::thread_rng();
    loop {
        let mut x = rng.gen_range(2..p);
        let mut y = x;
        let c = rng.gen_range(1..p);
        let mut d = 1;
        let mut z = 1;
        let mut i = 0;
        let mut j = 1;

        while d == 1 {
            x = (pow_mod(x, 2, p) + c) % p;
            y = (pow_mod(y, 2, p) + c) % p;
            y = (pow_mod(y, 2, p) + c) % p;

            z = (z * (if y > x { y - x } else { x - y })) % p;

            if z == 0 {
                break;
            }

            if i % 127 == 0 || i == j {
                d = gcd(z, p);
                if d > 1 && d < p {
                    return d;
                }
                if i == j {
                    y = x;
                    j <<= 1;
                }
            }

            i += 1;
        }
    }
}

// 找最大素因子
pub fn find_max_prime_factor(mut number: u128) -> u128 {
    let mut ans = 1;

    // 先检查小素数因子
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31].iter() {
        if number % *p == 0 {
            ans = ans.max(*p);
            while number % *p == 0 {
                number /= *p;
            }
        }
    }

    // 使用 Pollard Rho 分解大数
    while number > 1 {
        if is_prime(number) {
            ans = ans.max(number);
            break;
        }
        let factor = pollard_rho(number);
        ans = ans.max(factor);
        while number % factor == 0 {
            number /= factor;
        }
    }

    ans
}
// 计算 gcd
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// 确定性 Miller-Rabin 素性测试
fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    // 试除小素数
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97].iter() {
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
    // 确定性 Miller-Rabin 测试
    for &a in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37].iter() {
        if a >= n {
            continue;
        }
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

// 使用确定性 Pollard Rho 算法寻找因子
fn pollard_rho(p: u128) -> u128 {
    if p % 2 == 0 {
        return 2;
    }
    if is_prime(p) {
        return p;
    }

    let mut x = 2;
    let mut y = 2;
    let c = 1;
    let mut d = 1;

    while d == 1 {
        x = (pow_mod(x, 2, p) + c) % p;
        y = (pow_mod(y, 2, p) + c) % p;
        y = (pow_mod(y, 2, p) + c) % p;

        d = gcd(if x > y { x - y } else { y - x }, p);
    }

    if d == p {
        // 如果未找到因子，重新尝试
        pollard_rho(p)
    } else {
        d
    }
}

// 找最大素因子
pub fn find_max_prime_factor(mut number: u128) -> u128 {
    let mut ans = 1;

    // 先检查小素数因子
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97].iter() {
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

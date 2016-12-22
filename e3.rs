fn main(){
    let mx: i64 = 600851475143;
    let mut pr: i64 = 0;
    let mut primes: Vec<i64> = vec![2i64];
    for x in 2..mx{
        if prime(x, &primes) {
            if mx % x == 0{
                pr = x;
                println!("prime factor: {}", x);
            }
            primes.push(x);
        }
    }
    println!("max: {}", pr);
}

fn prime(x: i64, primes: &Vec<i64>) -> bool{
    let mut max_prime: i64 = 2;
    for pr in 0..primes.len(){
        if primes[pr] > max_prime{
            max_prime = primes[pr];
        }
        if x % primes[pr] == 0{
            return false;
        }
    }

    for z in max_prime..x{
        if x % z == 0{
            return false;
        }
    }
    return true;
}
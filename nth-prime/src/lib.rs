pub fn nth(n: u32) -> u32 {
    let prime = match n {
        0 => 2,
        1 => 3,
        _ => prime(n)
    };

    prime
}

fn prime(nth_prime: u32) -> u32 {
    let mut prime: u32 = 3;
    let mut prime_counter: u32 = 0;
    let mut i: u32 = 2;
    let mut n: u32;

    while prime_counter < nth_prime {
        i += 1;
        n = 2;
        while n < i {
            if i % n == 0 {
                break;
            }

            n += 1;
            
            if n == i {
                prime = i;
                prime_counter += 1;
            }
        }
    }

    prime
}
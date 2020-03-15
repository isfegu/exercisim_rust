pub fn nth(n: u32) -> u32 {
    let mut evaluation_counter: u32 = 2;
    let mut prime_counter: u32 = 0;

    let nth_prime = loop {
        if is_prime(evaluation_counter) {
            prime_counter += 1;
        }

        if (prime_counter - 1) == n {
            break evaluation_counter;
        }

        evaluation_counter += 1;
    };

    nth_prime
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    return prime_evaluation(n, 2)
}

fn prime_evaluation(n: u32, counter: u32) -> bool {
    if counter != n {
        if n % counter != 0 {
            return prime_evaluation(n, counter + 1);
        } 
        return false;
    }

    true
}

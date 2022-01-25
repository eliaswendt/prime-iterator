fn main() {
    // store all already found primes
    let mut prime_numbers: Vec<u64> = Vec::with_capacity(1000000000);
    prime_numbers.push(2);


    for current_number in (3..u64::MAX).step_by(2) {
        if current_number % 1000001 == 0 {
            println!("{}", prime_numbers.len());
        }

        let current_numbers_square_root = (current_number as f64).sqrt() as u64;

        let mut current_number_could_be_prime = true;

        // iterate over all already found prime numbers
        for prime_number in prime_numbers.iter() {
            if *prime_number > current_numbers_square_root {
                break
            }

            if current_number % *prime_number == 0 {
                // current number can be devided by a previous prime number -> can't be prime
                current_number_could_be_prime = false;
                break
            }
        }

        if current_number_could_be_prime {
            prime_numbers.push(current_number);
        }
    }
}

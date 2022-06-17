use std::time::Instant;

/// Iterator for prime numbers
/// WARNING: It currently does not yield the first two primes 2 and 3!
struct PrimeIterator {
    discovered_primes: Vec<u64>
}

impl PrimeIterator {
    pub fn new() -> Self {
        let mut discovered_primes: Vec<u64> = Vec::with_capacity(1000000000);
        discovered_primes.push(2); // per definition
        discovered_primes.push(3); // optimization: its faster to only search on odd numbers

        Self {
            discovered_primes
        }
    }
}

impl Iterator for PrimeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {

        for current_number in (self.discovered_primes.last().unwrap()+2..u64::MAX).step_by(2) {

            let current_numbers_square_root = (current_number as f64).sqrt() as u64;

            let mut current_number_could_be_prime = true;
    
            // iterate over all already found prime numbers
            for prime_number in self.discovered_primes.iter() {
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
                // if number could still be prime, it is!
                self.discovered_primes.push(current_number);
                return Some(current_number)
            }
        }

        // return None if there is no number left in the u64 space
        Option::None
    }
}


fn main() {
    // store all already found primes

    let prime_iterator = PrimeIterator::new();

    let mut time_since_last = Instant::now();
    
    for (i, prime) in prime_iterator.enumerate() {
        println!("{}: time(ns)={}, prime={}", 
            i, 
            time_since_last.elapsed().as_nanos(),
            prime 
        );

        time_since_last = Instant::now();
    }
}

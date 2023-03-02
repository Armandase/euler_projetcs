fn sqrt(nb : u64) -> u64{
    let mut i = 0;
    while i < nb{
        if i * i >= nb{
            return i;
        }
        i += 1;
    }
    return 0;
}

fn is_prime(nb : u64) -> bool{
    let max = sqrt(nb + 1);
    for i in 2..max{
        if nb % i == 0{
            return false;
        }
    }
    return true;
}

pub fn summation_of_primes() {
    let max = 2000000;
    let mut number : u64 = 2;
    for i in (3..max).step_by(2){
        if is_prime(i){
            number += i;
        }
    }
    println!("The sum of all the primes below 2 000 000 is {number}");
}

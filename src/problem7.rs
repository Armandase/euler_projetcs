fn sqrt(nb : u64) -> u64{
    let mut i = 0;
    while i < nb{
        if i * i >= nb {
            return i;
        }
        i += 1
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

pub fn the_10001st_prime_number(){
    let mut last = 2;

    for _i in 0..10001{
        loop{
            if is_prime(last){
                last += 1;
                break ;
            }
            last += 1;
        }
    }
    last -= 1;
    println!("The 10001st prime number is {last}");
}

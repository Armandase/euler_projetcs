fn sqrt(nb : u64) -> u64{
    let mut index = 0;
    while index < nb{
        if index * index >= nb{
            return index as u64;
        }
        index += 1;
    }
    return 0;
}

pub fn largest_prime_factor() -> i32{
    let pb : u64 = 600851475143;
    let mut factor = 2;
    let mut nb = 0;
    let tmp;
    let max = sqrt(pb);
   
    while pb % factor != 0{
        factor += 1;
    }
    tmp = factor;
    while factor < max {
        if pb % factor == 0{
            nb = factor;
        }
        factor += 1;
    }
    nb /= tmp;
    nb as i32
}

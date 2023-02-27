fn sqrt(number : u64) -> u64{
    let mut index: u64 = 0;
    let mut tmp;

    if number <= 0{
        return 0;
    }
    while index <= 1000{
        tmp = index * index;
        if tmp >= number{
            return index;
        }
        index += 1;
    }
    return 0;
}

fn is_prim (number: u64) -> bool{
    let max = sqrt(number + 1);

    for i in 2..max{
        if number % i == 0{
            return false;
        }
    }
    return true;
}

fn pow (nb : u64, power : u64) -> u64{
    let mut ret = nb;
    let mut _i = 1;

    if power == 0 {
        return 1;
    }
    while _i < power{
        ret *= nb;
        _i += 1;
    }
    return ret;
}

fn is_cube_from_nu(nu : u64) -> bool{
    let mut i = 0;
    while pow(i, 3) < nu + 1{
        if pow(i, 3) == nu{
            return true;
        }
        i += 1;
    }
    return false
}

fn search_partnership(nb : u64) -> bool{
    let mut i = 1;
    let mut index = pow(i, 3);

    while index < nb
    {
        let tmp : u64 = pow(index, 3) + pow(index, 2) * nb;
        if is_cube_from_nu(tmp) == true{
            return true
        }
        i += i;
        index = pow(i, 3);
    }
    return false
}

pub fn prime_cube_paternship() -> u64{
    let max : u64 = 1000000;
    let mut counter : u64 = 0;

    for i in 1..max{
        if is_prim(i) == true && search_partnership(i) == true{
            println!("nb : {i}");
            counter += 1;
        }
    }
    return counter;
}

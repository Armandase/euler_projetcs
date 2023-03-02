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

    if number % 2 == 0{
        return false;
    }
    for i in (3..max).step_by(2){
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
    let mut pow_i;

    pow_i = pow(i, 3);
    while pow_i < nu + 1{
        pow_i = pow(i, 3);
        if pow_i == nu{
            print!("ciruclar : {} from ->", i);
            return true;
        }
        i += 1;
    }
    return false
}

fn search_partnership(nb : u64, before : u64) -> u64{
    let mut index = before;
    let max = nb * 8;

    while index < max
    {
        let tmp : u64 = pow(index, 3) + pow(index, 2) * nb;
        if is_cube_from_nu(tmp) == true{
         //   println!("tmp {tmp} = {} + {} * {nb} ({index})", pow(index, 3), pow(index, 2));
            return index
        }
        index += 1;
    }
    return 1
}

pub fn prime_cube_paternship(){
    let max : u64 = 1000000;
    let mut counter : u64 = 0;
    let mut before = 1;
    let mut tmp;

    for i in (3..max).step_by(2){
        if is_prim(i) == true {
            tmp = search_partnership(i, before);
            if tmp > 1{
                before = tmp;
                println!("nb : {i}");
                counter += 1;
            }
        }
    }
    println!("resultat : {counter}");
}

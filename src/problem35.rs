fn is_prim (number: usize) -> bool{
    let max = sqrt(number + 1);
    for i in 2..max{
        if number % i == 0{
            return false;
        }
    }
    return true;
}

fn nb_len (mut number : usize) -> usize{
    let mut index = 1;
    while number > 10{
        number = number / 10;
        index += 1;
    }
    return index;
}

fn sqrt(number : usize) -> usize{
    let mut index: usize = 0;
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

fn turn_number(mut number : usize) -> usize{
    let mut tmp: usize;
    if nb_len(number) ==  1{
        return number;
    }
    tmp = number % 10;
    let mut i = 1;
    while i < nb_len(number){
        tmp *= 10;
        i+=1;
    }
    number /= 10;
    number += tmp;
    return number;
}

pub fn circular_primes() -> usize{
    let mut counter: usize = 0;

    for i in 2..1000000{
        let mut tmp:usize = i;
        let mut j:usize = 0;
        while j < nb_len(i){
            if is_prim(tmp) == false{
                break ;
            }
            tmp = turn_number(tmp);
            j += 1;
        }
        if j == nb_len(i){
            counter += 1;
        }
    }
    return counter;
}

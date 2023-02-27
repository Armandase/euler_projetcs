fn number_len(mut nb : i32) -> i32{
    let mut ret : i32 = 1;

    while nb >= 10{
        nb /= 10;
        ret += 1;
    }
    return ret;
}

fn pow (nb : i32, power : i32) -> i32{
    let mut ret = nb;
    let mut _i = 1;

    if power < 0{
        return 0;
    } else if power == 0 {
        return 1;
    }
    while _i < power{
        ret *= nb;
        _i += 1;
    }
    return ret;
}

fn is_palindrome(nb : i32) -> bool{
    let len = number_len(nb);
    let high : i32;
    let mut low;
    let mut tmp;

    high = nb / pow(10, len / 2);
    low = nb % pow(10, len / 2);
    tmp = 0;
    for i in 0..len / 2{
        tmp += low / pow(10, len / 2 - i - 1) * pow(10, i);
        low %= pow(10, (len / 2)- i - 1);
    }
    if high == tmp{
        return true;
    }
    false
}

pub fn largest_palindrome_product(){
    let max : i32 = 1000;
    let mut number : i32 = 0;

    for i in 100..max{
        for j in 100..max{
            if is_palindrome(i * j) && i * j > number {
                number = i * j;
            }
        }
    }
    println!("The largest palindrome made from the product of two 3-digit numbers is {number}");
}

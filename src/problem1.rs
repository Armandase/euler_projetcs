fn iter_in_nb(mut nb : i32, max : usize) -> i32{
    let mut ret :i32 = 0;
    let init = nb;

    while nb < max as i32 {
        ret += nb;
        nb += init;
    }
    return ret;
}

pub fn multiple_of_3_or_5(){
    let max : usize = 1000;
    let mut sum = iter_in_nb(3, max);
    sum += iter_in_nb(5, max);
    sum -= iter_in_nb(15, max);
    println!("The sum of all multiples of 3 or 5 below 1000 is {sum}");
    return ;
}

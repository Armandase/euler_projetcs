fn check_divider(nu : u64) -> bool {
    for i in (2..20).rev(){
        if nu % i != 0{
            return false;
        }
    }
    return true
}


pub fn smallest_multiple(){
    for i in (20..u64::MAX).step_by(20){
        if check_divider(i){
            println!("The smallest positive number that can be devided by each of the number from 1 to 20 is {i}");
            return ;
        }
    }
    println!("The smallest positive number that can be devided by each of the number from 1 to 20 is no found");
}

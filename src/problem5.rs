fn check_divider(nu : u64) -> bool {
    for i in (2..20).rev(){
        if nu % i != 0{
            return false;
        }
    }
    return true
}


pub fn smallest_multiple() -> i32{
    for i in (20..u64::MAX).step_by(20){
        if check_divider(i){
            return i as i32;
        }
    }
    return -1;
}

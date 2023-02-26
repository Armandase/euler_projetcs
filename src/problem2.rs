pub fn even_fibonacci_numbers() -> i32{
    let mut sum = 0;
    let mut before = 0;
    let mut current = 1;
    let max = 4000000;

    while current < max{
        before = current - before;
        current += before;
        if current % 2 == 0{
            sum += current;
        }
    }
    return sum;
}

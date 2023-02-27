pub fn even_fibonacci_numbers(){
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
    println!("The sum of the even number in the Fibonacci sequence is {sum}.");
    return ;
}

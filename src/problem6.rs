pub fn sum_square_difference(){
    let mut sum_of_square : u64 = 0;
    let mut square_of_sum : u64 = 0;
    let result;

    for i in 1..101{
        sum_of_square += i * i;
    }
    for i in 1..101{
        square_of_sum += i;
    }
    square_of_sum *= square_of_sum;
    result = square_of_sum - sum_of_square;
    println!("The difference between the sum of the squares from 0 to 100 and the square of the sum is {result}");
}

pub mod problem35;
pub mod problem1;
pub mod problem2;

fn main() {
    println!("There is {} numbers", problem35::circular_primes());
    println!("There is {} numbers", problem1::multiple_of_3_or_5());
    println!("There is {} numbers", problem2::even_fibonacci_numbers());
}

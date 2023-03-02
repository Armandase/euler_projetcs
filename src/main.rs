use std::io::stdin;
pub mod problem1;
pub mod problem2;
pub mod problem3;
pub mod problem4;
pub mod problem5;
pub mod problem6;
pub mod problem7;
pub mod problem8;
pub mod problem10;
pub mod problem35;
pub mod problem131;

fn get_problem(s : &String){
    type Pb = fn();
    let nu : usize = s.trim().parse().unwrap();
    let tab: [Pb; 12] = [problem1::multiple_of_3_or_5
        , problem2::even_fibonacci_numbers
        , problem3::largest_prime_factor
        , problem4::largest_palindrome_product
        , problem4::largest_palindrome_product
        , problem5::smallest_multiple
        , problem6::sum_square_difference
        , problem7::the_10001st_prime_number
        , problem8::largest_product_in_a_series
        , problem10::summation_of_primes
        , problem35::circular_primes
        , problem131::prime_cube_paternship];
    tab[nu]();
}

fn main() {
    let mut s = String::new();

    println!("You can select the problem nº");
    println!("\t1, 2, 3, 4, 5, 6, 7, 8, 10, 35, 131");
    loop {
        stdin().read_line(&mut s).expect("Error in standard in put");
        if s.trim() == ""{
            break ;
        }
        get_problem(&s);
        s = String::new();
    }
}

use std::io::stdin;
pub mod problem1;
pub mod problem2;
pub mod problem3;
pub mod problem4;
pub mod problem5;
pub mod problem6;
pub mod problem7;
pub mod problem35;
pub mod problem131;

fn get_problem(s : &String){
    if s.trim() == "1"{
        problem1::multiple_of_3_or_5();
    }
    if s.trim() == "2"{
        problem2::even_fibonacci_numbers();
    }
    if s.trim() == "3"{
        problem3::largest_prime_factor();
    }
    if s.trim() == "4"{
        problem4::largest_palindrome_product();
    } 
    if s.trim() == "5"{
        problem5::smallest_multiple();
    }
    if s.trim() == "6"{
        problem6::sum_square_difference();
    }
    if s.trim() == "7"{
        problem7::the_10001st_prime_number();
    }
    if s.trim() == "35"{
        problem35::circular_primes();
    }
}

fn main() {
    let mut s = String::new();

    println!("You can select the problem nº");
    println!("\t1, 2, 3, 4, 5, 6, 7, 35");
    loop {
        stdin().read_line(&mut s).expect("Error in standard in put");
        if s.trim() == ""{
            break ;
        }
        get_problem(&s);
        s = String::new();
    }
}

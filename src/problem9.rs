pub fn special_pythagorean_triplet(){
    let max;

    max = 1000;
    for i in 1..max{
        for j in 1..i{
            for k in 1..j{
                if k * k + j * j == i * i && k + j + i == max{
                    println!("The product of abc wich a + b + c == 1000 and there are a pythagorean triplet is {}", k * j * i);
                    return ;
                }
            }
        }
    }
}

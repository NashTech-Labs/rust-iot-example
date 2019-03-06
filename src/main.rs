fn factorial(num: u64) -> u64
{
    let mut i:u64 = num;
    let mut response:u64 = 1;
    while i>1 {
                  response = response*i;
                  i = i-1;
               }
    response
}

fn main() {
    println!("{}",factorial(3));
    println!("{}",factorial(7));
    println!("{}",factorial(8));
}
use std::io;
fn main() {
    println!("To find the nth fibonacci number, please input n");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i64 = n.trim().parse().expect("Not a number");
    println!("The {}th fibonacci number is {}", n, next(0, 1, n))
}
//n0 is first number, n1 is second, steps is iterations to undergo, starting at 0
fn next(mut n0:i128,mut n1:i128, steps:i64) -> i128 { 
    let mut count: i64 = 0;
    while count + 4 <= steps {
        let temp: i128 = n1;
        n1 = n1 + n0;
        n0 = temp;
        count += 1;
    }
    return n1 + n0
}
use std::io;

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    else if  n == 1 {
        return 1;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn main() {
    let mut number: String = String::new();

    println!("Please enter the index of a number in the Fibonacci sequence.");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = number.trim().parse().expect("Please type a number");
    let f_num: i32 = fibonacci(number);

    println!("\n\nThe number at index {} is {}", number, f_num);
}

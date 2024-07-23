use std::io;

fn main() {
    'run_program: loop {
        print_menu();
        let mut convert = String::new();
    
        io::stdin()
            .read_line(&mut convert)
            .expect("Failed to read line");
    
        match convert.trim() {
            "1" => {
                f_to_c(true);
            },
            "2" => {
                f_to_c(false);
            },
            "3" | "q" | "Q" => {
                break 'run_program;
            },
            _ => println!("Please review the menu options"),
        }
    }
}

fn f_to_c(to_celsius: bool) {
    let mut number_to_convert = String::new();

    let initial_degrees = String::from({
        if to_celsius {
            "Fahrenheit"
        } else {
            "Celsius"
        }
    });

    println!("\n\n");
    println!("Please enter the amount in {} that you wish to convert: ", initial_degrees);

    io::stdin()
        .read_line(&mut number_to_convert)
        .expect("Failed to read line");

    let number: f64 = number_to_convert.trim().parse().expect("Please type a number");

    let degrees = {
        if number == 1.0 || number == -1.0 {
            "degree"
        } else {
            "degrees"
        }
    };

    if to_celsius {
        println!("\n>>>> The value of {} {} Fahrenheit in Celsius is {} <<<<", number, degrees, fahrenheit_converter(number));
    } else {
        println!("\n>>>> The value of {} {} Celsius in Fahrenheit is {} <<<<", number, degrees, celsius_converter(number));
    }
    println!("\n\n");
}

fn fahrenheit_converter(x: f64) -> f64 {
    (x - 32.0) / 1.8
}

fn celsius_converter(x: f64) -> f64 {
    x * 1.8 + 32.0
}

fn print_menu() {
    println!("Welcome to FCConverter! Please read carefully, as our menu options have changed.");
    println!("1. Convert from Fahrenheit to Celsius.");
    println!("2. Convert from Celsius to Fahrenheit.");
    println!("2. Exit the program.");

    println!("\n\nPlease input 1, 2, or 3: ");
}

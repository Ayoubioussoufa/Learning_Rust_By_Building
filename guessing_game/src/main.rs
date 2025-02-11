use std::io; //imported io from the standard library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!: "); // ! means that its a macro not a function

    let secret_number = rand::rng().random_range(1..=100);
    // println!("The secret number is: {}", secret_number);

    loop {
    println!("Please input your guess: ");
    
    let mut guess = String::new(); // let to create a variable
    // in Rust, variables are immutable by default = we cant change them
    // thats why we add mut before the variable name
    // String::new returns a new empty instance of a String provided by the std

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    // if we didnt import the io, we would have to use std::io:stdin()... 
    // & = reference like C, without needing to copy that data into memory multiple times
    // reference are immutable by default thats why we write &mut rather than &guess
    // read_line returns a Result enumeration, that has one or multiple states
    // Result’s variants are Ok and Err
    
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,   
    };
    // Convert the guess from a String to a u32 (unsigned 32-bit integer)
    // we switch from expect to match expression to move from crashing on errors
    // parse return Result enum that has Ok and Err, 

    println!("You guessed: {guess}");


    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
}
    // The Ordering type is another enum and has the variants Less, Greater, and Equal
}

/*
This project was a hands-on way to introduce you to many new Rust concepts: let, match, functions, the use of external crates, and more */
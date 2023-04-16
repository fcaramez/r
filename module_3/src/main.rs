fn main() {
    // Keyword mut allows mutation of the string
    // Variables without mut cannot be mutated
    let mut x = 5;
    println!("The value of x is {x}");

    x = 6;

    println!("The value of x is {x}");

    // Constant
    // Constants require a type definition
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Semantically is uppercase

    // Shadowing => re-declaration of a variable on a different scope:

    let y = 5;

    let y = y * 5;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }

    println!("The value of y is {y}");

    // Shadow vs mut

    // With mut it is not possible to assign a value of a different type to the same variable
    /* let mut guess = "      ";
    guess = guess.len() */

    // With shadow we can re-declare that variable using the old value plus something else
    let guess = "    ";
    let guess = guess.len();

    // Numeric Operators

    // Addition
    let sum = 5 + 10;

    // Subtraction
    let difference = 95.5 - 4.3;

    // Multiplication
    let product = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // Remainder
    let remainder = 43 % 5;

    // Booleans

    let is_raining = false;
    const GOOD_DEV: bool = false;

    // Chars
    // Chars are defined by a single character using Single quotes ('')

    let first_letter = 'x';
    let z: char = 'Z'; // Explicit type annotation

    // Compound Types

    // Tuples => grouping together variety of types into one single compound type

    // Explicit type annotation
    let tup: (i32, f64, u8) = (5000, 6.4, 1);

    // Infer:
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup; // Destructuring

    println!("The value of z inside the tup is {z}");

    // Can also extract values based on index

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    // Array types

    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let repeated = [3; 5]; // => Will generate an array with 5 "3", the first argument is the element to repeat and the second the amount of repetitions
    print_hello_world();
    print_two_numbers(5, 10);
    let sum = sum_two_numbers(10, 20);

    // If expressions

    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // Unlike JS/TS, rust will not associate numbers or other datatypes as bools when evaluating conditions

    /* if number {
        println!("Number exists!")
    } */

    if number != 0 {
        println!("Number is different than 0")
    }

    // Chaining if with else if statement

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let my_name = "Xico";
    // If name is xico age = 20, else age = 25
    let age = if my_name == "Xico" { 20 } else { 25 };

    // Loops

    // Loop => Basically an infinite loop

    /*  loop {
           println!("again!");
       }
    */
    let mut counter = 0;

    // Here our loop is returning the value counter multiplied by 2 and we are assigning it to the result variable

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Using Loop labels for nested loops

    // Here we assign the label 'counting_up to our main loop and depending if a condition is met or not it will break that loop by calling break 'label_of_loop

    // Also, by just writing break it will break the loop in the current scope, not the one above

    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // Conditional Loops

    // While Loop

    let mut number = 2;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF");

    // Looping through collections

    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", arr[index]);

        index += 1;
    }

    // For Loop

    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("The value is: {element}");
    }

    // Reverse loop an array

    for number in (1..4).rev() {
        println!("Number: {number}");
    }
}

// Functions

fn print_hello_world() {
    println!("Hello World!");
}

// Arguments:
fn print_two_numbers(x: i32, y: i32) {
    println!("Numbers chosen are {x} and {y}");
}

// defining return types
fn sum_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}

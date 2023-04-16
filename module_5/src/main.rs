// Basic Struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Struct with no named fields to store different types and values

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn structs() {
    // Working with Structs

    let xico = User {
        active: true,
        username: String::from("fcaramez"),
        email: String::from("fcaramez@gmail.com"),
        sign_in_count: 4,
    };

    // Mutable structs

    let mut tomas = User {
        active: false,
        username: String::from("Tommy"),
        email: String::from("test@gmail.com"),
        sign_in_count: 1,
    };

    tomas.email = String::from("tomas@gmail.com");

    // Using spread in Structs

    let alex = User {
        email: String::from("alex@gmail.com"),
        username: String::from("Alex"),
        ..xico
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", alex.sign_in_count);

    println!("Hello, world!");
}

fn create_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Exercise to calculate area of a rectangle

// 1st way
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    )
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// 2nd way (with tuples)

fn run() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        calculate_area(rect1)
    );
}

fn calculate_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// 3rd way, refactoring with structs

struct Rectangle {
    width: u32,
    height: u32,
}

fn run_two() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        calculate_area_two(&rect1)
    );
}

fn calculate_area_two(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Defining a method

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn run_again() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

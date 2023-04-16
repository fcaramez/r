fn main() {
    // Ownership Rules

    /* Each value in Rust has an owner
    There can only be one owner at the time
    When the owner goes out of scope, the value is dropped */

    // Variable scope

    {
        // a does not exist yet
        // Different scope, different owner and different value
        let a = "Good day"; // A is declared and assigned, value is accessible
    } // here a is out of scope and value is lost / dropped

    // String Type

    let mut s = String::from("Hello");

    // push_str appends the parameter to the String, it also mutates it

    s.push_str(", World!");

    println!("{s}");

    // References and Borrowing

    // Here, S1 enters the scope
    let s1 = String::from("hello");

    // Here we are sending a reference of that value to be borrowed to the function
    let len = calculate_len(&s1);

    println!("The length of {s1} is {len}");

    // Mutable Reference

    // same as before, here the value enters the main scope
    let mut s = String::from("hello");

    // Here we send a refence of that mutable value to be borrowed to the function
    change(&mut s);

    // This will throw an error as you can only borrow one mutable reference at a time
    /* let r1 = &mut s;
       let r2 = &mut s;
    */

    /* At any given time, you can have either one mutable reference or any number of immutable references.
    References must always be valid */

    // The slice type

    // A String slice is a reference to a part of a String

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let world = &s[6..];

    println!("World: {world}");

    let first_word = first_word(&s);

    println!("{first_word}");

    // Slice also works for arrays

    let arr = [1, 2, 3, 4, 5, 6];

    let slice = &arr[1..3];

    assert_eq!(slice, &[2, 3]);
}

// Get the first word
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn calculate_len(s: &String) -> usize {
    // Here we get the borrowed reference and we do our changes
    s.len()
} // here the value goes out of scope again back to the main function ready to be used somewhere else

fn change(some_string: &mut String) {
    some_string.push_str(", World")
}

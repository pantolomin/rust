fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1; // s1 is moved to s2, and s1 is no longer valid
    // println!("{}", s1); // This will cause a compile-time error because s1 is no longer valid
    let s3 = s2.clone(); // s3 is a clone of s2, and both s2 and s3 are valid

    s2 = String::from("ahoy"); // s2 is now a new String, and the previous String is dropped
    s2.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s2); // This will print 'hello, world!'
    println!("s2 = {}, s3 = {}", s2, s3); // This will print 's2 = ahoy, world!, s3 = hello'

    let x = 5;
    let y = x; // x is copied to y, and both x and y are valid
    println!("x = {}, y = {}", x, y); // This will print 'x = 5, y = 5'

    takes_ownership(s2); // s2 is moved to the function, and s2 is no longer valid
    // println!("{}", s2); // This will cause a compile-time error because s2 is no longer valid

    makes_copy(x); // x is copied to the function, and x is still valid
    println!("x = {}", x); // This will print 'x = 5'

    let obtained_string = gives_ownership(); // gives_ownership moves its return value to obtained_string
    println!("obtained_string = {}", obtained_string);
    let obtained_string = takes_and_gives_back(obtained_string); // obtained_string is moved to the function, and obtained_string is no longer valid
    println!("obtained_string = {}", obtained_string);

    // Returning Multiple Values with Tuples - tedious and error-prone
    let s4 = String::from("same_pattern_with_tuple");
    let (s4, len) = calculate_length(s4); // s4 is moved to calculate_length, and s5 is the returned String
    println!("The length of '{}' is {}", s4, len);

    // Returning Multiple Values with References - more efficient and less error-prone
    let s5 = String::from("same_pattern_with_reference");
    let len = calculate_length_with_reference(&s5); // s5 is borrowed by calculate_length_with_reference, and s5 is still valid
    println!("The length of '{}' is {}", s5, len);

    let mut s6 = String::from("hello");
    concatenate_strings(&mut s6, ", world!"); // s6 is borrowed mutably by concatenate_strings, and s6 is still valid
    println!("{}", s6); // This will print 'hello, world!'

    let list = vec![1, 2, 3];
    let list_clone = list.clone(); // list_clone is a clone of list, and both list and list_clone are valid
    let slice = &list[0..2]; // slice is a reference to a portion of list, and list is still valid
    println!("list = {:?}, list_clone = {:?}, slice = {:?}", list, list_clone, slice); // This will print 'list = [1, 2, 3], list_clone = [1, 2, 3], slice = [1, 2]'

    println!("First word in '{}' is '{}'", s6, first_word(&s6)); // This will print 'First word in 'hello, world!' is 'hello,'
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and is dropped

fn makes_copy(some_integer: i32) {
    println!("some_integer = {some_integer}");
} // some_integer goes out of scope, but nothing special happens because it's a copy

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    println!("give: {some_string}");
    some_string // some_string is returned and moves out to the caller
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("took: {a_string} ... then give it back");
    a_string // a_string is returned and moves out to the caller
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length) // return the String and its length as a tuple
}

fn calculate_length_with_reference(s: &String) -> usize {
    s.len() // len() returns the length of a String
} // s goes out of scope, but because it does not have ownership of what it refers to, nothing happens

fn concatenate_strings(s1: &mut String, s2: &str) {
    s1.push_str(s2); // push_str() appends a literal to a String
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // Convert the String to an array of bytes
    for (i, &item) in bytes.iter().enumerate() { // Iterate over the bytes with their indices
        if item == b' ' { // Check if the byte is a space character
            return &s[0..i]; // Return the substring up to the first space
        }
    }
    &s[..] // If no space is found, return the whole string
}
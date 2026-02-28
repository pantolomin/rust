// Constants are always immutable and must have a type annotation. They can be declared in any scope, including the global scope.
const _THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

fn main() {
    // Mutable variables can be changed after they are declared. To make a variable mutable, we use the mut keyword.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Shadowing allows us to declare a new variable with the same name as a previous variable.
    // The new variable shadows the previous variable, and we can use the same name for different variables in different scopes.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);

    // Need to annotate the type of the variable when we want to parse a string into a number.
    // The parse method returns a Result type, which is an enum that can be either Ok or Err.
    // We can use the expect method to handle the error case and provide a custom error message.
    let _guess: u32 = "42".parse().expect("Not a number!");

    // Default type for integer literals is i32
    let _x: u32 = 15_674;
    let _y: i64 = -15_677_977_135_496_187;
    let _char: u8 = b'A';
    let _hex: i16 = 0x7FFF;
    // Default type for floating-point numbers is f64
    let _small_decimal: f32 = 2.5;
    let _large_decimal: f64 = 2.5e10;

    // Character literals are specified with single quotes and can be any Unicode scalar value, including letters, numbers, emojis, and more.
    let _z: char = 'ℤ';
    let _real: char = 'ℝ';
    let _heart_eyed_cat: char = '😻';

    // Tuples are a way to group together a fixed number of values of different types into a single compound type.
    // They are created using parentheses and can contain any number of values.
    let coordinates: (f64, f64) = (3.0, 5.0);
    println!("The coordinates are ({}, {})", coordinates.0, coordinates.1);
    // We can also destructure a tuple into individual variables using pattern matching.
    let (x, y) = coordinates;
    println!("The coordinates are ({}, {})", x, y);
    let _unit: () = (); // The unit type is a special type that has only one value, which is also written as ().

    let _months = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December",
    ];
    let _zeroes = [0; 5]; // This creates an array of five elements, all initialized to 0.
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    numbers[3] = 10;
    println!("The numbers: {:?}", numbers);
}

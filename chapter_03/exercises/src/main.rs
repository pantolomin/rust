fn main() {
    println!("100°F: {}°C", to_celsius(100.0));
    println!("20°C: {}°F", to_fahrenheit(20.0));
    println!("fibonacci(6): {}", fibonacci(6));
    println!("fibonacci(10): {}", fibonacci(10));
    christmas_carol();
}

fn to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn christmas_carol() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese-a-Laying",
        "seven Swans-a-Swimming",
        "eight Maids-a-Milking",
        "nine Ladies Dancing",
        "ten Lords-a-Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming"
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me:", days[i]);
        for j in (0..=i).rev() {
            if j == 0 && i > 0 {
                print!("and ");
            }
            if j > 1 {
                println!("{},", gifts[j]);
            } else {
                println!("{}", gifts[j]);
            }
        }
        println!();
    }
}

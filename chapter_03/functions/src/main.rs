fn main() {
    let choice = false;
    let value = {
        let tmp = if choice { five() } else { 10 };
        plus_one(tmp)
    };
    let mut counter = 0;
    let ten = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
        println!("Ten = {ten}");
    // A simple comment
    print_measurement(value, 'h');

    let mut count = 0;
    'first_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'first_loop;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut count = 3;
    while count > 0 {
        println!("count = {count}");
        count -= 1;
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn print_measurement(value: i32, unit_label: char) {
    if unit_label == 'h' {
        println!("The measurement is: {value} hours");
    } else if unit_label == 'm' {
        println!("The measurement is: {value} meters");
    } else {
        println!("The measurement is: {value}{unit_label}");
    }
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
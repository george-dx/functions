fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(number: i32) -> i32 {
    number + 1
}

fn labeled_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
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
}

fn for_loop(){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
}

fn while_loop(){
    let mut number = 3;

    while number != 0 {
        println!{"{number}!"};

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn nice_for_rev_loop() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main() {
    print_labeled_measurement(5, 'h');
    
    let number = five();
    println!("The value of number is: {number}");

    let six = plus_one(5);
    println!("The value of six is {six}");

    if number < six {
        println!("The number is smaller than {six}");
    } else if number == six {
        println!("The number is equal to {six}");
    } else {
        println!("The number is greater than {six}");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {result}");
    
    labeled_loop();

    while_loop();

    for_loop();

    nice_for_rev_loop();
}

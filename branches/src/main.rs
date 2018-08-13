fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
        // "six" here would cause a compile error as the type of number would be undetermined
    };

    // if-else
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if-else if-else
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // loop
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    // iterating over an array with an index
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value at index {} is: {}", index, a[index]);

        index += 1;
    }

    // iterating over an array with an iterator
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value at index: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}

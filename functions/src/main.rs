fn main() {
    println!("Hello, world!");

    another_function();
    yet_another_function(123, 123.4);

    let x = five();
    let x = plus_one(x);
    let heart_eyed_cat = '😻';
    let y = {
        let x = 3; // scoping
        x + 1 // expression
              //x + 1; semi-colon here throws a compiler error as block would end with statement
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("Program Complete {}", heart_eyed_cat);
}

fn five() -> u8 {
    5
    // 255 would cause a panic here in debug mode, it would silently overflow in release mode
}

fn plus_one(x: u8) -> u8 {
    x + 1
}

fn another_function() {
    println!("Another function.");
}

fn yet_another_function(a: i32, b: f32) {
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
}

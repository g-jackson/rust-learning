fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 2,
    };
    println!("user1: {}", user1.username);
    user1.email = String::from("someone@changedemail.com");
    println!("{}", user1.email);

    let user2 = build_user(String::from("test@test.com"), String::from("test2"));
    println!("{}", user2.active);

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("{}", user3.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!(
        "The origin is at ({}, {}, {}) - Black's rgb is {:?}",
        origin.0, origin.1, origin.2, black
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle {:#?} is {} square pixels. Thats {} whole pixels!",
        rect1,
        area(&rect1),
        rect1.area()
    );

    let sq1 = Rectangle::square(4);
    println!(
        "The area of square of width {} is {}",
        sq1.width,
        sq1.area()
    );
}

#[derive(Debug)]
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

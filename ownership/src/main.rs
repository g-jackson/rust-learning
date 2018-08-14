#[allow(unused_variables)] // turn off compiler warnings for unused variables

fn main() {
    let s = "hello"; // string literal from stack
    let mut t = String::from("hello"); // string object from heap
    t.push_str(", world!");
    println!("{}", t);

    let hello = &t[..5];
    let world = &t[6..];
    println!("{}{}", hello, world);

    let num1 = 1;
    let num2 = num1;

    let str1 = String::from("test");
    let mut str2 = str1;
    // println!("{}", str1); does not compile
    // str1 is no longer owner of the referenced string
    str2.push_str(" still referenced");
    println!("{}", str2);

    let str1 = String::from("test1");
    let str1 = takes_and_gives_back(str1);
    let mut str2 = str1.clone();
    str2.push_str("test3");
    change(&mut str2);
    println!("{}{}", str1, str2);

    let len = calculate_length(&str2);
    println!("The length of '{}' is {}.", str2, len);

}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    let b = some_string;
    b.push_str(", world");
}

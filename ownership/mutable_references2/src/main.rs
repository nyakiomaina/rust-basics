fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{} and {}", r1, r2);

    let r3 = change(&mut s);
    println!("{}", r3);
}

fn change(some_string:&mut String) -> String {
    some_string.push_str(", world");
    some_string.clone()
}

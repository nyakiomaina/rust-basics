fn main() {
    let _s1 = gives_ownership(); // gives_ownership moves it's return 
                                // value to s1
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2); //s2 moves into take_and_gives_back, which also moves
                                       //it's return value in to s3
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

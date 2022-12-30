fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function
                        // so nolonger valid here
   // println!("{}", s);

    let x = 5; // x comes in to scope

    makes_copy(x); // x would move into function
                   // but i32 is copy, so it's okay to still use x afterwards
}

 fn takes_ownership(some_string: String) { // some_strings comes in to scope
    println!("{}", some_string);
} // some_string goes out of scope
  // drop is called
  // the backing memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes in to scope
    println!("{}", some_integer);
} //some_interger goes out of scope

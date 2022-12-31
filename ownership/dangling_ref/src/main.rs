fn main() {
    let reference_to_nothing = dangle();
}
fn dangle() -> &String { // dangle()return the reference to the String
    let s = String::from("hello"); // s is the new String

    &s // return a reference to the String, s

} // s goes out of scope, and is dropped.
// It's memory goes away


fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // rust now considers s2 as invalid thus doesn't need to free anything 
                 // when s1 goes out of scope
}

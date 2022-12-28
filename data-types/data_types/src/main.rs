fn main() {
    //bool
    let t: bool = true;
    let f: bool  = false;
    println!("t = {}", t);
    println!("f = {}", f);

    //char
    let a: char = 'a';
    let special_character: char = "@";
    println!("a = {}", a);
    printlnn!("special character = {}", special_character);

    //integers
    let x = 5;
    let y: u64 = 293402358; // 64-bit unsigned interger (explicit declaration)
    let z: u32 = -200 // unsigned integer will return an error(only stores positive values)
     //signed interger types
     let m: i32 = -200;
     let n: i32 = 200;
     println!("m = {}", m);
     println!("m - {}", n);

    // floating point numbers
    let pi = 3.14159265358; // f64 by default

    //arrays
    let fruits: [&str; 4] = ["banana", "strawberry", "mango", "watermelon"];
    println!("The first elemet of the array is: {}", fruits[0]);

    let mut counter = 0;
    for elem in fruits.iter(){
        println!("the element of index {} is {}", counter, elem);
        counter += 1;
    }

    //slices
    let slice = &fruits[0..2]; //upper bound is exclusice so this will select the first 2 elements of the fruits array
    
    for elem in slice {
        println!("elem is {}", elem);
    }

    //str
    let str_slice = "Hello. I am str";

    println!("The value of the outer str is: {}", str_slice);

    //tuple
    let tuple = ("hello", 42, "world", [3,6,9]);

    println!("The first element is {}", tuple.0);
    println!("second element is {}", tuple.1);
    println!("third element is {}", tuple.2);
    let mut counter = 0;
    for elem in &tuple.3 {
        println!("Element {} of the fourth element {}", counter, elem);
        counter += 1;
    }


}

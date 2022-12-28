// functions with return values

fn five() -> i32 {
    5 // function return value and it's return type is i32
      //
      // function has no parameter
      // function body has only 5 with NO SEMI COLON
      // because it's an EXPRESSION WHOSE VALUE WE WANT TO RETURN
}

fn main() {
    let x = five(); // we are using the RETURN VALUE of a funtion to INITIALIZE A VARIABLE
                    // LINE SAME AS let x = 5;

    println!("The value of x is: {x}");
}

// there are no function calls, macros or eben let statements in the five function-- just the 5 by
// itself
//
// function return type is specified as i32



// example 2
//

fn main() {

    let x = print_one(5);

    println!("The value of x is: {x}");

}

fn plus_one(x: i32) -> i32 {

    x + 1 // no  semi colon at the end of this line
          // meaning its an expression
          // adding an expression would make it a statement and we'll get an error
}

# ownership of return values
The ownership of a variable follows the same pattern every time: 
assigning a value to another variable moves it. 
When a variable that includes data on the heap goes out of scope, 
the value will be cleaned up by drop UNLESS ownership of the data has been moved to another variable.

taking ownership and then returning ownership with every function is a bit tedious. 
What if we want to let a function use a value but not take ownership? 
It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, 
in addition to any data resulting from the body of the function that we might want to return as well.

REFERENCE ==> let a function use a value but not take ownership.

# dangling pointer
## What the hell is it? 
pointer that referes to the location in memory that may have been given to someone else.(by 
freeing some memory while preserving a pointer to that memory)

dangling references to see how rust prevents them wth compile-time error

In rust, compiler guarantees that references will never be dangling references,

if you have a reference to some data, the compiler will ensure the data will not go out of scope
before thr refence to the data does.


In the code, `s` is created inside `dangle`, when the code of `dangle` is finished, `s` will be deallocated but 
we tried  to return a reference to it. that means this reference would be pointing to an invalid `String`

To fix the issue, return the `String` instead of reference to it

```rust
fn main() {
    let s = dangle();
    println!("{}", s);

}
fn dangle(s: &String) -> String {
    let s = String::from("hello");
    s
}
```

// fn main() {
//     let wight1 = 30;
//     let height1 = 50;

//     println!("The area of the rectangle is {} square pixels.", area(wight1, height1));
// }

// fn area(width: u32, height: u32) -> u32{
//     height * width
// }


// fn main() {
//     let rect1 = (30, 50);
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32{
//     dimensions.0 * dimensions.1
// }


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height:u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "rect is {:#?}",rect1);
}
// fuction accesses the width and the height of the rectacle instance
// ACCESSING FIELDS OF A BORROE STRUCT INSTANCE DOES NOT MOVE THE FIELD VALUE
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
fn main() {
    let mut s = String::from("Hi Nyakio");
   let index = string_of_words(&s);

    println!("The index of the first space in '{}' is {}", s, index);
    //println!("The second word in '{}' is {}", s, second_word);

    s.clear(); // empties the string making it equal to ""
}

fn string_of_words(s: &String) -> usize {
    let _bytes = s.bytes();

    for(i, item) in s.bytes().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()

// fn string_of_words(s:&String) -> usize {
//     for(i,ch) in s.char_indices() {
//         if ch == ' ' {
//             return i;
//         }
//     }
//     s.len()
// }
}


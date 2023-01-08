use std::collections::HashMap;
use std::io;

struct Student{
    name:String,
    age:u8,
    grades:HashMap<String,u8>,
}

impl Student {
    fn new(name:&str, age:u8) -> Self{
        Self{
            name: name.to_string(),
            age,
            grades: HashMap::new(),
        }
    }
    fn add_grades(&mut self, subject: &str, grade: u8) {
        self.grades.insert(subject.to_string(), grade);
    }
    fn average_grade(&self)-> f32{
        // let total: u32 = self.grades.values().map(|x| *x as u32).sum();
        let total: u32 = self.grades.values().sum::<u8>() as u32;
        let count:u32 = self.grades.len() as u32;
        total as f32 / count as f32

    }
}

fn read_string_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn read_integer_input() -> u8 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid integer")
}

fn main() {
    let mut students = Vec::new();
    for i in 0..2{
        println!("Enter information for students {}:", i);
        println!("Name:");
        let name = read_string_input();
        println!("Age:");
        let age = read_integer_input();
        println!("Grade in Math: ");
        let math_grade = read_integer_input();
        println!("Grade in English: ");
        let english_grade = read_integer_input();
        println!("Grade in Physics");
        let physics_grade = read_integer_input();

        let mut s = Student::new(&name, age);
        s.add_grades("Math", math_grade);
        s.add_grades("English", english_grade);
        s.add_grades("Physics", physics_grade);
        students.push(s);
    }
    // loop {
    //     println!("Enter student information (name, age, grade in Math, grade in English, grade in Physics):");

    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).expect("Failed to read line");
    //     let input = input.trim();
    //     if input == "done" {
    //         break;
    //     }

    //     let mut parts = input.split_whitespace();
    //     let name = parts.next().expect("Expected student name");
    //     let age: u8 = parts.next().expect("Expected student age").parse().expect("Invalid age");
    //     let math_grade = parts.next().expect("Expected grade in Math").parse().expect("Invalid grade");
    //     let english_grade = parts.next().expect("Expected grade in English").parse().expect("Invalid grade");
    //     let physics_grade = parts.next().expect("Expected grade in Physics").parse().expect("Invalid grade");

    //     let mut s = Student::new(name, age);
    //     s.add_grades("Math", math_grade);
    //     s.add_grades("English", english_grade);
    //     s.add_grades("physics", physics_grade);
    //     students.push(s);

    // }

    let total_average: f32= students.iter().map(|s| s.average_grade()).sum();
    let student_count = students.len() as f32;
    println!("Average grade of all students is {}", total_average / student_count);
}
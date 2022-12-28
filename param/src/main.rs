fn main() {

    print_label_measurement(5, 'x')

}

fn print_label_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

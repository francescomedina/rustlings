// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

//UTILE

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data.clone());

    string_uppercase(&data);

    // let mut data1 = "Rust is great!".to_string();
    //
    // get_char(data1.clone());
    //
    // string_uppercase(&mut data1);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) {
    &data.to_uppercase();

    println!("{}", data);
}

//-------------------------VARIANT

// // Should not take ownership
// fn get_char(data: String) -> char {
//     data.chars().last().unwrap()
// }
//
// // Should take ownership
// fn string_uppercase(data: &mut String) {
//     &data.to_uppercase();
//
//     println!("{}", data);
// }

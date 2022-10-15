// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {
    let data = "Rust is great!".to_string();
    
    let last_char = get_char(&data);

    assert_eq!(last_char.to_string(),"!".to_string());

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    print!("{}",data.chars().last().unwrap());
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}

pub fn match_example(number: i32) {
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 | 4 | 5 => println!("Three or four or five"),
        6..=8 => println!("Six or seven or eight"),
        _ => println!("Unknown"),
    }
}

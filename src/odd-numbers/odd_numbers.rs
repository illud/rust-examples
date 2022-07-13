pub fn odd_numbers(from: i32, to: i32){
    for i in from..to{
        if i % 2 != 0 {
            println!("{}", i);
        }
    }
}

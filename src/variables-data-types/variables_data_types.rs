pub fn variables_data_types() {
    let company_string = "TutorialsPoint"; // string type
    let rating_float = 4.5; // float type
    let is_growing_boolean = true; // boolean type
    let icon_char = '♥'; //unicode character type
    println!("company name is:{}", company_string);
    println!("company rating on 5 is:{}", rating_float);
    println!("company is growing :{}", is_growing_boolean);
    println!("company icon is:{}", icon_char);

    let result = 10; // i32 by default
    let age: u32 = 20;
    let sum: i32 = 5 - 15;
    let mark: isize = 10;
    let count: usize = 30;
    println!("result value is {}", result);
    println!("sum is {} and age is {}", sum, age);
    println!("mark is {} and count is {}", mark, count);

    let age: u8 = 255;
    println!("age is {} ", age);

    let result = 10.00; //f64 by default
    let interest: f32 = 8.35;
    let cost: f64 = 15000.600; //double precision
    println!("result value is {}", result);
    println!("interest is {}", interest);
    println!("cost is {}", cost);

    let special_character = '@'; //default
    let alphabet: char = 'A';
    let emoji: char = '😁';
    println!("special character is {}", special_character);
    println!("alphabet is {}", alphabet);
    println!("emoji is {}", emoji);
}

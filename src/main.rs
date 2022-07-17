#[path = "./push-struct-to-vector/push_struct_to_vector.rs"]
mod push_struct_to_vector;

#[path = "./structs/structs.rs"]
mod structs;

#[path = "./prime-number/prime_number.rs"]
mod prime_number;

#[path = "./even-numbers/even_numbers.rs"]
mod even_numbers;

#[path = "./odd-numbers/odd_numbers.rs"]
mod odd_numbers;

#[path = "./calculate-area-of-rectangle/calculate_area_of_rectangle.rs"]
mod calculate_area_of_rectangle;

#[path = "./impl-example/impl_example.rs"]
mod impl_example;

#[path = "./variables/variables.rs"]
mod variables;

#[path = "./variables-data-types/variables_data_types.rs"]
mod variables_data_types;

#[path = "./test-example/add.rs"]
mod add;

#[path = "./match-example/match_example.rs"]
mod match_example;

fn main() {
    push_struct_to_vector::push_struct_to_vector();

    structs::structs();

    prime_number::prime_number(5);

    even_numbers::even_numbers(1, 20);

    odd_numbers::odd_numbers(1, 20);

    calculate_area_of_rectangle::calculate_area_of_rectangle(3, 2);

    impl_example::impl_example();

    variables::variables();

    variables_data_types::variables_data_types();

    add::add(1, 2);

    match_example::match_example(2);
}

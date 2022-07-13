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

fn main() {
    push_struct_to_vector::push_struct_to_vector();

    structs::structs();

    prime_number::prime_number(5);

    even_numbers::even_numbers(1, 20);

    odd_numbers::odd_numbers(1, 20);
}

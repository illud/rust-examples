#[path = "./push-struct-to-vector/push-struct-to-vector.rs"]
mod push_struct_to_vector;

#[path = "./structs/structs.rs"]
mod structs;

fn main() {
    push_struct_to_vector::push_struct_to_vector();

    structs::structs();
}

#[derive(Debug)]
struct Cat{
    name: String,
    age: i8,
    race: String
}

pub fn structs(){
    let cat_one = Cat{
        name: String::from("Eskapi"),
        age: 6,
        race: String::from("Maine Coon")
    };

    println!("{:#?}", cat_one);
}
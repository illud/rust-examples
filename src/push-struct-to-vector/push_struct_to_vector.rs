#[derive(Debug)]
struct Cat {
    name: String,
    age: i8,
    race: String,
}

pub fn push_struct_to_vector() {
    let mut cats = vec![];

    cats.push(Cat {
        name: String::from("Eskapi"),
        age: 6,
        race: String::from("Maine Coon"),
    });

    cats.push(Cat {
        name: String::from("Saturn"),
        age: 4,
        race: String::from("Mau"),
    });

    println!("{:#?}", cats);
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn print_rectangle(&self) {
        println!("Rectangle {} x {}", self.width, self.height);
    }
}

pub fn impl_example() {
    let rect = Rectangle {
        width: 3,
        height: 2,
    };

    rect.print_rectangle();
}

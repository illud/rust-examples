// Enum for mathematical operations
#[derive(PartialEq)]
enum Operators {
    Plus,
    Minus,
}

struct Operation {
    operator: Operators,
    y: u64,
    r: u64,
}

impl Operation {
    // Resolves - or + to find value of x, basic algebra operation
    fn value_of_x(&self) -> u64 {
        if self.operator == Operators::Plus {
            return self.r - self.y;
        }

        if self.operator == Operators::Minus {
            return self.r + self.y;
        } else {
            return 0;
        }
    }
}

pub fn anum_algebra() -> u64 {
    let result = Operation {
        operator: Operators::Minus,
        y: 23,
        r: 30,
    };

    result.value_of_x()
}

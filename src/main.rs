fn main() {
    // let file = std::fs::read_to_string("text").unwrap();
    // file.lines()
    //     .enumerate()
    //     .filter(|(idx, _)| idx & 2 == 0)
    //     .for_each(|(_, lines)| println!("{}", lines));
    // let c = Colors::Yellow;
    // match c {
    //     Colors::Red => println!("red"),
    //     Colors::Blue => println!("blue"),
    //     Colors::Green => println!("green"),
    //     Colors::Yellow => println!("Yellow"),
    // }
    let foo = Colors::Green;
    foo.is_green();
}

enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Colors {
    fn is_green(&self) -> bool {
        if let Colors::Green = self {
            return true;
        }
        return false;
    }
    fn is_green_parts(&self) -> bool {
        match self {
            Colors::Red => return false,
            Colors::Blue => return true,
            Colors::Green => return false,
            Colors::Yellow => return true,
        }
    }
}

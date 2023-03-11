fn multiply(num: Option<usize>) -> Option<usize> {
    return Some(num? * 5);
}

fn main() {
    println!("Okay");
    multiply(Ok(()))
}

// fn main() {
//     // let file = std::fs::read_to_string("text").unwrap();
//     // file.lines()
//     //     .enumerate()
//     //     .filter(|(idx, _)| idx & 2 == 0)
//     //     .for_each(|(_, lines)| println!("{}", lines));
//     // let c = Colors::Yellow;
//     // match c {
//     //     Colors::Red => println!("red"),
//     //     Colors::Blue => println!("blue"),
//     //     Colors::Green => println!("green"),
//     //     Colors::Yellow => println!("Yellow"),
//     // }
//     let foo = Colors::Green;
//     foo.is_green();
// }
//
// enum Colors {
//     Red,
//     Green,
//     Blue,
//     Yellow,
// }
//
// impl Colors {
//     fn is_green(&self) -> bool {
//         if let Colors::Green = self {
//             return true;
//         }
//         return false;
//     }
//     fn is_green_parts(&self) -> bool {
//         match self {
//             Colors::Red => return false,
//             Colors::Blue => return true,
//             Colors::Green => return false,
//             Colors::Yellow => return true,
//         }
//     }
// }

// struct Custom {
//     age: usize,
//     name: String,
// }
//
// enum Item {
//     Number(usize),
//     String(String),
//     MyCustom(Custom),
// }
//
// fn append(items: &mut Vec<Item>) {
//     items.push(Item::String("hello, nice".to_string()))
// }
//
// fn main() {
//     let foo = Item::Number(5);
// }
// oooooooo

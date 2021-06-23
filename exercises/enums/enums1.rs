// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!
#[derive(Debug)]
enum Message {
   Quit(String),
   Echo(String),
   Move(String),
   ChangeColor(String), // TODO: define a few types of messages as used below
}

fn main() {
    let home = Message::Quit(String::from("EOC"));
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}

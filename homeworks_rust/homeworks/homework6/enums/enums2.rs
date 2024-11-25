// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

#[derive(Debug)]
enum Message {
    Move{x: i32, y: i32},
    Echo{x: String},
    ChangeColor{r: i32, g: i32, b: i32},
    Quit,


}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo{x: String::from("hello world")},
        Message::ChangeColor{r: 200, g: 255, b: 255},
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

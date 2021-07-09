fn main() {
    println!("Hello, world!");
    let (r, g, b): (i32, i32, i32) = (255, 235, 100);

    //let msg: Message = Message::ChangeColor(r, g, b);
    let msg: Message = Message::Write("Use Message Write".to_string());

    match msg {
        Message::Quit() => quit(),
        Message::ChangeColor(r, g, b) => change_color(r, g, b),
        Message::Move { x: x, y: y } => move_cursor(x, y),
        Message::Write(s) => println!("{}", s),
    }
}

enum Message {
    Quit(),
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn quit() {
    println!("{}", "I am quit");
}
fn change_color(r: i32, g: i32, b: i32) {
    println!("r:{}, g:{}, b:{}", r, g, b);
}
fn move_cursor(x: i32, y: i32) {
    println!("x:{},y:{}", x, y);
}

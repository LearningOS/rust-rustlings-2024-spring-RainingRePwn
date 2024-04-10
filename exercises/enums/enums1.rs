// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo); // 这里Echo不是一个有效的变体，需要更正
    println!("{:?}", Message::Move { x: 10, y: 20 }); // Move是一个关联了数据的变体，需要提供数据
    println!("{:?}", Message::ChangeColor(255, 0, 0)); // ChangeColor是一个关联了数据的变体，需要提供数据
}

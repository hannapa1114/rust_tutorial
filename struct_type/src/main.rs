struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle{width: 50, height: 30};
    println!("rect1 is {}", rect1.width * rect1.height);
}


// fn main() {
//     let rect1 = Rectangle{width: 50, height: 30};
//     println!("rect1 is {}", area(&rect1));
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

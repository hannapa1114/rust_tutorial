fn main() {
    // another_function(5, 6);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    let z = five();

    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    // 세미콜론을 쓰지 않으면 반환값
    5
}
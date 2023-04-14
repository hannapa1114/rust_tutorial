use std::io;

fn main() {
    loop {
    println!("온도를 입력하세요:");

    let mut temp = String::new();

    
    io::stdin()
    .read_line(&mut temp)
    .expect("입력한 값을 읽지 못했습니다.");

    let temp: f64 = temp.trim().parse().expect("숫자를 입력하세요.");
    println!("섭씨면 1번 화씨면 2번 입력하세요:");

    let mut num_str = String::new();

    io::stdin()
    .read_line(&mut num_str)
    .expect("입력한 값을 읽지 못했습니다.");

    let num: i32 = num_str.trim().parse().expect("숫자를 입력하세요");

    if num == 1 {
        let fahrenheit = (temp * 1.8) + 32.0;

        let fahrenheit_format = format!("{:.1}", fahrenheit);

        println!("{}도 섭씨는 {}도 화씨입니다.", temp, fahrenheit_format);
    } 
    
    if num == 2 {
        let celsius = (temp - 32.0) * 0.5556;

        let celsius_format = format!("{:.1}", celsius);
        
        println!("{}도 화씨는 {}도 섭씨입니다.", temp, celsius_format);
        
    }
}


}

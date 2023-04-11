fn main() {
    // 정수형 부호 i8, i16, i32, i64 미부호 u8, u16, u32, u64
    // 정수형 리터럴 Decima 98_222, Hex	0xff, Octal	0o77, Binary 0b1111_0000, Byte (u8 only) b'A'
    
    // 부동소수점 타입
    let _x = 2.0; // f64, 기본 타입은 f64인데, 그 이유는 최신의 CPU 상에서는 f64가 f32와 대략 비슷한 속도를 내면서도 더 정밀한 표현이 가능하기 때문

    let _y: f32 = 3.0; // f32

       // addition
       let sum = 5 + 10;

       // subtraction
       let difference = 95.5 - 4.3;
   
       // multiplication
       let product = 4 * 30;
   
       // division
       let quotient = 56.7 / 32.2;
   
       // remainder
       let remainder = 43 % 5;

       println!("sum {} diff {} multi {} divi {} remain {} ", sum, difference, product, quotient, remainder);

    // boolean

    let t = true;

    let f: bool = false;

    println!("{} {}", t, f);

    // string char '' string ""

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{} {} {}", c, z, heart_eyed_cat);

    // 복합 타입들

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let (x, y, z) = tup;

    // println!("The value of x is: {}", x);
    // println!("The value of y is: {}", y);
    // println!("The value of z is: {}", z);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("{} {} {}",five_hundred, six_point_four, one);

    // 배열

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    // let index = 10;
    // let element = a[index]; panic - 배열보다 색인이 길 때

    // println!("The value of element is: {}", element);

}

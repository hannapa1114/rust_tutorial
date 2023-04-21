### move 소유권 이동 (heap)

-

```
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

- 소유권이 s1에서 s2로 이동되면 s1을 메모리에서 무효화시키고 s2만 유효하게 된다

```
error[E0382]: use of moved value: `s1`
 --> src/main.rs:4:27
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`,
which does not implement the `Copy` trait

```

#### copy 가능한 types (stack)

- u32와 같은 모든 정수형 타입들
- true와 false값을 갖는 부울린 타입 bool
- f64와 같은 모든 부동 소수점 타입들
- Copy가 가능한 타입만으로 구성된 튜플들. (i32, i32)는 Copy가 되지만, (i32, String)은 안됩니다.

```
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

### 참조자(&)

- 소유권을 이전하지 않고 참조할 수 있게 해줌

```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s는 String의 참조자입니다
    s.len()
} // 여기서 s는 스코프 밖으로 벗어났습니다. 하지만 가리키고 있는 값에 대한 소유권이 없기
  // 때문에, 아무런 일도 발생하지 않습니다.
```

- 가변참조자
  - 변수가 기본적으로 불변인 것처럼, 참조자도 마찬가지입니다. 우리가 참조하는 어떤 것을 변경하는 것은 허용되지 않습니다.

```
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

error: cannot borrow immutable borrowed content `*some_string` as mutable
 --> error.rs:8:5
  |
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^
```

- 가변 참조자를 사용하면 오류를 고칠 수 있다.
- 특정한 스코프 내에 특정한 데이터 조각에 대한 가변 참조자를 딱 하나만 만들 수 있다.

```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

- 불변참조자와 가변참조자는 함께 사용할 수 없다.

```
let mut s = String::from("hello");

let r1 = &s; // 문제 없음
let r2 = &s; // 문제 없음
let r3 = &mut s; // 큰 문제
```

### 스트링 슬라이스

- 소유권을 갖지 않는 데이터 타입
- 스트링 슬라이스는 String 일부에 대한 참조자(불변참조자)
- start..end 문법은 start부터 시작하여 end를 포함하지 않는 연속된 범위
- 스트링 슬라이스를 나타내는 타입은 &str

```
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

```
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

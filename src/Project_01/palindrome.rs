use crate::Tools::reader;

fn main() {
    let input_str = reader::read::<i32>(Some("정수 숫자를 입력해주세요."));

    let input_num: i32 = input_str.parse().unwrap();

    let mut original = input_num;
    let mut reversed = 0;

    while original > 0 {
        let remainder = original % 10;

        reversed = (reversed * 10) + remainder;

        original /= 10;
    }

    if input_num == reversed {

        println!("입력: {}", input_num);
        println!("출력:"); // what is this?
        println!("뒤집은 숫자: {}", reversed);
        println!("팰린드롬 여부: YES");
    } else {
        println!("입력: {}", input_num);
        println!("출력:"); // what is this?
        println!("뒤집은 숫자: {}", reversed);
        println!("팰린드롬 여부: NO");
    }
}
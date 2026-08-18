use crate::Tools::reader;

mod Tools;

fn main() {
    let array = reader::read::<String>(Some("목표치를 입력해주세요"));
    let input = reader::read::<String>(Some("숫자를 입력해주세요"));

    let mut pre_num: i32 = 0;
    let mut cur_len: i32 = 1;
    let mut max_len: i32 = 0;
    let mut isFirst: bool = true;

    for word in &input.split_whitespace().collect::<Vec<&str>>() {
        let parsed = word.parse::<i32>().unwrap();

        if isFirst {
            pre_num = parsed;
            isFirst = false;
        }
        else {
            if parsed == pre_num + 1 {
                cur_len += 1;
            }
            else {
                cur_len = 1;
            }

        }

        if max_len < cur_len {
            max_len = cur_len;
        }

        pre_num = parsed;
    }

    println!("출력: ");
    println!("가장 긴 길이: {}", max_len)
}
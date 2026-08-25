use rand::{rng, RngExt};
use crate::Tools::reader;

fn main() {
    let random_num = randomize();
    let mut try_num = 0;

    loop {
        let input = reader::read::<i32>(None);

        let nums: Vec<i32> = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        if nums.len() != 3 {
            println!("세 자리 숫자를 입력해주세요.");
            continue;
        }

        if nums[0] == 0 {
            println!("첫 번째 숫자는 0이 될 수 없습니다.");
            continue;
        }

        if nums[0] == nums[1]
            || nums[0] == nums[2]
            || nums[1] == nums[2]
        {
            println!("각 숫자는 서로 달라야 합니다.");
            continue;
        }

        try_num += 1;

        let mut strike = 0;
        let mut ball = 0;

        for (i, num) in nums.iter().enumerate() {
            if random_num[i] == *num {
                strike += 1;
            } else if random_num.contains(num) {
                ball += 1;
            }
        }

        println!("사용자 입력: {}", input);

        if strike == 3 {
            println!("정답입니다! (총 시도 횟수: {}회)", try_num);
            break;
        }

        println!("결과: {} Strike, {} Ball", strike, ball);
    }
}

fn randomize() -> Vec<i32> {
    let mut arr: Vec<i32> = Vec::new();

    while arr.len() < 3 {
        let num = rng().random_range(0..=9);

        if arr.is_empty() && num == 0 {
            continue;
        }

        if !arr.contains(&num) {
            arr.push(num);
        }
    }

    arr
}
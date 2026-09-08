use Lib::reader;

fn main() {
    let input = reader::read::<i32>(Some("판명할 숫자를 입력해주세요"));
    println!("입력: {}", input);

    let num = input.parse::<i32>().unwrap();

    let mut nums = validate_prime_num(num);

    println!("소수 목록:");
    for i in &mut nums {
        print!("{} ", i);
    }

    println!("\n소수의 개수: {}", nums.len());
}

fn validate_prime_num(num: i32) -> Vec<i32> {
    let mut rst = Vec::<i32>::new();

    for n in 2..=num{
        if (check_prime_num(n)){
            rst.push(n);
        }
    }

    return rst;
}

fn check_prime_num(num: i32) -> bool {
    for n in 2..num {
        if n * n > num {
            break;
        }

        if num % n == 0 {
            return false;
        }
    }

    return true;
}
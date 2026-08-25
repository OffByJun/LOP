use crate::Tools::reader;

// limitation
// Euclidean algorithm
fn main() {
    let input_str_1 = reader::read::<i32>(Some("첫 번째 숫자를 입력해주세요."));
    let input_str_2 = reader::read::<i32>(Some("첫 번째 숫자를 입력해주세요."));

    let input_num_1 = input_str_1.parse::<i32>().unwrap();
    let input_num_2 = input_str_2.parse::<i32>().unwrap();

    let mut gcd: i32 = gcd_fn(input_num_1, input_num_2); // 최대공약수
    let mut lcm: i32 = gcd_fn(input_num_1, input_num_2); // 최소공약수


    println!("첫 번째 수 : {}", input_num_1);
    println!("두 번째 수 : {}", input_num_2);

    println!("최대공약수 : {}", gcd);
    println!("최소공배수 : {}", lcm);
}

fn gcd_fn(mut a: i32, mut b: i32) -> i32 {
    let mut r = a % b;
    loop {
        if r == 0 {
            break;
        }

        a = b;
        b = r;
        r = a % b;
    }

    r
}

fn lcm_fn(a: i32, b: i32) -> i32 {
    a / gcd_fn(a, b) * b
}
use crate::Tools::reader;

fn main() {
    let input = reader::read::<usize>(Some("계단의 높이 :"));
    let n = input.parse::<usize>().unwrap();

    let result = stair(n);

    println!("올라가는 방법의 수 : {}", result);
}

fn stair(n: usize) -> u64 {
    if n == 1 {
        return 1;
    }

    if n == 2 {
        return 2;
    }

    let mut prev = 1u64;
    let mut current = 2u64;

    for _ in 3..=n {
        let next = prev + current;
        prev = current;
        current = next;
    }

    current
}
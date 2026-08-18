use std::io;
use std::str::FromStr;

pub fn read<T: FromStr>(pre_msg: Option<&str>) -> String {
    
    if let Some(n) = pre_msg {
        println!("{}", n);
    }

    loop {
        let mut result = String::new();


        io::stdin().read_line(&mut result).unwrap();
        let trimmed = result.trim();

        if validate::<T>(trimmed) {
            return trimmed.to_string();
        }

        println!("잘못된 입력입니다. 다시 입력해주세요: {}", trimmed);
    }
}

pub fn validate<T: FromStr>(input: &str) -> bool {
    if input.trim().is_empty() {
        return false;
    }

    return input.parse::<T>().is_ok()
}

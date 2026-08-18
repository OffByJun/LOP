use crate::Tools::reader;

mod Tools;

fn main() {
    let input = reader::read::<i32>(Some("숫자를 입력해주세요!"));

    let money = input.parse::<i32>().unwrap();
    println!("입력: {}", money);

    calculate_coin(money);
}

fn calculate_coin(reach: i32) {
    let mut money = reach;
    let mut fivehundred = 0;
    let mut hundred = 0;
    let mut fifty = 0;
    let mut ten = 0;

    while money > 0{
        if money >= 500 {
            fivehundred += 1;
            money -= 500;
        }
        else if money >= 100 {
            hundred += 1;
            money -= 100;
        }
        else if money >= 50 {
            fifty += 1;
            money -= 50;
        }
        else if money >= 10 {
            ten += 1;
            money -= 10;
        }
    }

    println!("출력: ");
    println!("500원: {}개", fivehundred);
    println!("100원: {}개", hundred);
    println!("50원: {}개", fifty);
    println!("10원: {}개", ten);
    println!("\n최소 동전 개수: {}개", fivehundred + hundred + fifty + ten);
}
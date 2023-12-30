use std::io;


fn main() {
    println!("Guess the number!");  // 打印提示信息：Guess the number!
    println!("Please input your guess.");  // 打印提示信息：Please input your guess.
    let mut guess = String::new();  // 生命一个String类型的变量
    io::stdin().read_line(&mut guess).expect("Failed to read line");  // 从终端获取输入
    println!("You guessed:{}",guess);  // 打印提示信息：You guessed:[数字]
}
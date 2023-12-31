use std::io;


fn main() {
    println!("Guess the number!");  // 打印提示信息：Guess the number!
    println!("Please input your guess.");  // 打印提示信息：Please input your guess.
    let mut guess = String::new();  // 声明一个可变的String类型的变量，new()是String的一个关联函数（静态方法）
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");  // 从终端获取输入并赋值给上一步声明的变量guess
    println!("You guessed:{}",guess);  // 打印提示信息：You guessed:[输入的数字]
}
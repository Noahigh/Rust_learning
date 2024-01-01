use std::io;
use std::cmp::Ordering;
use rand::Rng;  // trait（特征），随机数相关
// 可以使用Cargo doc --open 在本地构建一份有关所有依赖的文档，并自动地在浏览器中将文档打开

fn main() {
    println!("Guess the number!");  // 打印提示信息：Guess the number!
    let secret_number = rand::thread_rng().gen_range(1..101);  // 左闭右开1～100
    println!("The secret number is: {}", secret_number);  // 打印提示信息：The secret number is: [生成的随机数字]

    loop {
        println!("Please input your guess:");  // 打印提示信息：Please input your guess.
        let mut guess = String::new();  // 声明一个可变的String类型的变量，new()是String的一个关联函数（静态方法）
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");  // 从终端获取输入并赋值给上一步声明的变量guess
        // trim(): 去掉前后空格
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");  // 隐藏机制（Shadow），将字符串转换为某种数值，此处转换为u32（32位无符号整型）
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,  // _代表通配符，代表所有可能的Err
        };
        println!("You guessed:{}", guess);  // 打印提示信息：You guess2ed:[输入的数字]

        match guess.cmp(&secret_number) {  // 返回 guess 和 secret_number 的比较结果
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
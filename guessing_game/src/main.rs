use std::io;    // Элемент стандартной библиотеки, не входящий в "прелюдию"
use std::cmp::Ordering;
use rand::Rng;

// Точка входа в программу
fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        // Макрос для вывода в stdout
        println!("Please input your guess");

        // Создаём изменяемую переменную связанную с экземпляром String
        let mut guess = String::new();

        // Обрабатываем пользовательский ввод
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Обработка сбоя в программе

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        // Сравниваем переменную с помощью функции cmp
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Yot win!!");
                break;
            }
        }
    }
}
use std::io;    // Элемент стандартной библиотеки, не входящий в "прелюдию"

// Точка входа в программу
fn main() {
    println!("Guess the number");

    println!("Please input your guess");

    // Создаём изменяемую переменную связанную с экземпляром String
    let mut guess = String::new();

    // Обрабатываем пользовательский ввод
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // Обработка сбоя в программе

    println!("You guessed: {guess}");
}
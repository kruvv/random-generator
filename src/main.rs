use random_generator::{format_numbers, generate_unique_numbers, validator};
use std::io::{self, Write};

fn main() {
    println!("=== Генератор уникальных случайных чисел ===");

    let n = read_positive_number("Введите количество возможных значений (N): ");
    let m = read_positive_number("Введите сколько чисел сгенерировать (M): ");

    if !validator::can_generate_unique_numbers(n, m) {
        eprintln!(
            "Ошибка: невозможно сгенерировать {} уникальных чисел из диапазона 1..{}",
            m, n - 1
        );
        std::process::exit(1);
    }

    let result = generate_unique_numbers(n, m);
    println!("\nРезультат: {}", format_numbers(&result));
}

fn read_positive_number(prompt: &str) -> u64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");

        let input = input.trim();

        if !validator::is_valid_positive_number(input) {
            eprintln!("Ошибка: введите целое положительное число (только цифры)");
            continue;
        }

        return input.parse().unwrap();
    }
}
use random_generator::{format_numbers, generate_unique_numbers, validator};
use std::io::{self, Write};

fn select_variant() {
    println!("\n*** Выберите вариант: ***");
    println!("0. Выход");
    println!("1. Первое поле 7 из 35, второе поле 1 из 54");
    println!("2. Первое поле M из N, второе поле K из P");
}

fn main() {
    println!("=== Генератор уникальных случайных чисел ===");

    let (n, m, p, k) = loop {
        select_variant();
        let v = read_positive_number("\nВаш вариант: ");

        match v {
            0 => std::process::exit(0),
            1 => {
                check_generate_unique_numbers(35, 7);
                check_generate_unique_numbers(54, 1);
                break (35, 7, 54, 1);
            }
            2 => {
                let n = read_positive_number(
                    "Введите количество возможных значений первого поля (N): ",
                );
                let m = read_positive_number("Введите сколько чисел сгенерировать (M): ");
                let p = read_positive_number(
                    "Введите количество возможных значений второго поля (P): ",
                );
                let k = read_positive_number("Введите сколько чисел сгенерировать (K): ");
                check_generate_unique_numbers(n, m);
                check_generate_unique_numbers(p, k);
                break (n, m, p, k);
            }
            _ => {
                eprint!("К сожелению такого варианта нет, попробуйте другой вариант.\n");
            }
        }
    };

    let s = read_positive_number("Введите количество сетов (S): ");
    println!("");

    for item in 0..s {
        let result1 = generate_unique_numbers(n, m);
        let result2 = generate_unique_numbers(p, k);
        println!(
            "Результат {}: {}, {}",
            item + 1,
            format_numbers(&result1),
            format_numbers(&result2)
        );
    }
}

fn read_positive_number(prompt: &str) -> u64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Ошибка чтения ввода");

        let input = input.trim();

        // Разрешаем «0» – используется для выхода.
        if input == "0" {
            return 0;
        }

        // Требуем положительное число.
        if !validator::is_valid_positive_number(input) {
            eprintln!("Ошибка: введите целое положительное число (только цифры)");
            continue;
        }

        // Парсим, зная, что строка содержит только цифры.
        return input
            .parse::<u64>()
            .expect("Не удалось преобразовать ввод в число");
    }
}

fn check_generate_unique_numbers(d: u64, x: u64) /*-> Result<(), String>*/
{
    if !validator::can_generate_unique_numbers(d, x) {
        /*  Ok(())
        } else {*/
        eprintln!(
            "Ошибка: невозможно сгенерировать {} уникальных чисел из диапазона 1..{}",
            x,
            d - 1
        );
        std::process::exit(1);
    }
}

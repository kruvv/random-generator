use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;

/// Генерирует M уникальных чисел из диапазона 1..N-1
pub fn generate_unique_numbers(n: u64, m: u64) -> Vec<u64> {
    let total_possible = n - 1;

    // Гибридная стратегия: выбираем оптимальный метод
    if m > total_possible / 2 {
        generate_by_shuffling(n, m)
    } else {
        generate_by_hashset(n, m)
    }
}

/// Метод 1: Генерация через HashSet (для малых M)
fn generate_by_hashset(n: u64, m: u64) -> Vec<u64> {
    let mut rng = rand::thread_rng();
    let mut set = HashSet::with_capacity(m as usize);
    let mut result = Vec::with_capacity(m as usize);

    while result.len() < m as usize {
        let num = rng.gen_range(1..n);
        if set.insert(num) {
            result.push(num);
        }
    }

    result
}

/// Метод 2: Перемешивание всех чисел (для больших M)
fn generate_by_shuffling(n: u64, m: u64) -> Vec<u64> {
    let mut numbers: Vec<u64> = (1..n).collect();
    let mut rng = rand::thread_rng();
    numbers.shuffle(&mut rng);
    numbers.truncate(m as usize);
    numbers
}

/// Форматирует вектор чисел в строку вида [ 1, 23, 4, 17 ]
pub fn format_numbers(numbers: &[u64]) -> String {
    if numbers.is_empty() {
        return "[ ]".to_string();
    }

    let elements: Vec<String> = numbers.iter().map(|&num| num.to_string()).collect();
    format!("[ {} ]", elements.join(", "))
}

/// Валидатор для пользовательского ввода
pub mod validator {
    /// Проверяет, что строка содержит только цифры и является положительным числом
    pub fn is_valid_positive_number(input: &str) -> bool {
        if input.is_empty() {
            return false;
        }

        // Только цифры
        if !input.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }

        // Парсим и проверяем, что > 0
        match input.parse::<u64>() {
            Ok(num) => num > 0,
            Err(_) => false,
        }
    }

    /// Проверяет, можно ли сгенерировать M уникальных чисел из диапазона 1..N-1
    pub fn can_generate_unique_numbers(n: u64, m: u64) -> bool {
        n > 1 && m > 0 && m <= n - 1
    }
}

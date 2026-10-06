use random_generator::validator;

#[test]
fn test_valid_positive_numbers() {
    assert!(validator::is_valid_positive_number("1"));
    assert!(validator::is_valid_positive_number("42"));
    assert!(validator::is_valid_positive_number("1000"));
    assert!(validator::is_valid_positive_number("999999999"));
}

#[test]
fn test_invalid_positive_numbers() {
    // Пустая строка
    assert!(!validator::is_valid_positive_number(""));

    // Ноль
    assert!(!validator::is_valid_positive_number("0"));

    // Отрицательные числа
    assert!(!validator::is_valid_positive_number("-1"));
    assert!(!validator::is_valid_positive_number("-42"));

    // Буквы и спецсимволы
    assert!(!validator::is_valid_positive_number("abc"));
    assert!(!validator::is_valid_positive_number("12abc"));
    assert!(!validator::is_valid_positive_number("12.5"));
    assert!(!validator::is_valid_positive_number("12,5"));
    assert!(!validator::is_valid_positive_number("12 5"));
    assert!(!validator::is_valid_positive_number("!@#"));
    assert!(!validator::is_valid_positive_number("1a2b3c"));
}

#[test]
fn test_can_generate_unique_numbers() {
    // Валидные случаи
    assert!(validator::can_generate_unique_numbers(10, 1));
    assert!(validator::can_generate_unique_numbers(10, 5));
    assert!(validator::can_generate_unique_numbers(10, 9));
    assert!(validator::can_generate_unique_numbers(2, 1));
    assert!(validator::can_generate_unique_numbers(100, 50));

    // Невалидные случаи
    assert!(!validator::can_generate_unique_numbers(1, 1)); // N <= 1
    assert!(!validator::can_generate_unique_numbers(10, 0)); // M = 0
    assert!(!validator::can_generate_unique_numbers(10, 10)); // M > N-1
    assert!(!validator::can_generate_unique_numbers(5, 10)); // M > N-1
    assert!(!validator::can_generate_unique_numbers(0, 1)); // N = 0
}

#[test]
fn test_validator_edge_cases() {
    // Очень большие числа
    assert!(validator::is_valid_positive_number("18446744073709551615")); // max u64

    // Числа с пробелами
    assert!(!validator::is_valid_positive_number(" 123 ")); // пробелы не допускаются

    // Смешанные символы
    assert!(!validator::is_valid_positive_number("123abc456"));
}

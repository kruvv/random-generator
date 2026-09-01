use random_generator::{format_numbers, generate_unique_numbers, validator};

#[test]
fn test_full_generation_flow() {
    let n = 100;
    let m = 20;
    
    // Проверяем валидацию
    assert!(validator::can_generate_unique_numbers(n, m));
    
    // Генерируем
    let result = generate_unique_numbers(n, m);
    
    // Проверяем результат
    assert_eq!(result.len(), m as usize);
    
    // Проверяем форматирование
    let formatted = format_numbers(&result);
    assert!(formatted.starts_with("[ "));
    assert!(formatted.ends_with(" ]"));
    
    // Проверяем, что все числа в правильном диапазоне
    for &num in &result {
        assert!(num >= 1 && num < n);
    }
}

#[test]
fn test_deterministic_validation_and_generation() {
    // Тест на граничных значениях
    let test_cases = vec![
        (10, 1, true),
        (10, 9, true),
        (10, 10, false),
        (1, 1, false),
        (0, 1, false),
        (5, 0, false),
    ];
    
    for (n, m, should_be_valid) in test_cases {
        assert_eq!(
            validator::can_generate_unique_numbers(n, m),
            should_be_valid,
            "n={}, m={} should be {}",
            n, m, if should_be_valid { "valid" } else { "invalid" }
        );
        
        if should_be_valid {
            let result = generate_unique_numbers(n, m);
            assert_eq!(result.len(), m as usize);
        }
    }
}

#[test]
fn test_generate_with_max_capacity() {
    let n = 50;
    let m = 49; // Максимально возможное количество
    
    let result = generate_unique_numbers(n, m);
    assert_eq!(result.len(), m as usize);
    
    // Проверяем, что все числа от 1 до 49 присутствуют
    let mut sorted = result.clone();
    sorted.sort();
    let expected: Vec<u64> = (1..n).collect();
    assert_eq!(sorted, expected);
}
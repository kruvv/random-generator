use random_generator::{format_numbers, generate_unique_numbers};
use std::collections::HashSet;

#[test]
fn test_generate_unique_count() {
    let n = 100;
    let m = 10;
    let result = generate_unique_numbers(n, m);
    assert_eq!(result.len(), m as usize);

    // Проверяем, что все числа в диапазоне 1..n-1
    for &num in &result {
        assert!(num >= 1 && num < n);
    }

    // Проверяем, что все числа уникальны
    let set: HashSet<_> = result.iter().collect();
    assert_eq!(set.len(), result.len());
}

#[test]
fn test_hybrid_strategy() {
    let n = 100;

    // Малый M - должен использовать HashSet
    let small_m = 10;
    let result1 = generate_unique_numbers(n, small_m);
    assert_eq!(result1.len(), small_m as usize);

    // Большой M - должен использовать перемешивание
    let large_m = 80;
    let result2 = generate_unique_numbers(n, large_m);
    assert_eq!(result2.len(), large_m as usize);
}

#[test]
fn test_edge_cases() {
    // Минимальный случай
    let n = 2;
    let m = 1;
    let result = generate_unique_numbers(n, m);
    assert_eq!(result, vec![1]);

    // Полный диапазон
    let n = 5;
    let m = 4;
    let mut result = generate_unique_numbers(n, m);
    result.sort();
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_generate_large_range() {
    let n = 1000;
    let m = 500;
    let result = generate_unique_numbers(n, m);
    assert_eq!(result.len(), m as usize);

    // Проверяем уникальность для большого набора
    let set: HashSet<_> = result.iter().collect();
    assert_eq!(set.len(), result.len());
}

#[test]
fn test_format_numbers() {
    assert_eq!(format_numbers(&[]), "[ ]");
    assert_eq!(format_numbers(&[1]), "[ 1 ]");
    assert_eq!(format_numbers(&[1, 2, 3]), "[ 1, 2, 3 ]");
    assert_eq!(format_numbers(&[10, 20, 30, 40]), "[ 10, 20, 30, 40 ]");
}

#[test]
fn test_shuffling_returns_all_numbers() {
    let n = 10;
    let m = 9;
    let mut result = generate_unique_numbers(n, m);
    result.sort();
    assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

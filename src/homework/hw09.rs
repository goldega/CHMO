// Функція для зсуву рядка
pub fn shift_string(s: &str, n: usize) -> String {
    let len = s.len();
    
    if len == 0 {
        return String::new();
    }

    // Оскільки зсув більше довжини рядка повторюється, беремо залишок від ділення на довжину
    let n = n % len;

    // Розділяємо рядок на дві частини та об'єднуємо їх після зсуву
    let (first, second) = s.split_at(len - n);
    format!("{}{}", second, first)
}

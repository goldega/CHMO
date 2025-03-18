pub fn is_palindrome(n: i32) -> bool {
    // Якщо число негативне, воно не може бути паліндромом
    if n < 0 {
        return false;
    }

    let original = n;  // зберігаємо оригінальне число
    let mut reversed = 0;  // змінна для перевертання числа
    let mut num = n;

    // Перевертаємо число
    while num != 0 {
        let digit = num % 10;  // отримуємо останню цифру
        reversed = reversed * 10 + digit;  // додаємо її до перевертеного числа
        num /= 10;  // усуваємо останню цифру з числа
    }

    original == reversed  // порівнюємо оригінальне і перевернуте числа
}

pub fn shift_string(s: &str, n: usize) -> String {
    let len = s.len();
    
    if len == 0 {
        return String::new();
    }

    let n = n % len;

    let (first, second) = s.split_at(len - n);
    format!("{}{}", second, first)
}

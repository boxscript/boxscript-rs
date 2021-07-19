pub fn chars(code: &str) -> Vec<Vec<char>> {
    let length: Option<usize> = code.lines().map(|line| line.chars().count()).max();

    if length.is_none() {
        Vec::new()
    } else {
        code.lines()
            .map(|line| {
                format!("{1:\u{0}<0$}", length.unwrap(), line)
                    .chars()
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_chars() {
        use super::chars;
        assert_eq!(chars(&"ab\ncd"), vec![vec!['a', 'b'], vec!['c', 'd']]);
        assert_eq!(chars(&""), Vec::new() as Vec<Vec<char>>);
        assert_eq!(chars(&"ab\nc"), vec![vec!['a', 'b'], vec!['c', '\0']]);
    }
}

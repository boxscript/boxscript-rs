use std::collections::HashMap;

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

pub fn neighboring(code: &str, loc: &[usize; 2]) -> HashMap<char, char> {
    let mut neighbors: HashMap<char, char> = HashMap::new();
    let matrix: Vec<Vec<char>> = chars(code);
    let directions: [(char, [isize; 2]); 4] =
        [('N', [-1, 0]), ('S', [1, 0]), ('W', [0, -1]), ('E', [0, 1])];

    for (key, value) in &directions {
        let row = &matrix.get((loc[0] as isize + value[0]) as usize);
        if row.is_none() {
            neighbors.insert(*key, '\0');
        } else {
            let col = &row.unwrap().get((loc[1] as isize + value[1]) as usize);
            if col.is_none() {
                neighbors.insert(*key, '\0');
            } else {
                neighbors.insert(*key, *col.unwrap());
            }
        }
    }

    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_turns_strings_to_chars() {
        assert_eq!(chars(&"ab\ncd"), vec![vec!['a', 'b'], vec!['c', 'd']]);

        assert_eq!(chars(&""), Vec::new() as Vec<Vec<char>>);

        assert_eq!(chars(&"ab\nc"), vec![vec!['a', 'b'], vec!['c', '\0']]);
    }

    #[test]
    fn it_finds_neighbors() {
        assert_eq!(
            neighboring(&"ab\ncd", &[0, 0]),
            [('N', '\0'), ('S', 'c'), ('W', '\0'), ('E', 'b')]
                .iter()
                .cloned()
                .collect::<super::HashMap<char, char>>()
        );

        assert_eq!(
            neighboring(&"", &[0, 0]),
            [('N', '\0'), ('S', '\0'), ('W', '\0'), ('E', '\0')]
                .iter()
                .cloned()
                .collect::<super::HashMap<char, char>>()
        );

        assert_eq!(
            neighboring(&"abc\ndef\nghi", &[1, 1]),
            [('N', 'b'), ('S', 'h'), ('W', 'd'), ('E', 'f')]
                .iter()
                .cloned()
                .collect::<super::HashMap<char, char>>()
        );

        assert_eq!(
            neighboring(&"abc\ndef\nghi", &[100, 43000]),
            [('N', '\0'), ('S', '\0'), ('W', '\0'), ('E', '\0')]
                .iter()
                .cloned()
                .collect::<super::HashMap<char, char>>()
        );
    }
}

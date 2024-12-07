pub fn read_to_chars(filename: &str) -> Vec<Vec<char>> {
    let mut chars: Vec<Vec<char>> = std::fs::read_to_string(filename).unwrap().split("\n").map(|x| x.chars().collect()).collect();
    chars.retain(|x| *x != []);
    chars
}

pub fn read_to_vec(filename: &str) -> Vec<String> {
    let mut lines: Vec<String> = std::fs::read_to_string(filename).unwrap().split("\n").map(|x| x.to_string()).collect();
    lines.retain(|x| *x != "".to_string());
    lines
}



#[cfg(test)]
mod tests {
    use crate::read_to_chars;

    #[test]
    fn read_to_chars_test() {
        assert_eq!(read_to_chars("tests/file"),    "testfile\nabcdefgh\n12345678".split("\n").map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>());
    }
}
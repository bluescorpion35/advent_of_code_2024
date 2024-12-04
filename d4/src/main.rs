fn main() {
    let mut file = std::fs::read_to_string("input")
        .unwrap()
        .split("\n")
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();
    for mut i in &mut file {
        i.append(&mut vec!['u', 'u', 'u']);
    }
    let mut temp: Vec<char> = vec![];

    for i in 0..file.len() + 3 {
        temp.push('u');
    }
    for i in 0..3 {
        file.push(temp.clone());
        file.insert(0, temp.clone());
    }
    for i in &file {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
    let mut count = 0;
    for i in 3..(file.len() - 3) {
        for j in 0..(file[i].len() - 3) {
            print!("\nline: {}\n", file[i].clone().into_iter().collect::<String>());
                if check(
                    file[i][j],
                    file[i][j + 2],
                    file[i + 1][j + 1],
                    file[i + 2][j],
                    file[i + 2][j + 2]) { count += 1; }
        }
        println!("----------------- END -----------------");
    }
    println!("{}", count);
}

fn check(a: char, b: char, c: char, d: char, e: char) -> bool {
    print!(" chars: {}{}{}{}{}", a, b, c, d, e);
    let chars = vec![a, b, c, d, e];
    let string = chars.iter().collect::<String>();
    println!(" chars: {}", string == "SSAMM" || string == "MMASS" || string == "MSAMS" ||  string == "SMASM");
    string == "SSAMM" || string == "MMASS" || string == "MSAMS" ||  string == "SMASM"
}

fn main() {
    let file = std::fs::read_to_string("input").unwrap();
    let mut lines: Vec<&str> = file.split('\n').collect();
    lines.truncate(lines.len() - 1);
    let mut lines_split: Vec<Vec<i32>> = vec![];
    for elem in lines {
        lines_split.push(elem.split(' ').map(|x| x.parse::<i32>().unwrap()).collect());
    }

    let mut safe = 0;
    let mut counter = 0;
    for line in lines_split {
        println!("---------- exited {counter} ----------");
        counter += 1;
        if is_safe(line.clone()) {
            safe += 1;
        } else {
            for i in 0..line.len() {
                let mut line_cloned = line.clone();
                let _ = line_cloned.remove(i);
                if is_safe(line_cloned) {
                    safe += 1;
                    break;
                }
            }
        }
    }
    println!("safe: {}", safe);
}

fn is_safe(line: Vec<i32>) -> bool {
    let mut inc: Vec<i32> = vec![];
    for i in 0..line.len() - 1 {
        inc.push(line[i + 1] - line[i]);
    }

    println!("{:?}", inc);

    let mut neg = inc[0].is_negative();

    for i in &inc {
        println!("{i}");
        println!("{neg} {}", *i < 0);
        if !(neg == (i.is_negative())) || i.abs() == 0 || i.abs() > 3 {
            println!("---------- Error -----------");
            return false;
        }
    }

    true
}

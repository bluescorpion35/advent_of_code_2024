fn main() {
    let file = std::fs::read_to_string("input").unwrap();
    let mut lines = file.split("\n").collect::<Vec<&str>>();

    lines.truncate(lines.len() - 1);

    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for  line in &lines {
        let temp: Vec<&str> = line.split("   ").collect();
        list1.push(temp[0].parse::<i32>().unwrap());
        list2.push(temp[1].parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();

    let mut similarities: Vec<i32> = vec![];

    for num in &list2 {
        similarities.push((list1.iter().filter(|&n| *n == *num).count() as i32)  * num);
    }

    println!("{:?}", similarities.into_iter().sum::<i32>());
}

use regex::Regex;

fn main() {
    let file = std::fs::read_to_string("input").unwrap();
    let lines: Vec<&str> = file.split('\n').collect();
    let mut valid: Vec<&str> = vec![];

    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();

    for line in lines {
        valid.append(&mut re.find_iter(line).map(|m| m.as_str()).collect::<Vec<&str>>());
    }
    let mut switch = true;
    let mut total = 0;
    let re = Regex::new(r"[0-9]+,[0-9]+").unwrap();

    for i in valid {
        if i == "don't()" {
            switch = false;
        } else if i == "do()" {
            switch = true;
        } else if switch {
            let nums = re.find(i).map(|m| m.as_str()).unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            println!("{:?} {:?}", nums, i);
            total += nums[0] * nums[1];
        }
    }
    println!("{}", total);

}

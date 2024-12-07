use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct Input {
    row_obstacles: BTreeMap<usize, Vec<usize>>,
    col_obstacles: BTreeMap<usize, Vec<usize>>,
    start_pos: (usize, usize),
    num_rows: usize,
    num_cols: usize,
}

fn read_input(filename: &str) -> Input {
    let mut row_obstacles: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut col_obstacles: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut start_pos = None;
    let mut num_rows = 0;
    let mut num_cols = 0;

    let content = fs::read_to_string(filename).expect("Failed to read file");

    for (r, line) in content.lines().enumerate() {
        if start_pos.is_none() {
            if let Some(pos) = line.find('^') {
                start_pos = Some((r, pos));
            }
        }

        let obs: Vec<usize> = line
            .chars()
            .enumerate()
            .filter_map(|(c, ch)| if ch == '#' { Some(c) } else { None })
            .collect();

        if !obs.is_empty() {
            row_obstacles.insert(r, obs.clone());
            for &c in &obs {
                col_obstacles.entry(c).or_insert_with(Vec::new).push(r);
            }
        }

        num_rows += 1;
        num_cols = line.len();
    }

    Input {
        row_obstacles,
        col_obstacles,
        start_pos: start_pos.expect("No starting position found"),
        num_rows,
        num_cols,
    }
}

fn first_less_than(items: &[usize], x: usize) -> Option<usize> {
    items.iter().rev().find(|&&item| item < x).cloned()
}

fn first_greater_than(items: &[usize], x: usize) -> Option<usize> {
    items.iter().find(|&&item| item > x).cloned()
}

fn patrol(
    row_obstacles: &mut BTreeMap<usize, Vec<usize>>,
    col_obstacles: &mut BTreeMap<usize, Vec<usize>>,
    num_rows: usize,
    num_cols: usize,
    start_pos: (usize, usize),
) -> Option<BTreeSet<(usize, usize, usize, usize)>> {
    let mut dir = '^';
    let (mut r, mut c) = start_pos;
    let mut segments = BTreeSet::new();

    loop {
        let (mut new_r, mut new_c) = (r, c);
        match dir {
            '^' => {
                if let Some(obs_r) = col_obstacles
                    .get(&c)
                    .and_then(|obs| first_less_than(obs, r))
                {
                    new_r = obs_r + 1;
                    dir = '>';
                } else {
                    segments.insert((r, c, 0, c));
                    break;
                }
            }
            '>' => {
                if let Some(obs_c) = row_obstacles
                    .get(&r)
                    .and_then(|obs| first_greater_than(obs, c))
                {
                    new_c = obs_c - 1;
                    dir = 'v';
                } else {
                    segments.insert((r, c, r, num_cols - 1));
                    break;
                }
            }
            'v' => {
                if let Some(obs_r) = col_obstacles
                    .get(&c)
                    .and_then(|obs| first_greater_than(obs, r))
                {
                    new_r = obs_r - 1;
                    dir = '<';
                } else {
                    segments.insert((r, c, num_rows - 1, c));
                    break;
                }
            }
            '<' => {
                if let Some(obs_c) = row_obstacles
                    .get(&r)
                    .and_then(|obs| first_less_than(obs, c))
                {
                    new_c = obs_c + 1;
                    dir = '^';
                } else {
                    segments.insert((r, c, r, 0));
                    break;
                }
            }
            _ => unreachable!(),
        }

        let new_segment = (r, c, new_r, new_c);
        if segments.contains(&new_segment) {
            return None;
        }
        segments.insert(new_segment);
        r = new_r;
        c = new_c;
    }

    Some(segments)
}

fn get_covered_points(
    segments: &BTreeSet<(usize, usize, usize, usize)>,
) -> BTreeSet<(usize, usize)> {
    let mut covered_points = BTreeSet::new();

    for &(r1, c1, r2, c2) in segments {
        if r1 == r2 {
            for c in c1.min(c2)..=c1.max(c2) {
                covered_points.insert((r1, c));
            }
        } else if c1 == c2 {
            for r in r1.min(r2)..=r1.max(r2) {
                covered_points.insert((r, c1));
            }
        }
    }

    covered_points
}

fn find_patrol_cycles(
    covered_points: &BTreeSet<(usize, usize)>,
    row_obstacles: &mut BTreeMap<usize, Vec<usize>>,
    col_obstacles: &mut BTreeMap<usize, Vec<usize>>,
    num_rows: usize,
    num_cols: usize,
    start_pos: (usize, usize),
) -> usize {
    let mut num_cycles = 0;

    for &(r, c) in covered_points {
        row_obstacles.entry(r).or_default().push(c);
        row_obstacles.get_mut(&r).unwrap().sort();

        col_obstacles.entry(c).or_default().push(r);
        col_obstacles.get_mut(&c).unwrap().sort();

        if patrol(row_obstacles, col_obstacles, num_rows, num_cols, start_pos).is_none() {
            num_cycles += 1;
        }

        row_obstacles.get_mut(&r).unwrap().retain(|&x| x != c);
        col_obstacles.get_mut(&c).unwrap().retain(|&x| x != r);
    }

    num_cycles
}

fn main() {
    let filename = "input3";
    let input = read_input(filename);

    // Part 1
    let part1_start = Instant::now();

    let segments = patrol(
        &mut input.row_obstacles.clone(),
        &mut input.col_obstacles.clone(),
        input.num_rows,
        input.num_cols,
        input.start_pos,
    )
    .expect("No segments found");

    let covered_points = get_covered_points(&segments);
    let part1_duration = part1_start.elapsed();

    println!(
        "Part1: steps_covered = {} (Elapsed: {:.2?})",
        covered_points.len(),
        part1_duration
    );

    // Part 2
    let part2_start = Instant::now();

    let num_cycles = find_patrol_cycles(
        &covered_points,
        &mut input.row_obstacles.clone(),
        &mut input.col_obstacles.clone(),
        input.num_rows,
        input.num_cols,
        input.start_pos,
    );
    let part2_duration = part2_start.elapsed();

    println!(
        "Part2: num_cycles = {} (Elapsed: {:.2?})",
        num_cycles, part2_duration
    );
}

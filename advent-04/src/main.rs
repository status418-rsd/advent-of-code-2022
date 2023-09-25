use std::iter::Peekable;
use std::str::Lines;

fn main() {
    let mut peekable: Peekable<Lines> = include_str!("input").lines().peekable();
    let mut res_1: u32 = 0;
    let mut res_2: u32 = 0;
    while let Some(mut line) = peekable.next() {
        let assignment_paris: Vec<&str> = line.rsplit_terminator(",").collect();
        let range_1 = get_range(assignment_paris[0].split("-").collect());
        let range_2 = get_range(assignment_paris[1].split("-").collect());

        // part 1
        if (range_1.contains(range_2.iter().max().unwrap()) && range_1.contains(range_2.iter().min().unwrap())) ||
          (range_2.contains(range_1.iter().max().unwrap()) && range_2.contains(range_1.iter().min().unwrap())) {
            res_1 += 1;
        }


        // part 2
        for val in range_1 {
            if range_2.contains(&val) {
                res_2 +=1;
                break;
            }
        }
    }
    println!("{}", res_1);
    println!("{}", res_2);
}


fn get_range(pair: Vec<&str>) -> Vec<u32> {
    let start: u32 = pair[0].parse::<u32>().unwrap();
    let end: u32 = pair[1].parse::<u32>().unwrap();
    (start..=end).collect()
}

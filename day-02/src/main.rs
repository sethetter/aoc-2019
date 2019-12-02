use std::fs;

fn main() {
    let raw_contents = fs::read_to_string("input.txt").expect("Error reading the file.");
    let contents = raw_contents.trim(); // get rid of trailing \n

    let mut intcodes: Vec<usize> = contents.split(",").map(|x| x.parse().unwrap()).collect();

    intcodes[1] = 12;
    intcodes[2] = 2;

    println!("Part 1: {}\n", process(intcodes.clone()));

    let target = 19690720;
    let mut result = 0;
    let mut noun = 0;
    let mut verb = 0;

    for n in 0..99 {
        for v in 0..99 {
            intcodes[1] = n;
            intcodes[2] = v;

            result = process(intcodes.clone());

            if result == target {
                noun = n;
                verb = v;
                break;
            }
        }
        if result == target { break; }
    }

    println!("Part 2: {}", (noun * 100) + verb);
}

fn process(mut intcodes: Vec<usize>) -> usize {
    let mut pos = 0;

    while intcodes[pos] != 99 {
        let op = intcodes[pos];
        let x = intcodes[pos+1];
        let y = intcodes[pos+2];
        let store_at = intcodes[pos+3];

        match op {
            1 => intcodes[store_at] = intcodes[x] + intcodes[y],
            2 => intcodes[store_at] = intcodes[x] * intcodes[y],
            _ => println!("Invalid op code {} at pos {}", op, pos),
        }

        pos += 4;
    }

    intcodes[0]
}

#[test]
fn test_process() {
    assert_eq!(process(vec![1,0,0,0,99]), 2);
}

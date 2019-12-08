use std::fs;

fn main() {
    let raw_input = fs::read_to_string("input.txt").expect("Failed to read input file.");
    let input = raw_input.trim(); // get rid of tailing \n

    let w = 25;
    let h = 6;

    let nums: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let layer_w_fewest_zeros = nums.chunks(w * h).min_by(|f1, f2| {
        let f1_0s = f1.iter().filter(|x| *x == &0).count();
        let f2_0s = f2.iter().filter(|x| *x == &0).count();
        f1_0s.cmp(&f2_0s)
    }).unwrap();

    let count_1s = layer_w_fewest_zeros.iter().filter(|x| *x == &1).count();
    let count_2s = layer_w_fewest_zeros.iter().filter(|x| *x == &2).count();

    println!("Part 1: {}", count_1s * count_2s);
}

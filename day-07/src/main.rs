use std::fs;
use std::cmp::max;
use permutohedron::Heap;

type IntCodes = Vec<isize>;

#[derive(Clone)]
struct IntCodeComp {
    pos: usize,
    codes: IntCodes,
    inputs: Vec<isize>,
    output: Vec<isize>,
}

impl IntCodeComp {
    fn new(codes: IntCodes, inputs: Vec<isize>) -> IntCodeComp {
        IntCodeComp{codes: codes, inputs: inputs, pos: 0, output: vec![]}
    }

    fn first_out(&self) -> isize {
        *self.clone().skip_while(|outs| outs.len() == 0).nth(0).unwrap().iter().nth(0).unwrap()
    }

    fn param(&self, t: char, v: isize) -> isize {
        match t {
            '0' => self.codes[v as usize],
            '1' => v,
            _ => unreachable!(),
        }
    }

    fn n_params(&self, opcode: String, num: usize) -> Vec<isize> {
        (1..=num).fold(vec![], |mut out, i| {
            let param_type = opcode.chars().nth(4-i).unwrap();
            out.push(self.param(param_type, self.codes[self.pos+i]));
            out.clone()
        })
    }
}

impl Iterator for IntCodeComp {
    type Item = Vec<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.codes[self.pos] == 99 { return None }

        let opcode = format!("{:0>6}", self.codes[self.pos]);
        match opcode.chars().nth(5) {
            Some('1') => { // Add
                let params = self.n_params(opcode, 2);
                let dest = self.codes[self.pos+3] as usize;
                self.codes[dest] = params[0] + params[1];
                self.pos += 4;
            },
            Some('2') => { // Mult
                let params = self.n_params(opcode, 2);
                let dest = self.codes[self.pos+3] as usize;
                self.codes[dest] = params[0] * params[1];
                self.pos += 4;
            },
            Some('3') => { // Input
                let dest = self.codes[self.pos+1] as usize;
                self.codes[dest] = self.inputs.pop().unwrap();
                self.pos += 2;
            },
            Some('4') => { // Output
                self.output.push(*self.n_params(opcode, 1).first().unwrap());
                self.pos += 2;
            },
            Some('5') => { // Jump If True
                let params = self.n_params(opcode, 2);
                if params[0] > 0 {
                    self.pos = params[1] as usize;
                } else {
                    self.pos += 3;
                }
            },
            Some('6') => { // Jump If False
                let params = self.n_params(opcode, 2);
                if params[0] == 0 {
                    self.pos = params[1] as usize;
                } else {
                    self.pos += 3;
                }
            },
            Some('7') => { // Less Than
                let params = self.n_params(opcode, 2);
                let dest = self.codes[self.pos+3] as usize;
                if params[0] < params[1] {
                    self.codes[dest] = 1;
                } else {
                    self.codes[dest] = 0;
                }
                self.pos += 4;
            },
            Some('8') => { // Equals
                let params = self.n_params(opcode, 2);
                let dest = self.codes[self.pos+3] as usize;
                if params[0] == params[1] {
                    self.codes[dest] = 1;
                } else {
                    self.codes[dest] = 0;
                }
                self.pos += 4;
            },
            Some(_) => return None,
            None => return None,
        }
        Some(self.output.clone())
    }
}

fn main() {
    let raw_contents = fs::read_to_string("input.txt").expect("Error reading the file.");
    let contents = raw_contents.trim(); // get rid of trailing \n

    let intcodes: IntCodes = contents.split(',').map(|x| x.parse().unwrap()).collect();

    let max_out_perm = permutations(vec![0,1,2,3,4]).iter().fold(0, |prev_max, perms| {
        let amp1 = IntCodeComp::new(intcodes.clone(), vec![0, perms[0]]);
        let amp2 = IntCodeComp::new(intcodes.clone(), vec![amp1.first_out(), perms[1]]);
        let amp3 = IntCodeComp::new(intcodes.clone(), vec![amp2.first_out(), perms[2]]);
        let amp4 = IntCodeComp::new(intcodes.clone(), vec![amp3.first_out(), perms[3]]);
        let amp5 = IntCodeComp::new(intcodes.clone(), vec![amp4.first_out(), perms[4]]);

        max(amp5.first_out(), prev_max)
    });

    println!("Part1 : {}", max_out_perm);
}

fn permutations(mut v: Vec<isize>) -> Vec<Vec<isize>> {
    let heap = Heap::new(&mut v);
    heap.into_iter().collect()
}

fn main() {
    let instructions = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 9, 19, 23, 2, 23, 13, 27,
        1, 27, 9, 31, 2, 31, 6, 35, 1, 5, 35, 39, 1, 10, 39, 43, 2, 43, 6, 47, 1, 10, 47, 51, 2, 6,
        51, 55, 1, 5, 55, 59, 1, 59, 9, 63, 1, 13, 63, 67, 2, 6, 67, 71, 1, 5, 71, 75, 2, 6, 75,
        79, 2, 79, 6, 83, 1, 13, 83, 87, 1, 9, 87, 91, 1, 9, 91, 95, 1, 5, 95, 99, 1, 5, 99, 103,
        2, 13, 103, 107, 1, 6, 107, 111, 1, 9, 111, 115, 2, 6, 115, 119, 1, 13, 119, 123, 1, 123,
        6, 127, 1, 127, 5, 131, 2, 10, 131, 135, 2, 135, 10, 139, 1, 13, 139, 143, 1, 10, 143, 147,
        1, 2, 147, 151, 1, 6, 151, 0, 99, 2, 14, 0, 0,
    ];
    for noun in 0..100 {
        for verb in 0..100 {
            let mut new_instructions = instructions.clone();
            new_instructions[1] = noun;
            new_instructions[2] = verb;
            let answer = compute(new_instructions)[0];
            // println!("{}", answer);
            if answer == 19_690_720 {
                println!("Part 2: {}", 100 * noun + verb)
            }
        }
    }
    println!("{:?}", compute(instructions));
}

fn compute(instructions: Vec<i32>) -> Vec<i32> {
    let mut instructions = instructions;
    let mut location: usize = 0;
    let mut next_instruction = instructions[location];
    while next_instruction != 99 {
        match next_instruction {
            1 => {
                let destination: usize = instructions[location + 3] as usize;
                let read_1: usize = instructions[location + 1] as usize;
                let read_2: usize = instructions[location + 2] as usize;
                instructions[destination] = instructions[read_1] + instructions[read_2]
            }
            2 => {
                let destination: usize = instructions[location + 3] as usize;
                let read_1: usize = instructions[location + 1] as usize;
                let read_2: usize = instructions[location + 2] as usize;
                instructions[destination] = instructions[read_1] * instructions[read_2]
            }
            _ => return vec![-1],
        }
        location += 4;
        next_instruction = instructions[location];
    }
    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_requirement_for_weight() {
        assert_eq!(
            compute(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert_eq!(compute(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(compute(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(compute(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            compute(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}

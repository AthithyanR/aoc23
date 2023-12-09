use std::fs;

#[derive(Debug)]
struct Num {
    num: usize,
    pos: (isize, (isize, isize)) // (row, (col_start, col_end))
}

pub fn main() {
    let file = fs::read_to_string("src/day3/input").unwrap();
    let lines = file.lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_length = lines.len() as isize;
    println!("max_length {}", max_length);

    let mut nums: Vec<Num> = vec![];
    for (row_idx, line) in lines.iter().enumerate() {
        let mut num_str = String::new();
        let mut num_str_start: Option<isize> = None;

        let row_idx = row_idx as isize;
        for (col_idx, char) in line.iter().enumerate() {
            let col_idx = col_idx as isize;
            if char.is_numeric() {
                if num_str_start.is_none() {
                    num_str_start = Some(col_idx);
                }
                num_str.push(*char);
            } else {
                if num_str_start.is_some() {
                    let num = Num {
                        num: num_str.parse().unwrap(),
                        pos: (row_idx, (num_str_start.take().unwrap(), col_idx - 1))
                    };
                    nums.push(num);
                    num_str.clear();
                }
            }
        }
        if num_str_start.is_some() {
            let num = Num {
                num: num_str.parse().unwrap(),
                pos: (row_idx, (num_str_start.take().unwrap(), max_length - 1))
            };
            nums.push(num);
            num_str.clear();
        }
    }

    let mut sum: usize = 0;
    for num in nums.iter() {
        let rows_to_check = vec![num.pos.0 - 1, num.pos.0, num.pos.0 + 1];
        let col_idxs = ((num.pos.1.0 - 1)..=(num.pos.1.1 + 1)).collect::<Vec<_>>();
        // println!("idxs to check for num {} - {:?}", num.num, idxs_to_check);

        for row_idx in rows_to_check.iter() {
            for col_idx in col_idxs.iter() {
                if row_idx < &0 || row_idx >= &max_length || col_idx < &0 || col_idx >= &max_length {
                    continue;
                }
                let char = lines[*row_idx as usize][*col_idx as usize];
                if char != '.' && !char.is_numeric() {
                    // println!("Is valid num, {}", num.num);
                    sum += num.num;
                } 
            }
        }
    }

    println!("The sum is {sum}")
}
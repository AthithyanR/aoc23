use std::fs;

#[derive(Debug, Clone)]
struct Num {
    num: usize,
    pos: (isize, (isize, isize)), // (row, (col_start, col_end))
    read: bool
}

#[derive(Debug)]
struct Data {
    lines: Vec<Vec<char>>,
    nums: Vec<Num>,
}

impl Data {
    fn new(file: String) -> Self {
        Self {
            lines: file
                .lines()
                .collect::<Vec<_>>()
                .iter()
                .map(|line| line.chars().collect())
                .collect(),
            nums: vec![],
        }
    }

    fn load_nums(&mut self) {
        self.lines.iter().enumerate().for_each(|(row_idx, line)| {
            let mut num_str = String::new();
            let mut num_str_start: Option<isize> = None;

            let row_idx = row_idx as isize;
            line.iter().enumerate().for_each(|(col_idx, char)| {
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
                            pos: (row_idx, (num_str_start.take().unwrap(), col_idx - 1)),
                            read: false
                        };
                        self.nums.push(num);
                        num_str.clear();
                    }
                }
            });

            // num leading upto EOL
            if num_str_start.is_some() {
                let num = Num {
                    num: num_str.parse().unwrap(),
                    pos: (
                        row_idx,
                        (num_str_start.take().unwrap(), self.lines.len() as isize - 1),
                    ),
                    read: false,
                };
                self.nums.push(num);
                num_str.clear();
            }
        });
    }
}

// part 2
pub fn main() {
    let file = fs::read_to_string("src/day3/input").unwrap();
    let mut data = Data::new(file);
    data.load_nums();
   
   let max_len = data.lines.len() as isize;
   let mut total_sum: usize = 0;
    data.lines.iter().enumerate().for_each(|(row_idx, line)| {
        let row_idx = row_idx as isize;
        line.iter().enumerate().for_each(|(col_idx, char)| {
            let col_idx = col_idx as isize;
            if char == &'*' {
                // println!("found a gear at {row_idx}, {col_idx}");

                let mut connected_nums: Vec<Num> = vec![];

                let rows_to_check = vec![row_idx - 1, row_idx, row_idx + 1];
                let cols_to_check = vec![col_idx - 1, col_idx, col_idx + 1];

                rows_to_check.iter().for_each(|row_idx| {
                    cols_to_check.iter().for_each(|col_idx| {
                        if (row_idx < &0 || row_idx >= &max_len)
                            || (col_idx < &0 || col_idx >= &max_len) {
                            return;
                        }

                        data.nums.iter_mut().for_each(|num| {
                            if !num.read && &num.pos.0 == row_idx && (num.pos.1.0..=num.pos.1.1).contains(&col_idx) {
                                // println!("matched {:#?}", num);
                                num.read = true;
                                connected_nums.push(num.clone());
                            }
                        });
                    });
                });

                if connected_nums.len() != 2 {
                    return;
                }

                total_sum += connected_nums.first().unwrap().num * connected_nums.last().unwrap().num;
            }
        })
    });
// 223667365
    println!("The sum is {total_sum}");
}

// part 1
// pub fn main() {
//     let file = fs::read_to_string("src/day3/input").unwrap();
//     let lines = file.lines()
//         .collect::<Vec<_>>()
//         .iter()
//         .map(|line| line.chars().collect::<Vec<_>>())
//         .collect::<Vec<_>>();

//     let max_length = lines.len() as isize;
//     println!("max_length {}", max_length);

//     let mut nums: Vec<Num> = vec![];
//     for (row_idx, line) in lines.iter().enumerate() {
//         let mut num_str = String::new();
//         let mut num_str_start: Option<isize> = None;

//         let row_idx = row_idx as isize;
//         for (col_idx, char) in line.iter().enumerate() {
//             let col_idx = col_idx as isize;
//             if char.is_numeric() {
//                 if num_str_start.is_none() {
//                     num_str_start = Some(col_idx);
//                 }
//                 num_str.push(*char);
//             } else {
//                 if num_str_start.is_some() {
//                     let num = Num {
//                         num: num_str.parse().unwrap(),
//                         pos: (row_idx, (num_str_start.take().unwrap(), col_idx - 1))
//                     };
//                     nums.push(num);
//                     num_str.clear();
//                 }
//             }
//         }
//         if num_str_start.is_some() {
//             let num = Num {
//                 num: num_str.parse().unwrap(),
//                 pos: (row_idx, (num_str_start.take().unwrap(), max_length - 1))
//             };
//             nums.push(num);
//             num_str.clear();
//         }
//     }

//     let mut sum: usize = 0;
//     for num in nums.iter() {
//         let rows_to_check = vec![num.pos.0 - 1, num.pos.0, num.pos.0 + 1];
//         let col_idxs = ((num.pos.1.0 - 1)..=(num.pos.1.1 + 1)).collect::<Vec<_>>();
//         // println!("idxs to check for num {} - {:?}", num.num, idxs_to_check);

//         for row_idx in rows_to_check.iter() {
//             for col_idx in col_idxs.iter() {
//                 if row_idx < &0 || row_idx >= &max_length || col_idx < &0 || col_idx >= &max_length {
//                     continue;
//                 }
//                 let char = lines[*row_idx as usize][*col_idx as usize];
//                 if char != '.' && !char.is_numeric() {
//                     // println!("Is valid num, {}", num.num);
//                     sum += num.num;
//                 }
//             }
//         }
//     }

//     println!("The sum is {sum}")
// }

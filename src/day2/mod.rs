use std::{
    fs,
    collections:: HashMap
};

// part-1
// pub fn main() {
//     let max_map: HashMap<&str, i32> = HashMap::from([
//         ("red", 12),
//         ("green", 13),
//         ("blue", 14),
//     ]);

//     let input = fs::read_to_string("./src/day2/input").unwrap();
//     let lines: Vec<&str> = input.lines().collect();
//     let mut game_num_sum = 0;

//     'line_loop: for (idx, line) in lines.iter().enumerate() {
//         let splitted_line: Vec<&str> = line.split(": ").collect();
//         let groups = splitted_line[1];
//         for group in groups.split("; ").collect::<Vec<&str>>() {
//             for group_color in group.split(", ").collect::<Vec<&str>>() {
//                 let final_str: Vec<&str> = group_color.split(" ").collect();
//                 let count: i32 = final_str[0].parse().unwrap();
//                 let color_name = final_str[1];

//                 if count > *max_map.get(color_name).unwrap() {
//                     continue 'line_loop;
//                 }
//             }
//         }

//         let game_num = idx + 1;
//         println!("Adding game - {}", game_num);
//         game_num_sum += game_num;
//     }

//     println!("game_num_sum - {}", game_num_sum);
// }

// part-2
pub fn main() {
    let input = fs::read_to_string("./src/day2/input").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut game_sum = 0;

    for line in lines {
        let mut color_max_map: HashMap<&str, u32> = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        let groups = line.split(": ").collect::<Vec<&str>>()[1];
        for group in groups.split("; ").collect::<Vec<&str>>() {
            for group_color in group.split(", ").collect::<Vec<&str>>() {
                let count_color_str: Vec<&str> = group_color.split(" ").collect();
                let count: u32 = count_color_str[0].parse().unwrap();
                let color_name = count_color_str[1];

                let existing_color_max = color_max_map.get(color_name).unwrap();
                if count > *existing_color_max {
                    color_max_map.insert(color_name, count);
                }
            }
        }

        let color_max_product: u32 = color_max_map.values().product();
        game_sum += color_max_product;
    }

    println!("game_sum - {}", game_sum);
}
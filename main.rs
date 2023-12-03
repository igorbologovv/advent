use crate::io::BufReader;
use std::fs::File;
use std::io::{self, BufRead};

fn str_to_num(st: &str) -> i32 {
    st.trim().parse::<i32>().unwrap()
}
fn extract_game_number(line: &String) -> u32 {
    let parts: Vec<&str> = line.split(':').collect();

    let extract_game: Vec<&str> = parts[0].trim().split(" ").collect();

    //print!("{:?}", extract_game);
    let game_number = extract_game[1].trim().parse::<u32>().unwrap();
    game_number
}

fn extract_game_data(line: &String) -> Vec<&str> {
    let parts: Vec<&str> = line.split(':').collect();

    let second_part: Vec<&str> = parts[1].trim().split(";").collect();

    second_part
}

fn is_bigger(
    cube_color: &str,
    etalon_color: &str,
    current_maximum: i32,
    number: &str,
) -> Option<i32> {
    if cube_color == etalon_color {
        // println!("{:?} == {:?}", cube_color, etalon_color);
        let num = str_to_num(number);
        if num > current_maximum {
            // println!("{:?} > {:?}", num, possible_amount);
            return Some(num);
        } else {
            return None;
        }
    }

    None
}

fn main() -> io::Result<()> {
    let file = File::open("/home/peteshko/advent_of_code/cubes.txt")?;
    let reader = BufReader::new(file);

    let mut sum_of_products = 0;
    for line in reader.lines() {
        let line = line?;

        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        if line != "" {
            // "num color, num color", "num color, num color", "num color, num color";
            let game_data = extract_game_data(&line);

            for game in game_data {
                //"num color", "num color";
                let num_color_list: Vec<&str> = game.split(",").collect();

                for n_c in num_color_list {
                    //"num", "color"
                    let num_color_vec: Vec<&str> = n_c.trim().split(" ").collect();

                    if let Some(val) = is_bigger(num_color_vec[1], "red", red_max, num_color_vec[0])
                    {
                        red_max = val;
                    }

                    if let Some(val) =
                        is_bigger(num_color_vec[1], "green", green_max, num_color_vec[0])
                    {
                        green_max = val;
                    }

                    if let Some(val) =
                        is_bigger(num_color_vec[1], "blue", blue_max, num_color_vec[0])
                    {
                        blue_max = val;
                    }
                }
            }
        }
        sum_of_products += red_max * green_max * blue_max;
    }
    println!("Sum of Game Numbers: {}", sum_of_products);
    Ok(())
}

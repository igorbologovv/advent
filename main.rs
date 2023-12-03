use crate::io::BufReader;
use std::fs::File;
use std::io::{self, BufRead};

fn open_file(path_to_file: &str) -> io::Result<BufReader<File>> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

fn is_string_numeric(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}

fn process_lines(reader: BufReader<File>) -> Vec<Vec<String>> {
    let mut two_d_array: Vec<Vec<String>> = Vec::new();

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let characters: Vec<String> = line.chars().map(|c| c.to_string()).collect();
                two_d_array.push(characters);
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }

    two_d_array
}

fn get_neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    // Check left
    if col > 0 {
        neighbors.push((row, col - 1));
    }

    // Check right
    if col < cols - 1 {
        neighbors.push((row, col + 1));
    }

    // Check up
    if row > 0 {
        neighbors.push((row - 1, col));
    }

    // Check down
    if row < rows - 1 {
        neighbors.push((row + 1, col));
    }

    // Check diagonals
    if row > 0 && col > 0 {
        neighbors.push((row - 1, col - 1)); // Top-left
    }
    if row > 0 && col < cols - 1 {
        neighbors.push((row - 1, col + 1)); // Top-right
    }
    if row < rows - 1 && col > 0 {
        neighbors.push((row + 1, col - 1)); // Bottom-left
    }
    if row < rows - 1 && col < cols - 1 {
        neighbors.push((row + 1, col + 1)); // Bottom-right
    }

    neighbors
}

fn main() {
    let path_to_file = "/home/peteshko/advent_of_code/engine_number.txt";
    let mut two_d_array: Vec<Vec<String>> = Vec::new();
    match open_file(path_to_file) {
        Ok(reader) => {
            two_d_array = process_lines(reader);
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
        }
    }
    let mut numbers_to_sum = 0;

    let mut number_seq: String = "".to_string();

    let mut check_list: Vec<(usize, usize)> = Vec::new();

    for (row_idx, line) in two_d_array.iter().enumerate() {
        for (col_idx, symbol) in line.iter().enumerate() {
            if symbol == "*" {
                let mut neighbor_product: Vec<i32> = Vec::new();

                // It means we found some symbol, now get all values, which are nex to it.
                let neighbors = get_neighbors(row_idx, col_idx, two_d_array.len(), line.len());

                for neighbor in neighbors.clone() {
                    if let Some(row) = two_d_array.get(neighbor.0) {
                        if let Some(cell) = row.get(neighbor.1) {
                            //Check if the cell contains a numeric value
                            if cell.chars().all(char::is_numeric) {
                                // Check left and right for adjacent numbers
                                let mut left_index = neighbor.1;
                                let mut right_index = neighbor.1;
                                // Try to find number in checklist
                                if !check_list.contains(&(neighbor)) {
                                    // Check left
                                    while left_index > 0
                                        && row[left_index - 1].chars().all(char::is_numeric)
                                    {
                                        left_index -= 1;
                                        if neighbors.clone().contains(&(neighbor.0, left_index)) {
                                            let pos = (neighbor.0, left_index);
                                            check_list.push(pos);
                                        }
                                    }

                                    // Check right
                                    while right_index < row.len() - 1
                                        && row[right_index + 1].chars().all(char::is_numeric)
                                    {
                                        right_index += 1;
                                        if neighbors.clone().contains(&(neighbor.0, right_index)) {
                                            let pos = (neighbor.0, right_index);
                                            check_list.push(pos);
                                        }
                                    }

                                    // Collect numbers which we found into a srring
                                    let concatenated_number: String =
                                        row[left_index..=right_index].iter().cloned().collect();

                                    // Do something with the concatenated number
                                    neighbor_product
                                        .push(concatenated_number.parse::<i32>().unwrap())
                                }
                            }
                        }
                    }
                }

                if neighbor_product.len() == 2 {
                    numbers_to_sum += neighbor_product[0] * neighbor_product[1];
                }
            }
        }
    }
    println!("Concatenated Number: {}", numbers_to_sum);
}

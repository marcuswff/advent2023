use super::utils::read_file;
use::std::collections::HashMap;

const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/input03.txt";
// const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/test/input03.txt";

pub fn task1() {
    let data = read_file(PATH);

    let map = data
        .iter()
        .map(|s| s
            .chars()
            .collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let result = get_engine_sum(&map);

    println!("{:?}", result);
 }

pub fn task2() {
    let data = read_file(PATH);

    let map = data
        .iter()
        .map(|s| s
            .chars()
            .collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let result: u32 = get_gear_sum(&map);

    println!("{:?}", result);
}

fn get_gear_sum(map: &Vec<Vec<char>>) -> u32 {
    let directions: Vec<Direction> = get_direction();
    let mut sum = 0;

    for (x, row) in map.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if cell == &'*' {
                sum += get_gear_product(&x, &y, &directions, map);
            }
        }
    }
    sum
}

fn get_gear_product(x: &usize, y: &usize, directions: &Vec<Direction>, map: &Vec<Vec<char>>) -> u32 {
    const RADIX: u32 = 10;
    let row_length = map.len();
    let column_length = map[0].len();

    let mut adjacent_numbers: HashMap<(usize,usize), u32> = HashMap::from([]);

    for direction in directions {

        let x_check = i32::try_from(*x).unwrap() + direction.x;
        let y_check = i32::try_from(*y).unwrap() + direction.y;

        if valid_position(&x_check, &row_length) && valid_position(&y_check, &column_length) {
            let cell_check = map[x_check as usize][y_check as usize];

            if cell_check.is_digit(RADIX) {
                let (i, number) = get_whole_number(&(x_check as usize), &(y_check as usize), &column_length, &map);
                adjacent_numbers.entry(((x_check as usize), i)).or_insert(number);
            }
        }
    }

    let mut gear_product = 0;
    if adjacent_numbers.values().len() == 2 {
        gear_product = 1;
        for (_k, v) in adjacent_numbers{
            gear_product *= v;
        }
    }
    gear_product
}

fn get_whole_number(x: &usize, y: &usize, column_length: &usize, map: &Vec<Vec<char>>) -> (usize, u32) {
    const RADIX: u32 = 10;

    let mut y_current = y.clone();
    let mut current_cell: char;
    let mut add = 1;

    while valid_position(&(y_current as i32), column_length) {
        current_cell = map[*x][y_current];
        if current_cell.is_digit(RADIX) {
            if y_current > 0 {
                y_current -= 1;
            } else {
                add = 0;
                break;
            }
        } else {
            break
        }
    }

    y_current += add;
    let y_start = y_current.clone();
    let mut number: String = "".to_string();

    while valid_position(&(y_current as i32), column_length) {
        current_cell = map[*x][y_current];
        if current_cell.is_digit(RADIX) {
            number.push(current_cell);
            y_current += 1;
        } else {
            break;
        }
    }

    (y_start, number.parse::<u32>().unwrap())
}

fn get_engine_sum(map: &Vec<Vec<char>>) -> i32 {
    let row_length = map.len();
    let column_length = map[0].len();
    let directions: Vec<Direction> = get_direction();

    let mut sum = 0;
    let mut current_number = "".to_string();
    let mut found_symbol = false;

    for (x, row) in map.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if cell == &'.' || !cell.is_numeric() {
                update_sum(&current_number, &mut sum, &found_symbol);
                current_number = "".to_string();
                found_symbol = false;
            }
            if cell.is_numeric() {
                current_number.push(*cell);
                if !found_symbol {
                    check_for_symbols(&x, &y, &mut found_symbol, &directions, &row_length, &column_length, &map);
                }
            }
        }
    };
    sum
}

fn update_sum(current_number: &String, sum: &mut i32, found_symbol: &bool) {
    if current_number == "" {
        return 
    }
    if *found_symbol {
        let value: i32 = current_number.parse().unwrap();
        *sum += value;
    }
}

fn check_for_symbols(x: &usize, y: &usize, found_symbol: &mut bool, directions: &Vec<Direction>, row_length: &usize, column_length: &usize, map: &Vec<Vec<char>>) {
    for direction in directions {
        let x_check = i32::try_from(*x).unwrap() + direction.x;
        let y_check = i32::try_from(*y).unwrap() + direction.y;

        if valid_position(&x_check, row_length) && valid_position(&y_check, column_length) {
            let cell_check = map[x_check as usize][y_check as usize];
            if !cell_check.is_numeric() && cell_check != '.' {
                *found_symbol = true;
            }
        }
    }
}

fn valid_position(pos: &i32, max_value: &usize) -> bool {
    pos >= &0 && pos < &i32::try_from(*max_value).unwrap() 
}

fn get_direction() -> Vec<Direction> {
    let mut vec = vec![];
    for x in vec![-1, 0, 1] {
        for y in vec![-1, 0, 1] {
            vec.push(
                Direction {
                    x: x,
                    y: y
                }
            )
        }
    }
    vec
}
struct Direction {
    x: i32,
    y: i32,
}
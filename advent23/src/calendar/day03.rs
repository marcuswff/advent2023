use super::utils::read_file;

const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/input03.txt";
// const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/test/input02.txt";

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


}

fn get_engine_sum(map: &Vec<Vec<char>>) -> i32 {
    let row_length = map.len();
    let column_length = map[0].len();

    let directions = get_direction();

    let mut sum = 0;
    let mut current_number = "".to_string();
    let mut found_symbol = false;

    for (x, row) in map.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if cell == &'.' || !cell.is_numeric() {
                update_sum(&mut current_number, &mut sum, &mut found_symbol);
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

fn update_sum(current_number: &mut String, sum: &mut i32, found_symbol: &mut bool) {
    if current_number == "" {
        return 
    }
    if *found_symbol {
        let value: i32 = current_number.parse().unwrap();
        *sum += value;
    }
    *current_number = "".to_string();
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
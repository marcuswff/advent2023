use super::utils::read_file;
use std::collections::HashMap;

const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/input02.txt";
// const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/test/input02.txt";

pub fn task1() {
    let data = read_file(PATH);

    let games = data
        .iter()
        .map(|g| create_game(g))
        .collect::<Vec<Game>>();

    let max_cubes: HashMap<CubeType, usize> = HashMap::from([
        (CubeType::Red, 12),
        (CubeType::Green, 13),
        (CubeType::Blue, 14),
    ]);
 
    let res: usize = games
        .iter()
        .filter(|g| game_is_possible(&g, &max_cubes))
        .map(|g| g.id)
        .sum();

    println!("{:?}", res);
}

pub fn task2() {

    let data = read_file(PATH);

    let games = data
        .iter()
        .map(|g| create_game(g))
        .collect::<Vec<Game>>();

    let sum: usize = games
        .iter()
        .map(|g| get_power(&g))
        .sum();

    println!("{:?}", sum);

}

fn get_power(game: &Game) -> usize {
    let mut max_cube: HashMap<CubeType, usize> = HashMap::from([
        (CubeType::Red, 0),
        (CubeType::Green, 0),
        (CubeType::Blue, 0),
    ]);

    for round in &game.round {
        for cube in round {
            if max_cube.get(&cube.cube).unwrap() < &cube.count {
                *max_cube.get_mut(&cube.cube).unwrap() = cube.count;
            }
        }
    }

    let mut power: usize = 1;
    for (_k,v) in max_cube {
        power *= v;
    }
    
    power
}

fn game_is_possible(game: &Game, max_cubes: &HashMap<CubeType, usize>) -> bool {
    for round in &game.round {
        for cube in round {
            if max_cubes.get(&cube.cube).unwrap() < &cube.count {
                return false
            }
        }
    }
    true
}

fn create_game(game_string: &str) -> Game {

    let collection = game_string.split(":").collect::<Vec<&str>>();
    let id: usize = collection[0]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let round = get_rounds(collection[1]);

    Game {
        id: id as usize, 
        round: round
    }

}

fn get_rounds(round_string: &str) -> Vec<Vec<Cube>> {
    round_string
        .split(";")
        .collect::<Vec<&str>>()
        .iter()
        .map(|v| v
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|i| get_cube(i))
            .collect::<Vec<Cube>>()
        )
        .collect::<Vec<Vec<Cube>>>()

}

fn get_cube(cube_string: &str) -> Cube {
    let vec = cube_string
        .split_whitespace()
        .collect::<Vec<&str>>();

    Cube {
        count: vec.get(0).unwrap().parse().unwrap(),
        cube: get_cube_type(vec.get(1).unwrap()),
    }
}

fn get_cube_type(cube_type_string: &str) -> CubeType {
    match cube_type_string {
        "blue" => CubeType::Blue,
        "red" => CubeType::Red,
        "green" => CubeType::Green,
        _ => panic!("Unsupported cube")
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
enum CubeType {
    Blue,
    Red,
    Green,
}

#[derive(Debug, Clone)]
struct Cube {
    cube: CubeType,
    count: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    round: Vec<Vec<Cube>>,
}



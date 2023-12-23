use super::utils::read_file;
const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/input07.txt";
// const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/test/input06.txt";

pub fn task1() {
    let data = read_file(PATH);
   
}

pub fn task2() {
     
}



fn get_data(vec: &Vec<String>) -> Vec<Race> {
    let vec = vec
        .iter()
        .map(|s| 
            s.split(":")
            .last()
            .unwrap()
            .split_whitespace()
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
        )            
        .collect::<Vec<Vec<usize>>>();

    vec[0]
        .iter()
        .zip(vec[1].iter())
        .map(| (t, d) | 
            Race {
                time: *t,
                distance: *d
            })
        .collect::<Vec<Race>>()
}

fn get_data_2(vec: &Vec<String>) -> Race {
    let vec = vec
        .iter()
        .map(|s| 
            s.split(":")
            .last()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()
        )            
        .collect::<Vec<Vec<&str>>>();

    Race {
        time: vec[0].join("").parse::<usize>().unwrap(),
        distance: vec[1].join("").parse::<usize>().unwrap(),
    }
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize, 
}
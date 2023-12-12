use super::utils::read_file;
const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/input05.txt";
// const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/test/input05.txt";

pub fn task1() {
    let data = read_file(PATH);
    let seeds = get_seeds_data(data);

    let result = seeds
        .seed
        .iter()
        .map(|s| seeds.map
            .iter()
            .fold(*s, |seed, map| map.get_map(&seed))
        )
        .min().unwrap();

    println!("{:?}", result);
 }

pub fn task2() {
    let data: Vec<String> = read_file(PATH);
    let seeds = get_seeds_data(data);

    let seed_range = seeds.seed
        .chunks(2)
        .map(|c| SeedRange{
            start: c[0], 
            end: c[0] + c[1],
        })
        .collect::<Vec<SeedRange>>();

    for i in 0..10000000 {
        let reverse_seed = seeds.get_reverse_map(&i);
        for seed in &seed_range {
            if seed.is_in_range(&reverse_seed) {
                println!("{}", i);
                return
            }
        }
    }

    println!("No number found");
}

fn get_seeds_data(data: Vec<String>) -> Seed {
    let mut iterator = data.iter().peekable();
    let line = iterator.next().unwrap();
    let seed_numbers: Vec<usize> = get_seed_numbers(line);

    iterator.next();

    let mut map: Vec<MapRange> = Vec::from([]);

    while iterator.peek().is_some() {

        let _ = iterator.next().unwrap().as_str();
        let mut vec_range: Vec<Range> = Vec::from([]);

        while iterator.peek().is_some() {
            if iterator.peek().unwrap() == &&"".to_string() {
                iterator.next();
                break
            }
            vec_range.push(
                get_range(iterator.next().unwrap())
            )
        }
        map.push(
            MapRange {
                    range: vec_range,
                }
        );
    }
    Seed {
        seed: seed_numbers,
        map: map,
    }
}

fn get_range(line: &str) -> Range {
    let line = line
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

        Range { 
            source: line[1],
            destination: line[0],
            length: line[2],
        }
}

fn get_seed_numbers(s: &str) -> Vec<usize> {
    let numbers = s
        .split(":")
        .collect::<Vec<&str>>()[1];
    
    get_vector_from_string(numbers)
}

fn get_vector_from_string(numbers: &str) -> Vec<usize> {
    numbers
    .split_whitespace()
    .collect::<Vec<&str>>()
    .iter()
    .map(|s| s.parse::<usize>().unwrap())
    .collect::<Vec<usize>>()
}

#[derive(Debug)]
struct MapRange {
    range: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    source: usize,
    destination: usize,
    length: usize,
}

#[derive(Debug)]
struct SeedRange {
    start: usize,
    end: usize,
}

struct Seed {
    seed: Vec<usize>,
    map: Vec<MapRange>,
}

impl Seed {
    fn get_reverse_map(&self, seed: &usize) -> usize {
        let mut temp_seed = *seed;
        for map_range in self.map.iter().rev() {
            temp_seed = map_range.get_reverse_map(&temp_seed);
        }
        temp_seed
    }
}

impl Range {
    fn source_to_destination(&self, seed: usize) -> usize {
        seed - &self.source + &self.destination
    }

    fn destination_to_source(&self, seed: &usize) -> usize {
        seed - &self.destination + &self.source
    }

    fn is_in_range(&self, seed: &usize) -> bool {
        &self.source <= seed && seed < &( self.source + self.length ) 
    }

    fn is_in_range_reverse(&self, seed: &usize) -> bool {
        &self.destination <= seed && seed < &( self.destination + self.length ) 
    }
}

impl MapRange {
    fn get_map(&self, seed: &usize) -> usize {
        for range in &self.range {
            if range.is_in_range(seed) {
                return range.source_to_destination(*seed);
            }
        }
        *seed
    }

    fn get_reverse_map(&self, seed: &usize) -> usize {
        for range in &self.range {
            if range.is_in_range_reverse(seed) {
                return range.destination_to_source(seed);
            }
        }
        *seed
    }
}

impl SeedRange {
    fn  is_in_range(&self, seed: &usize) -> bool {
        &self.start <= seed && seed < &self.end
    }
}
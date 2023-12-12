use super::utils::read_file;
use::std::collections::HashSet;
use::std::collections::HashMap;

const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/input04.txt";
// const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/test/input04.txt";

pub fn task1() {
    let data = read_file(PATH);

    let vec = data
        .iter()
        .map(|s| str_to_cards(&s.as_str()))
        .collect::<Vec<CardGame>>();
        
    let set: usize = vec
        .iter()
        .map(|g| g.intersection())
        .collect::<Vec<HashSet<&usize>>>()
        .iter()
        .map(|h| calculate_sum(h))
        .sum();

    println!("{:?}", set);
 }

pub fn task2() {
    let data: Vec<String> = read_file(PATH);

    let vec = data
        .iter()
        .map(|s| str_to_cards(&s.as_str()))
        .collect::<Vec<CardGame>>();

    let mut map: HashMap<usize, usize> = HashMap::from([]);
    let mut card_count = 0;

    for game in vec {

        let number_of_wins = game.intersection().len();
        map.entry(game.id).or_insert(0);

        for i in (game.id+1)..(game.id + number_of_wins+1) {
            *map.entry(i).or_insert(0) += 1 as usize + map.get(&game.id).unwrap();
        }   
        card_count += 1 + map.get(&game.id).unwrap();
    }

    println!("{:?}", card_count);

}

fn calculate_sum(winning_cards: &HashSet<&usize>) -> usize {
    if winning_cards.len() > 0 {
        let base: usize = 2;
        return base.pow((winning_cards.len() - 1) as u32)
    }
    0
}


fn str_to_cards(s: &str) -> CardGame {
    let vec = s.split(":").collect::<Vec<&str>>();
    let cards = vec[1].split("|").collect::<Vec<&str>>();
    let id = vec[0]
        .split_whitespace()
        .collect::<Vec<&str>>()[1]
            .parse::<usize>().unwrap();

    CardGame { 
        id: id, 
        winning_cards: vec_to_card(&cards[0]), 
        play_cards: vec_to_card(&cards[1]), 
    }
}

fn vec_to_card(s: &str) -> HashSet<usize> {
    s
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s
            .parse::<usize>()
            .unwrap())
        .collect::<HashSet<usize>>()
}

struct CardGame {
    id: usize,
    winning_cards: HashSet<usize>,
    play_cards: HashSet<usize>,
}

impl CardGame {
    fn intersection (&self) -> HashSet<&usize> {
        self.winning_cards.intersection(&self.play_cards).collect()
    }
}
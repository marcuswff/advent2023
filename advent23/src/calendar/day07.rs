// use super::utils::read_file;
// use::std::collections::HashMap;

// const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/input07.txt";
// // const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/test/input06.txt";

// pub fn task1() {
//     let data = read_file(PATH);

//     let hand_vec = data
//         .iter()
//         .map(|s| 
//             s.split_whitespace()
//             .collect::<Vec<&str>>()
//             .map(|v| 
//                 Hand {
//                     cards: v.next().unwrap(),
//                     bid: v.next().unwrap(),
//                     rank: 0,
//                 }
//             )            
//         );

//     println!("{:?}", data);        
// }

// pub fn task2() {

// }

// fn get_card_score(card: &char) {
// }

// #[derive(PartialEq, Eq)]
// struct Hand {
//     cards: String,
//     bid: usize,
//     rank: usize,
// }

// fn get_card_rank(card: &char) -> usize {
//     let chars = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
//     let mut hash_map = HashMap::new();

//     for (i, c) in chars.iter().rev().enumerate() {
//         hash_map.entry(c).or_insert(i);
//     }
//     *hash_map.get(card).unwrap()
// }

// impl Ord for Hand {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         // Compare cards lexicographically
//         for i in 0..self.cards.len() {
//             let card = self.cards.chars().nth(i).unwrap();
//             let other_card = other.cards.chars().nth(i).unwrap();

//             if card != other_card {
//                 match card < other_card {
//                     true => return std::cmp::Ordering::Less,
//                     false => return std::cmp::Ordering::Greater,

//                 };
//             }
//         }
//         panic!("Failed Comparison");
//         // std::cmp::Ordering::Equal
//     }
// }

// impl PartialOrd for Hand {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }
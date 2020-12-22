use std::collections::{HashSet, VecDeque};
use std::fs;

type Deck = VecDeque<usize>;

#[derive(Debug, Eq, PartialEq)]
enum Winner {
    P1,
    P2,
}

fn load_players(input: &str) -> (Deck, Deck) {
    let players: Vec<&str> = input.split("\n\n").collect();

    let player1: Deck = players[0]
        .lines()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let player2: Deck = players[1]
        .lines()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    (player1, player2)
}

fn combat(player1: &mut Deck, player2: &mut Deck) -> usize {
    while !player1.is_empty() && !player2.is_empty() {
        if player1[0] > player2[0] {
            player1.rotate_left(1);
            player1.push_back(player2.pop_front().unwrap());
        } else {
            player2.rotate_left(1);
            player2.push_back(player1.pop_front().unwrap());
        }
    }
    if player1.is_empty() {
        score(&player2)
    } else {
        score(&player1)
    }
}

fn score(d: &Deck) -> usize {
    d.iter().rev().enumerate().map(|(i, x)| (i + 1) * x).sum()
}

fn recursive_combat(player1: &mut Deck, player2: &mut Deck) -> usize {
    recursive_combat_game(player1.clone(), player2.clone()).1
}

fn recursive_combat_game(mut player1: Deck, mut player2: Deck) -> (Winner, usize) {
    let mut seen: HashSet<(Deck, Deck)> = HashSet::new();

    loop {
        // loop check
        if seen.contains(&(player1.clone(), player2.clone())) {
            return (Winner::P1, score(&player1));
        }
        // game over
        if player2.is_empty() {
            return (Winner::P1, score(&player1));
        }
        if player1.is_empty() {
            return (Winner::P2, score(&player2));
        }
        seen.insert((player1.clone(), player2.clone()));

        // recursive
        if player1.len() > player1[0] && player2.len() > player2[0] {
            let new1 = player1
                .iter()
                .skip(1)
                .take(player1[0])
                .map(|&x| x)
                .collect();

            let new2 = player2
                .iter()
                .skip(1)
                .take(player2[0])
                .map(|&x| x)
                .collect();

            match recursive_combat_game(new1, new2) {
                (Winner::P1, _) => {
                    player1.rotate_left(1);
                    player1.push_back(player2.pop_front().unwrap());
                }
                (Winner::P2, _) => {
                    player2.rotate_left(1);
                    player2.push_back(player1.pop_front().unwrap());
                }
            }
        } else {
            // regular
            if player1[0] > player2[0] {
                player1.rotate_left(1);
                player1.push_back(player2.pop_front().unwrap());
            } else {
                player2.rotate_left(1);
                player2.push_back(player1.pop_front().unwrap());
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("../inputs/day22.txt").unwrap();

    let (mut player1, mut player2) = load_players(&input);
    let score = combat(&mut player1, &mut player2);
    println!("{}", score);

    let (mut player1, mut player2) = load_players(&input);
    let score = recursive_combat(&mut player1, &mut player2);
    println!("{}", score);
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day22-test.txt").unwrap();
        let (mut player1, mut player2) = load_players(&input);

        let score = combat(&mut player1, &mut player2);

        assert_eq!(score, 306);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("../inputs/day22-test.txt").unwrap();
        let (mut player1, mut player2) = load_players(&input);

        let score = recursive_combat(&mut player1, &mut player2);

        assert_eq!(score, 291);
    }

    #[test]
    fn test_part2_loop() {
        let player1: VecDeque<usize> = vec![43, 19].into();
        let player2: VecDeque<usize> = vec![2, 29, 14].into();

        assert_eq!(recursive_combat_game(player1, player2).0, Winner::P1);
    }
}

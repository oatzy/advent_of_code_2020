use std::collections::VecDeque;
use std::fs;

type Deck = VecDeque<usize>;

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

fn main() {
    let input = fs::read_to_string("../inputs/day22.txt").unwrap();
    let (mut player1, mut player2) = load_players(&input);

    let score = combat(&mut player1, &mut player2);

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
}

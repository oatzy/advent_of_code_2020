use std::collections::{HashMap, HashSet};
use std::fs;

type A<'a> = HashMap<&'a str, HashSet<&'a str>>;

struct Menu<'a> {
    ingredients: HashMap<&'a str, usize>,
    allergens: A<'a>,
}

impl<'a> Menu<'a> {
    fn new() -> Self {
        Self {
            ingredients: HashMap::new(),
            allergens: HashMap::new(),
        }
    }

    fn add_from_string(&mut self, food: &'a str) {
        let parts: Vec<_> = food[..food.len() - 1].split(" (contains ").collect();
        let ingredients: HashSet<_> = parts[0].split(" ").collect();

        for i in ingredients.iter() {
            let count = self.ingredients.entry(i).or_insert(0);
            *count += 1;
        }

        for a in parts[1].split(", ") {
            if self.allergens.contains_key(a) {
                let current = self.allergens.remove(a).unwrap();
                self.allergens
                    .insert(a, current.intersection(&ingredients).map(|&s| s).collect());
            } else {
                self.allergens.insert(a, ingredients.clone());
            }
        }
    }

    fn non_allergenic(&self) -> usize {
        let allergenic: HashSet<_> = self.allergens.values().flatten().collect();

        self.ingredients
            .iter()
            .filter(|(k, _)| !allergenic.contains(k))
            .map(|(_, v)| v)
            .sum()
    }
}

fn deduce<'a>(allergens: &A<'a>) -> HashMap<&'a str, &'a str> {
    // identify and set fields which have only one match
    let mut deduced = HashMap::new();
    let mut allergens = allergens.clone();

    while !allergens.is_empty() {
        let mut new = allergens.clone();

        for (k, v) in allergens {
            if v.len() == 1 {
                let v = v.iter().next().unwrap();
                deduced.insert(k, *v);
                new.remove(k);

                for w in new.values_mut() {
                    w.remove(v);
                }
            }
        }
        allergens = new;
    }

    deduced
}

fn dangerous_list(allergenic: &HashMap<&str, &str>) -> String {
    let mut allergenic: Vec<_> = allergenic.iter().collect();
    allergenic.sort_by(|x, y| x.0.cmp(&y.0));
    allergenic
        .iter()
        .map(|(_, &v)| v)
        .collect::<Vec<_>>()
        .join(",")
}

fn main() {
    let mut menu = Menu::new();

    let input = fs::read_to_string("../inputs/day21.txt").unwrap();
    for line in input.lines() {
        menu.add_from_string(&line);
    }

    println!("{}", menu.non_allergenic());

    let allergenic = deduce(&menu.allergens);
    println!("{}", dangerous_list(&allergenic));
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let mut menu = Menu::new();

        let input = fs::read_to_string("../inputs/day21-test.txt").unwrap();
        for line in input.lines() {
            menu.add_from_string(&line);
        }

        assert_eq!(menu.non_allergenic(), 5);
    }

    #[test]
    fn test_part2() {
        let mut menu = Menu::new();

        let input = fs::read_to_string("../inputs/day21-test.txt").unwrap();
        for line in input.lines() {
            menu.add_from_string(&line);
        }

        let allergenic = deduce(&menu.allergens);
        assert_eq!(
            dangerous_list(&allergenic),
            "mxmxvkd,sqjhc,fvjkl".to_string()
        );
    }
}

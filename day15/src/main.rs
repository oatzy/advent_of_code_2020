use std::collections::HashMap;

fn find_nth(prelude: &Vec<usize>, n: usize) -> usize {
    let mut inx = prelude.len();
    let mut current = *prelude.last().unwrap();

    let mut last_seen: HashMap<usize, usize> = prelude[..inx - 1]
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i + 1))
        .collect();

    while inx < n {
        // if inx % 1000000 == 0 {
        //     println!("{} {}", inx, current);
        // }
        let new = match last_seen.get(&current) {
            Some(x) => inx - x,
            None => 0,
        };

        last_seen.insert(current, inx);

        inx += 1;
        current = new;
    }

    current
}

fn main() {
    let input = vec![2, 0, 1, 7, 4, 14, 18];
    // println!("{}", find_nth(&input, 2020));
    println!("{}", find_nth(&input, 30000000));
}

mod test {
    use super::*;

    #[test]
    fn test_part1_short() {
        let prelude = &vec![0, 3, 6];
        assert_eq!(find_nth(prelude, 10), 0);
    }

    #[test]
    fn test_part1_long() {
        assert_eq!(find_nth(&vec![1, 3, 2], 2020), 1);
        assert_eq!(find_nth(&vec![2, 1, 3], 2020), 10);
        assert_eq!(find_nth(&vec![1, 2, 3], 2020), 27);
        assert_eq!(find_nth(&vec![2, 3, 1], 2020), 78);
        assert_eq!(find_nth(&vec![3, 2, 1], 2020), 438);
        assert_eq!(find_nth(&vec![3, 1, 2], 2020), 1836);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(find_nth(&vec![0, 3, 6], 30000000), 175594);
        assert_eq!(find_nth(&vec![1, 3, 2], 30000000), 2578);
        assert_eq!(find_nth(&vec![2, 1, 3], 30000000), 3544142);
        assert_eq!(find_nth(&vec![1, 2, 3], 30000000), 261214);
        assert_eq!(find_nth(&vec![2, 3, 1], 30000000), 6895259);
        assert_eq!(find_nth(&vec![3, 2, 1], 30000000), 18);
        assert_eq!(find_nth(&vec![3, 1, 2], 30000000), 362);
    }
}

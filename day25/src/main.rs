static MOD: usize = 20201227;

fn find_loop_size(key: usize, subject: usize) -> usize {
    let mut i = 1;
    let mut loop_size = 0;

    while i != key {
        i = (i * subject) % MOD;
        loop_size += 1;
    }

    loop_size
}

fn get_private_key(public: usize, loop_size: usize) -> usize {
    let mut i = 1;

    for _ in 0..loop_size {
        i = (i * public) % MOD;
    }

    i
}

fn main() {
    let card_public = 11239946;
    let door_public = 10464955;

    let card_loop = find_loop_size(card_public, 7);
    let door_loop = find_loop_size(door_public, 7);

    let card_private = get_private_key(card_public, door_loop);
    let door_private = get_private_key(door_public, card_loop);

    assert_eq!(card_private, door_private);
    println!("{}", card_private);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_loop() {
        let card_public = 5764801;
        let door_public = 17807724;

        assert_eq!(find_loop_size(card_public, 7), 8);
        assert_eq!(find_loop_size(door_public, 7), 11);
    }

    #[test]
    fn test_get_public_key() {
        let card_public = 5764801;
        let door_public = 17807724;

        assert_eq!(get_private_key(card_public, 11), 14897079);
        assert_eq!(get_private_key(door_public, 8), 14897079);
    }
}

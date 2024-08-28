use rand::Rng;

#[derive(Debug, Clone)]
pub enum Puzzle {
    Cube3x3
}

pub struct Scrambler {}
impl Scrambler {
    pub fn new_scramble(p: Puzzle) -> String {
        match p {
            Puzzle::Cube3x3 => Self::scramble_3x3()
        }
    }

    fn scramble_3x3() -> String {
        let scramble_length = 20;
        let moves = ["R", "L", "U", "D", "F", "B"];
        let modifiers = ["", "'", "2"];

        let mut rng = rand::thread_rng();
        let mut scramble = Vec::with_capacity(scramble_length);
        let mut last_move = "";

        for _ in 0..scramble_length {
            let new_move = loop {
                let m = moves[rng.gen_range(0..moves.len())];
                if m != last_move {
                    break m;
                }
            };

            let modifier = modifiers[rng.gen_range(0..modifiers.len())];
            scramble.push(format!("{}{}", new_move, modifier));
            last_move = new_move;
        }

        scramble.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scramble_3x3_returns_a_20_move_scramble() {
        let scramble = Scrambler::scramble_3x3();
        let moves: Vec<_> = scramble.split_whitespace().collect();
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn test_no_consecutive_repeated_moves() {
        let scramble = Scrambler::scramble_3x3();
        let moves: Vec<_> = scramble.split_ascii_whitespace().collect();
        for window in moves.windows(2) {
            assert_ne!(window[0].chars().next(), window[1].chars().next());
        }
    }
}
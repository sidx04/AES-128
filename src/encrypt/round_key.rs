use crate::{State, NB};

pub fn add_round_key(state: &mut State, key_schedule: &[[u8; 4]]) -> State {
    for row in 0..NB {
        for col in 0..4 {
            // println!("{col}, {row}, {}", round * NB + col);
            // println!("{:02x}, {:02x}", state[row][col], key_schedule[row][col]);
            state[row][col] ^= key_schedule[row][col];
        }
    }

    return *state;
}

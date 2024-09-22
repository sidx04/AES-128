use crate::utils::{transpose, GF256};
use crate::State;

/// ! TODO: Generate Doc
pub fn mix_coloumns(state: &mut State) -> State {
    let state_copy = state.clone();
    for i in 0..4 {
        let mut temp = [0u8; 4];
        for j in 0..4 {
            temp[j] = state_copy[i][j];
        }
        state[0][i] = (GF256(temp[0]) * GF256(2))
            ^ (GF256(temp[1]) * GF256(3))
            ^ (GF256(temp[2]) * GF256(1))
            ^ (GF256(temp[3]) * GF256(1));

        state[1][i] = (GF256(temp[0]) * GF256(1))
            ^ (GF256(temp[1]) * GF256(2))
            ^ (GF256(temp[2]) * GF256(3))
            ^ (GF256(temp[3]) * GF256(1));

        state[2][i] = (GF256(temp[0]) * GF256(1))
            ^ (GF256(temp[1]) * GF256(1))
            ^ (GF256(temp[2]) * GF256(2))
            ^ (GF256(temp[3]) * GF256(3));

        state[3][i] = (GF256(temp[0]) * GF256(3))
            ^ (GF256(temp[1]) * GF256(1))
            ^ (GF256(temp[2]) * GF256(1))
            ^ (GF256(temp[3]) * GF256(2));
    }
    let state = transpose(state);

    return state;
}

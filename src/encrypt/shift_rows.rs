use crate::{utils::transpose, State, NB};

fn shift_row(row: [u8; 4], shift: usize) -> [u8; 4] {
    let mut shifted_row = [0u8; 4];
    for i in 0..4 {
        shifted_row[i] = row[(i + shift) % NB];
    }
    return shifted_row;
}

/// # Shift Rows
/// Shifts the rows of a state matrix to the left by a specific number of positions.
/// It shifts the bytes in each row of the state matrix to the left by a specific number of positions, determined by the row index.
///
/// ## Parameters:
///
/// * `state`: A mutable reference to the state matrix to be modified.
///
/// ## Returns:
///
/// The state matrix after applying the `shift_rows()` operation.
///
/// ## Examples:
///
/// ```rust
/// use aes_128_round_function::State;
/// use aes_128_round_function::encrypt;
///
/// fn main() {
///     let mut state = [[0xa7, 0x61, 0xca, 0x9b],
///                      [0x97, 0xbe, 0x8b, 0x45],
///                      [0xd8, 0xad, 0x1a, 0x61],
///                      [0x1f, 0xc9, 0x73, 0x69]];
///
///     encrypt::shift_rows(&mut state);
///
///     assert_eq!(state, [[0xa7, 0x61, 0xca, 0x9b],   
///                       [0xbe, 0x8b, 0x45, 0x97],  
///                       [0x1a, 0x61, 0xd8, 0xad],   
///                       [0x69, 0x1f, 0xc9, 0x73]]);
/// }
/// ```
pub fn shift_rows(state: &mut State) -> State {
    for row_index in 0..4 {
        state[row_index] = shift_row(state[row_index], row_index);
    }
    let state = transpose(state);
    return state;
}

pub fn rot_word(word: &[u8; 4]) -> [u8; 4] {
    shift_row(*word, 1)
}

use crate::{utils::transpose, State, AES_SBOX, NB};

pub fn substitute(byte: u8) -> u8 {
    let upper_nibble: u8;
    let lower_nibble: u8;
    upper_nibble = ((byte >> 4) & 0xF).into();
    lower_nibble = (byte & 0xF).into();

    AES_SBOX[upper_nibble as usize][lower_nibble as usize]
}

/// # Sub Bytes
/// Substitutes each byte in the given state with its corresponding value from the S-box.
///
/// ## Arguments
///
/// * `state`: A mutable reference to the state to be substituted.
///
/// ## Example
///
/// ```rust
/// use aes_128_round_function::{encrypt::sub_bytes, State};
///
/// fn main() {
///     let mut state: State = [
///         [0x19, 0xa0, 0x9a, 0xe9],
///         [0x3d, 0xf4, 0xc6, 0xf8],
///         [0xe3, 0xe2, 0x8d, 0x48],
///         [0xbe, 0x2b, 0x2a, 0x08],
///     ];
///
///     sub_bytes(&mut state);
///
///     assert_eq!(state, [
///         [0xd4, 0xe0, 0xb8, 0xed],
///         [0x27, 0xbf, 0xb4, 0x41],
///         [0x11, 0x98, 0x5d, 0x52],
///         [0xae, 0xf1, 0xe5, 0x30]
///     ]);
/// }
/// ```
pub fn sub_bytes(state: &mut State) -> State {
    for row_index in 0..4 {
        for col_index in 0..NB {
            let byte = state[row_index][col_index];
            state[row_index][col_index] = substitute(byte);
        }
    }
    let state = transpose(state);
    return state;
}

pub fn sub_word(word: &mut [u8; NB]) -> [u8; NB] {
    for i in 0..NB {
        word[i] = substitute(word[i]);
    }
    return *word;
}

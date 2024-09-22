use crate::encrypt::rot_word;
use crate::encrypt::sub_word;
use crate::State;
use crate::RCON;
use crate::{Key, KeySchedule, KeyScheduleTrait, NB, NK, NR};

/// ! TODO: Generate Doc
pub fn expand_key(key: &Key) -> KeySchedule {
    let mut i;
    // let key: Key = Key::new();
    let mut keyschedule: KeySchedule = KeySchedule::new();

    for i in 0..NK {
        for j in 0..NB {
            keyschedule[i][j] = key[i][j];
        }
    }

    i = NK;

    while i < NB * (NR + 1) {
        let mut temp = keyschedule[i - 1];
        if i % NK == 0 {
            temp = sub_word(&mut rot_word(&temp));
            temp[0] ^= RCON[i / NK];
        } else if NK > 6 && i % NK == 4 {
            temp = sub_word(&mut temp);
        }

        // XOR temp with keyschedule[i - NK]
        for j in 0..NB {
            keyschedule[i][j] = keyschedule[i - NK][j] ^ temp[j];
        }

        i += 1;
    }

    return keyschedule;
}

pub fn transpose(state: &mut State) -> State {
    for i in 0..4 {
        for j in i + 1..4 {
            let temp = state[i][j];
            state[i][j] = state[j][i];
            state[j][i] = temp;
        }
    }

    *state
}

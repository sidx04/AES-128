use aes_128_round_function::encrypt::{add_round_key, mix_coloumns, shift_rows, sub_bytes};
use aes_128_round_function::utils::{expand_key, transpose};
use aes_128_round_function::State;
use aes_128_round_function::{Key, KeySchedule, NB, NR};

fn main() {
    // let msg: &str = "Hello world!";
    // println!("{:#02x?}", msg.as_bytes());

    // let mut state: State = [
    //     [0x19, 0xa0, 0x9a, 0xe9],
    //     [0x3d, 0xf4, 0xc6, 0xf8],
    //     [0xe3, 0xe2, 0x8d, 0x48],
    //     [0xbe, 0x2b, 0x2a, 0x08],
    // ];
    // println!("Initial state: {:02x?}", state);

    // let mut sub_byte_arr: State = encrypt::sub_bytes(&mut state);
    // println!("{:02x?}", sub_byte_arr);

    // let mut shift_rows_arr: State = encrypt::shift_rows(&mut sub_byte_arr);
    // println!("{:02x?}", shift_rows_arr);

    // let mix_cols_arr: State = encrypt::mix_coloumns(&mut shift_rows_arr);
    // println!("{:02x?}", mix_cols_arr);

    // let key: Key = [
    //     [0x2b, 0x7e, 0x15, 0x16], // First column
    //     [0x28, 0xae, 0xd2, 0xa6], // Second column
    //     [0xab, 0xf7, 0x15, 0x88], // Third column
    //     [0x09, 0xcf, 0x4f, 0x3c], // Fourth column
    // ];
    // let expanded_key = expand_key(&key);
    // let expanded_key_slice = &expanded_key[39..44];
    // println!("{:#02x?}", expanded_key_slice);

    // let mut state: State = [
    //     [0x00, 0x11, 0x22, 0x33],
    //     [0x44, 0x55, 0x66, 0x77],
    //     [0x88, 0x99, 0xaa, 0xbb],
    //     [0xcc, 0xdd, 0xee, 0xff],
    // ];
    // let mut key: Key = [
    //     [0x00, 0x01, 0x02, 0x03],
    //     [0x04, 0x05, 0x06, 0x07],
    //     [0x08, 0x09, 0x0a, 0x0b],
    //     [0x0c, 0x0d, 0x0e, 0x0f],
    // ];

    // let key_schedule: KeySchedule = expand_key(&key);
    // let key_schedule_slice = &key_schedule[0..4];
    // println!("Key Schedule:\t{:02x?}", &key_schedule);
    // // println!("Key Schedule:\t{:02x?}", &key_schedule_slice[0]);

    // let ark = add_round_key(&mut state, &key_schedule_slice, 0);
    // println!("After Round Key:\t{:02x?}", ark);

    let mut state: State = [
        [0x00, 0x11, 0x22, 0x33],
        [0x44, 0x55, 0x66, 0x77],
        [0x88, 0x99, 0xaa, 0xbb],
        [0xcc, 0xdd, 0xee, 0xff],
    ];

    let key: Key = [
        [0x00, 0x01, 0x02, 0x03],
        [0x04, 0x05, 0x06, 0x07],
        [0x08, 0x09, 0x0a, 0x0b],
        [0x0c, 0x0d, 0x0e, 0x0f],
    ];

    println!("Orginal state:\t{:02x?}", state);
    println!("Cipher key:\t{:02x?}", key);

    let key_schedule: KeySchedule = expand_key(&key);
    println!("Key Schedule:\t{:02x?}", &key_schedule);

    println!(
        "--------------------------------------- ROUND 0 ---------------------------------------"
    );
    let ark0 = add_round_key(&mut state, &key_schedule[0..4]);
    println!("Add-Round Key:\t{:02x?}", ark0);

    for round in 1..NR {
        println!(
            "--------------------------------------- ROUND {round} ---------------------------------------"
        );

        let s_box = sub_bytes(&mut state);
        println!("Sub-Bytes:\t{:02x?}", s_box);
        let s_row = shift_rows(&mut state);
        println!("Shift Rows:\t{:02x?}", s_row);
        let mix_col = mix_coloumns(&mut state);
        println!("Mix Coloumns:\t{:02x?}", mix_col);
        println!(
            "Key slice:\t{:02x?}",
            &key_schedule[(round * NB)..((round + 1) * NB - 1) + 1]
        );
        let ark = add_round_key(
            &mut state,
            &key_schedule[(round * NB)..((round + 1) * NB - 1) + 1],
        );
        println!("Add-Round Key:\t{:02x?}", ark);
    }
    println!(
        "--------------------------------------- ROUND 10 ---------------------------------------"
    );
    let mut state = transpose(&mut state);
    let s_box = sub_bytes(&mut state);
    println!("Sub-Bytes:\t{:02x?}", s_box);

    let mut state = transpose(&mut state);
    let s_row = shift_rows(&mut state);
    println!("Shift Rows:\t{:02x?}", s_row);

    println!(
        "Key slice:\t{:02x?}",
        &key_schedule[(10 * NB)..((10 + 1) * NB - 1) + 1]
    );

    add_round_key(
        &mut state,
        &key_schedule[(NR * NB)..((NR + 1) * NB - 1) + 1],
    );

    println!("Final state:\t{:02x?}", state);
}

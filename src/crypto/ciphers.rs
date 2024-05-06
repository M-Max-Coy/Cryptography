use crate::crypto::math_helper;

/*
Finds the inverse of a byte by mapping the byte to its corresponding element in GF(2^8)
and finding the inverse. 0 is mapped to 0.
*/
pub fn inv(byte: u8) -> u8 {
    if byte == 0 {
        return 0;
    }

    let mut res: u8 = 1;
    let mut cur: u8 = math_helper::byte_multiply(&byte,&byte);
    for i in 0..7 {
        res = math_helper::byte_multiply(&res,&cur);
        cur = math_helper::byte_multiply(&cur,&cur);
    }

    return res;
}

/*
Scales and shifts input byte with formula A(byte)+b
*/
pub fn aff(byte: u8) -> u8 {
    let mut res: u8 = 0;

    let A: [u8; 8] = [0b11110001, 0b11100011, 0b11000111, 0b10001111, 0b00011111, 0b00111110, 0b01111100, 0b11111000];

    for i in 0..8 {
        let mut bit = 0;
        for j in 0..8 {
            bit ^= (A[i] >> j & 1) & (byte >> j & 1);
        }
        res ^= bit << i
    }

    let b: u8 = 0b01100011;

    res ^= b;

    return res;
}

/*
Inverse of aff function
*/
pub fn inv_aff(byte: u8) -> u8 {
    let b: u8 = 0b01100011;

    let byte2: u8 = byte ^ b;

    let mut res: u8 = 0;

    let A: [u8; 8] = [0b01010010, 0b00101001, 0b10010100, 0b01001010, 0b00100101, 0b10010010, 0b01001001, 0b10100100];

    for i in 0..8 {
        let mut bit = 0;
        for j in 0..8 {
            bit ^= (A[i] >> j & 1) & (byte2 >> j & 1);
        }
        res ^= bit << (7-i)
    }

    return res;
}

/*
Replaces all the bytes in the state array to scramble state array data
*/
fn sub_bytes(state: &mut [[u8; 4]; 4]) {
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] = aff(inv(state[i][j]));
        }
    }
}

/*
Inverse of sub bytes
*/
fn inv_sub_bytes(state: &mut [[u8; 4]; 4]) {
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] = inv(inv_aff(state[i][j]));
        }
    }
}

/*
Combines a key with the state matrix by finding the XOR of the two matrices
*/
fn add_round_key(state: &mut [[u8; 4]; 4], key: [[u8; 4]; 4]) {
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] = state[i][j] ^ key[i][j];
        }
    }
}

/*
Scrambles state array by shifting rows across columns
*/
fn shift_rows(state: &mut [[u8; 4]; 4]) {
    for i in 0..4 {
        state[i].rotate_left(i);
    }
}

/*
Inverse of shift rows
*/
fn inv_shift_rows(state: &mut [[u8; 4]; 4]) {
    for i in 0..4 {
        state[i].rotate_right(i);
    }
}

/*
Replaces each byte in column using information from every byte in the column for diffusion
*/
fn mix_columns(state: &mut [[u8; 4]; 4]) {
    // let M: [[u8; 4]; 4] = [
    //     [0b01000000, 0b11000000, 0b10000000, 0b10000000],
    //     [0b10000000, 0b01000000, 0b11000000, 0b10000000],
    //     [0b10000000, 0b10000000, 0b01000000, 0b11000000],
    //     [0b11000000, 0b10000000, 0b10000000, 0b01000000]
    // ];

    let M: [[u8; 4]; 4] = [
        [0b00000010, 0b00000011, 0b00000001, 0b00000001],
        [0b00000001, 0b00000010, 0b00000011, 0b00000001],
        [0b00000001, 0b00000001, 0b00000010, 0b00000011],
        [0b00000011, 0b00000001, 0b00000001, 0b00000010]
    ];

    let copy = state.clone();

    for i in 0..4 {
        for j in 0..4 {
            let mut sum: u8 = 0;
            for k in 0..4 {
                sum ^= math_helper::byte_multiply(&M[i][k],&copy[k][j]);
            }
            state[i][j] = sum;
        }
    }
}

/*
Inverse of mix columns
*/
fn inv_mix_columns(state: &mut [[u8; 4]; 4]) {
    // let M: [[u8; 4]; 4] = [
    //     [0b01000000, 0b11000000, 0b10000000, 0b10000000],
    //     [0b10000000, 0b01000000, 0b11000000, 0b10000000],
    //     [0b10000000, 0b10000000, 0b01000000, 0b11000000],
    //     [0b11000000, 0b10000000, 0b10000000, 0b01000000]
    // ];

    let M: [[u8; 4]; 4] = [
        [0b00001110, 0b00001011, 0b00001101, 0b00001001],
        [0b00001001, 0b00001110, 0b00001011, 0b00001101],
        [0b00001101, 0b00001001, 0b00001110, 0b00001011],
        [0b00001011, 0b00001101, 0b00001001, 0b00001110]
    ];

    let copy = state.clone();

    for i in 0..4 {
        for j in 0..4 {
            let mut sum: u8 = 0;
            for k in 0..4 {
                sum ^= math_helper::byte_multiply(&M[i][k],&copy[k][j]);
            }
            state[i][j] = sum;
        }
    }
}

/*
Gets a column or "word" from the state array and returns it
*/
fn get_column_as_array(state: &[[u8; 4]; 4], col: usize) -> [u8; 4] {
    let mut res: [u8; 4] = [0; 4];

    for i in 0..4 {
        res[i] = state[i][col];
    }

    return res;
}

/*
Adds (XOR) two words
*/
fn word_add(w1: &[u8; 4], w2: &[u8; 4]) -> [u8; 4] {
    let mut res: [u8; 4] = [0; 4];

    for i in 0..4 {
        res[i] = w1[i] ^ w2[i];
    }

    return res;
}

/*
Shifts each byte in word to the right 1 space
*/
fn cycle(word: &[u8; 4]) -> [u8; 4] {
    let mut res: [u8; 4] = [0; 4];

    res[0] = word[1];
    res[1] = word[2];
    res[2] = word[3];
    res[3] = word[0];

    return res;
}

/*
Substitutes byte with appropriate byte from S-Box
*/
fn sub_word(word: &[u8; 4]) -> [u8; 4] {
    let mut res: [u8; 4] = [0; 4];

    for i in 0..4 {
        res[i] = aff(inv(word[i]));
    }

    return res;
}

/*
Gets the round constant for round i
*/
fn rc(i: usize) -> u8 {
    if i == 1 {
        return 1;
    } else {
        return math_helper::byte_multiply(&2, &rc(i-1));
    }
}

/*
Generates key schedule for AES
*/
fn key_schedule(key: [[u8; 4]; 4]) -> [[[u8; 4]; 4]; 11] {
    let mut schedule: [[[u8; 4]; 4]; 11] = [[[0; 4]; 4]; 11];
    schedule[0] = key;
    return key_schedule_helper(&mut schedule, 1);
}

/*
Helper function for key schedule that does most of the work. Expands the original key into 11 4x4 arrays of bytes.
*/
fn key_schedule_helper(schedule: &mut [[[u8; 4]; 4]; 11], cur: usize) -> [[[u8; 4]; 4]; 11] {
    if cur >= 11 {
        return *schedule;
    }

    // Create key for current iteration
    let mut cur_key: [[u8; 4]; 4] = [[0; 4]; 4];

    let prev_key = &schedule[cur-1];

    // Get words from previous key
    let w0: [u8; 4] = get_column_as_array(prev_key, 0); 
    let w1: [u8; 4] = get_column_as_array(prev_key, 1); 
    let w2: [u8; 4] = get_column_as_array(prev_key, 2); 
    let w3: [u8; 4] = get_column_as_array(prev_key, 3); 

    // Create words to go into current key
    let mut rc_word: [u8; 4] = [0; 4];
    rc_word[0] = rc(cur);

    let w4: [u8; 4] = word_add(&w0, &word_add(&rc_word, &sub_word(&cycle(&w3)))); 
    let w5: [u8; 4] = word_add(&w4, &w1);
    let w6: [u8; 4] = word_add(&w5, &w2);
    let w7: [u8; 4] = word_add(&w6, &w3);

    // Adds words to current key
    for i in 0..4 {
        cur_key[i][0] = w4[i];
    }
    for i in 0..4 {
        cur_key[i][1] = w5[i];
    }
    for i in 0..4 {
        cur_key[i][2] = w6[i];
    }
    for i in 0..4 {
        cur_key[i][3] = w7[i];
    }

    schedule[cur] = cur_key;

    return key_schedule_helper(schedule, cur+1);
}

/*
Encrypts a message block using the AES block cipher

Parameters:
    message - 4x4 array of bytes
    key - 4x4 array of bytes 

Returns:
    4x4 array of bytes; the encrypted message
*/
pub fn AES_encrypt(message: [[u8; 4]; 4], key: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let schedule = key_schedule(key);

    let mut state = message;

    // Add initial key
    add_round_key(&mut state,schedule[0]);

    for r in 1..=10 {
        // Sub Bytes
        sub_bytes(&mut state);

        // Shift Rows
        shift_rows(&mut state);

        // Mix Columns
        if r != 10 {
            mix_columns(&mut state);
        }

        // Add Round Key
        add_round_key(&mut state,schedule[r]);
    }

    return state;
}

pub fn AES_decrypt(ciphertext: [[u8; 4]; 4], key: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let schedule = key_schedule(key);

    let mut state = ciphertext;

    // Add initial key
    add_round_key(&mut state,schedule[10]);

    for r in (0..=9).rev() {
        // Inv Shift Rows
        inv_sub_bytes(&mut state);

        // Inv Sub Bytes
        inv_shift_rows(&mut state);

        // Add Round Key
        add_round_key(&mut state,schedule[r]);

        // Inv Mix Columns
        if r != 0 {
            inv_mix_columns(&mut state);
        }
    }

    return state;
}

pub fn AES_main() {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inv() {
        assert_eq!(inv(0),0);
        assert_eq!(inv(1),1);
        assert_eq!(inv(255),28);
        assert_eq!(inv(247),140);
        assert_eq!(inv(79),9);
    }

    #[test]
    fn test_sub_bytes() {
        let mut state: [[u8; 4]; 4] = [
            [0x00, 0x18, 0x2b, 0x30],
            [0x4f, 0x5a, 0x62, 0x74],
            [0x88, 0x9c, 0xae, 0xb5],
            [0xc9, 0xdd, 0xe2, 0xf3]
        ];

        sub_bytes(&mut state);

        let res: [[u8; 4]; 4] = [
            [0x63, 0xad, 0xf1, 0x04],
            [0x84, 0xbe, 0xaa, 0x92],
            [0xc4, 0xde, 0xe4, 0xd5],
            [0xdd, 0xc1, 0x98, 0x0d]
        ];
        
        assert_eq!(state,res);
    }

    #[test]
    fn test_inv_sub_bytes() {
        let mut state: [[u8; 4]; 4] = [
            [0x63, 0xad, 0xf1, 0x04],
            [0x84, 0xbe, 0xaa, 0x92],
            [0xc4, 0xde, 0xe4, 0xd5],
            [0xdd, 0xc1, 0x98, 0x0d]
        ];

        inv_sub_bytes(&mut state);

        let res: [[u8; 4]; 4] = [
            [0x00, 0x18, 0x2b, 0x30],
            [0x4f, 0x5a, 0x62, 0x74],
            [0x88, 0x9c, 0xae, 0xb5],
            [0xc9, 0xdd, 0xe2, 0xf3]
        ];
        
        assert_eq!(state,res);
    }

    #[test]
    fn test_AES_encrypt() {
        let message: [[u8; 4]; 4] = [
            [0x54, 0x4f, 0x4e, 0x20],
            [0x77, 0x6e, 0x69, 0x54],
            [0x6f, 0x65, 0x6e, 0x77],
            [0x20, 0x20, 0x65, 0x6f]
        ];

        let key: [[u8; 4]; 4] = [
            [0x54, 0x73, 0x20, 0x67],
            [0x68, 0x20, 0x4b, 0x20],
            [0x61, 0x6d, 0x75, 0x46],
            [0x74, 0x79, 0x6e, 0x75]
        ];

        let ciphertext: [[u8; 4]; 4] = AES_encrypt(message,key);

        let res: [[u8; 4]; 4] = [
            [0x29, 0x57, 0x40, 0x1a],
            [0xc3, 0x14, 0x22, 0x02],
            [0x50, 0x20, 0x99, 0xd7],
            [0x5f, 0xf6, 0xb3, 0x3a]
        ];

        assert_eq!(ciphertext,res);
    }
}
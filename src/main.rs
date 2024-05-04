mod crypto;

fn main() {
    let M: [[u8; 4]; 4] = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];

    let K: [[u8; 4]; 4] = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];

    let encrypted: [[u8; 4]; 4] = crypto::ciphers::AES_encrypt(M,K);

    println!("{:?}", encrypted);
}



mod utils;

use wasm_bindgen::prelude::*;
use solana_sdk::signer::*;
use std::fmt::Debug;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, wasm-vanity-address!");
// }

#[wasm_bindgen]
#[derive(Debug)]
pub struct SolKeypair {
    public_key: [u8; 32],
    secret_key: [u8; 64],
}

#[wasm_bindgen]
impl SolKeypair {
    pub fn generate(s: &str) -> SolKeypair {
        log!("Vanity String is: {}", s);

        let mut i = 0;

        loop {
            i += 1;

            let keypair = keypair::Keypair::new();
    
            let pubkey = keypair.pubkey();
            let pubkey_string = pubkey.clone().to_string();

            // println!("{} -> {}", i, pubkey_string);
            // log!("{} -> {:?}", i, pubkey_string);

            if pubkey_string.starts_with(s) {
                break 
                    SolKeypair {
                        public_key: pubkey.to_bytes(),
                        secret_key: keypair.to_bytes(),
                    };
            }
        }
    }

    pub fn public_key(&self) -> *const u8 {
        self.public_key.as_ptr()
    }

    pub fn secret_key(&self) -> *const u8 {
        self.secret_key.as_ptr()
    }
}

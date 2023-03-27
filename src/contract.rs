use ink_lang::contract;
use ink_prelude::string::String;
use ink_prelude::vec::Vec;

#[contract]
mod art_authentication {
    use super::*;

    #[ink(storage)]
    pub struct ArtToken {
        name: String,
        symbol: String,
        total_supply: u64,
        owner_of: Vec<Option<AccountId>>,
        artwork_info: Vec<String>,
        artwork_price: Vec<u128>,
        artist_royalty_fee: Vec<u8>,
    }

    impl ArtToken {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            let mut owner_of = Vec::new();
            owner_of.push(None);

            let mut artwork_info = Vec::new();
            artwork_info.push("".to_string());

            let mut artwork_price = Vec::new();
            artwork_price.push(0);

            let mut artist_royalty_fee = Vec::new();
            artist_royalty_fee.push(0);

            Self {
                name,
                symbol,
                total_supply: 0,
                owner_of,
                artwork_info,
                artwork_price,
                artist_royalty_fee,
            }
        }

        #[ink(message)]
        pub fn mint(&mut self, artwork_info: String, artwork_price: u128, artist_royalty_fee: u8) -> u64 {
            let caller = self.env().caller();

            let token_id = self.total_supply;
            self.total_supply += 1;

            self.owner_of.push(Some(caller));

            self.artwork_info.push(artwork_info);
            self.artwork_price.push(artwork_price);
            self.artist_royalty_fee.push(artist_royalty_fee);

            self.env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                token_id,
            });

            token_id
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, token_id: u64) {
            let caller = self.env().caller();

            let current_owner = self.owner_of[token_id as usize].unwrap();
            assert_eq!(caller, current_owner, "Only the current owner can transfer the token");

            self.owner_of[token_id as usize] = Some(to);

            self.env().emit_event(Transfer {
                from: Some(current_owner),
                to: Some(to),
                token_id,
            });
        }

        #[ink(message)]
        pub fn owner_of(&self, token_id: u64) -> Option<AccountId> {
            self.owner_of[token_id as usize]
        }
    }

    #[ink(event)]
    pub struct Transfer {
        from: Option<AccountId>,
        to: Option<AccountId>,
        token_id: u64,
    }
}
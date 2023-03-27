use super::*;
use ink_lang as ink;

#[ink::test]
fn mint_creates_new_token() {
    let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

    let mut contract = ArtToken::new(String::from("ArtToken"), String::from("ART"));
    assert_eq!(contract.total_supply, 0);

    let artwork_info = String::from("This is a test artwork");
    let artwork_price = 1000;
    let artist_royalty_fee = 10;

    let token_id = contract.mint(artwork_info.clone(), artwork_price, artist_royalty_fee);
    assert
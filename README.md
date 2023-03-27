# Art Authentication and Ownership Tracking
## Summary
The idea behind using an ERC-721 equivalent token on the PSP34 blockchain for art authentication and ownership tracking is to provide a transparent and immutable record of an artwork's ownership and provenance.  
Each artwork would be assigned a unique token, which would contain information about the artwork's authenticity, ownership history, and provenance. Art collectors, museums, and galleries could use the dApp to scan the artwork's token and verify its authenticity and ownership history. This would make it more difficult for fraudulent artworks to enter the market and give buyers confidence in their purchases.  
In addition, artists could use the dApp to create and sell their artwork as unique digital assets. By assigning each artwork a unique token, the artist could retain ownership of the original digital file while allowing collectors to purchase a verified and authenticated digital copy. This would give artists more control over the distribution and ownership of their artwork, while providing collectors with a secure and transparent way to buy and sell digital art.  
Overall, the purpose of using an ERC-721 equivalent token on the PSP34 blockchain for art authentication and ownership tracking is to provide a more secure and transparent way to track the ownership and provenance of artworks, while also giving artists more control over the distribution and ownership of their artwork.  

## Players
1. Artists: These are the creators of the artwork who mint the ERC-721 tokens on the PSP34 blockchain to represent their art pieces. They interact with the smart contract to create and manage the tokens, set the initial price, and define the royalty fee they want to receive on future sales.
2. Collectors: These are individuals or entities who purchase the ERC-721 tokens from the artists or from a marketplace that lists the tokens for sale. They interact with the smart contract to buy, transfer, and sell the tokens, as well as to check ownership and authenticity information.
3. Marketplaces (such as galleries or auction houses): These are platforms or websites that facilitate the buying and selling of ERC-721 tokens representing artwork. They interact with the smart contract to list tokens for sale, process transactions, and distribute royalties to the artists based on the fees specified in the smart contract.
4. Regulators: These are individuals or organizations responsible for verifying the authenticity of artwork and ensuring compliance with relevant laws and regulations. They interact with the smart contract to verify ownership and authenticity information recorded on the PSP34 blockchain.
The smart contract serves as a central authority for managing the ownership and authenticity information of the ERC-721 tokens representing artwork, and the users of the smart contract are the artists, collectors, marketplaces, and regulators who interact with it to participate in the decentralized and transparent art market.

## Scenario
1. Artists: The artist will mint a new ERC-721 token on the PSP34 blockchain for each piece of art they create. They will interact with the smart contract by calling the mint function, providing information such as the name, description, image, and other relevant details about the art piece. The artist will also set the initial price for the artwork and define the royalty fee they want to receive on future sales.
2. Collectors: The collector will interact with the smart contract by purchasing an ERC-721 token from the artist or from a marketplace that lists the token for sale. They will pay the price set by the artist and the transaction will be recorded on the blockchain. The collector will become the owner of the token and will have the right to transfer, sell or display the artwork as they see fit.
3. Marketplace: A marketplace can be created to allow collectors to buy and sell ERC-721 tokens representing artwork. The marketplace will interact with the smart contract by listing ERC-721 tokens for sale, allowing buyers to make purchases, and handling the transfer of ownership between parties. The marketplace will charge a commission on each sale and distribute royalties to the artist based on the fee specified in the smart contract.
4. Regulators: A regulator may also interact with the smart contract by verifying the authenticity of an artwork by checking its corresponding ERC-721 token on the PSP34 blockchain. They can ensure that the art piece has not been duplicated or replicated, and the ownership history of the art piece can also be traced on the blockchain.

## Actions
1. Artist:
- Mint a new ERC-721 token on the PSP34 blockchain for each piece of art they create by calling the mint function of the smart contract.
- Provide information such as the name, description, image, and other relevant details about the art piece while creating the token.
- Set the initial price for the artwork and define the royalty fee they want to receive on future sales in the smart contract.
- Transfer ownership of the token to a collector when the artwork is sold.
2. Collector:
- Purchase an ERC-721 token from the artist or from a marketplace that lists the token for sale by sending the required amount of cryptocurrency to the designated address of the smart contract.
- Check the ownership and authenticity information of the token on the PSP34 blockchain by interacting with the smart contract.
- Transfer ownership of the token to another collector by sending the token to their walletaddress through the smart contract.
- Sell the token on a marketplace and receive the proceeds from the sale minus the
commission and royalties. 
3. Marketplace:
- List ERC-721 tokens for sale on the marketplace by interacting with the smart contract.
- Process transactions by accepting payments from buyers and transferring the tokens to
their wallet address through the smart contract.
- Charge a commission on each sale and distribute royalties to the artist based on the fee
specified in the smart contract.
- Provide a platform for collectors to buy and sell tokens and interact with the smart
contract on their behalf. 
4. Regulator:
- Verify the authenticity of an artwork by checking its corresponding ERC-721 token on the PSP34 blockchain.
- Ensure that the art piece has not been duplicated or replicated by checking the ownership history of the token on the blockchain.
- Use the ownership and authenticity information recorded on the PSP34 blockchain to enforce relevant laws and regulations.

## Implements
### Contract
The code defines a Rust smart contract using the Ink! language, with a module called my_psp34. The Contract struct is defined with a single field:  
• psp34: a psp34::Data struct, which is used to implement the PSP34 standard for fungible tokens.  


The impl block defines the functionality of the contract, including:  
• a constructor that initializes the Contract struct with a psp34::Data instance  
• implementation of the PSP34 standard functions: total_supply, balance_of, transfer, and transfer_from.  
The total_supply function returns the total number of tokens that have been minted. The balance_of function returns the balance of a given account.  
The transfer function transfers tokens from the caller's account to another account.  
The transfer_from function transfers tokens from one account to another, but only if the caller has been authorized to do so.  


The contract also includes a few Rust attributes:  
• #[ink(storage)] marks the struct as an Ink! storage smart contract.  
• #[storage_field] marks a field as a persistent storage field, which means its value is persisted across contract calls.  
• #[ink(constructor)] marks a function as a constructor for the contract.  
• #[ink(message)] marks a function as a callable message on the contract.  

### Test
There are two functions being tested in this code:  
1. mint(): This function creates a new token by minting it with a unique token_id. The function
takes three arguments: artwork_info, artwork_price, and artist_royalty_fee. The test ensures that a new token is created when the function is called, and that the total_supply of tokens in the contract is incremented by 1.
2. transfer(): This function transfers ownership of a token from the sender to a new owner specified by to. The function takes two arguments: to, the new owner, and token_id, the id of the token being transferred. The test ensures that ownership of the token is transferred correctly and that the new owner is able to access the token using owner_of().

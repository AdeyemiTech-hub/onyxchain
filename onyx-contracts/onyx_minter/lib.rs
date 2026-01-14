#![cfg_attr(not(feature = \"std\"), no_std, no_main)]
#[ink::contract]
mod onyx_minter {
    #[ink(storage)]
    pub struct OnyxMinter { admin: AccountId }
    impl OnyxMinter {
        #[ink(constructor)] pub fn new(admin: AccountId) -> Self { Self { admin } }
        #[ink(message, payable)] pub fn buy_token(&mut self) {
            self.env().transfer(self.admin, self.env().transferred_value()).expect(\"Transfer Failed\");
        }
    }
}

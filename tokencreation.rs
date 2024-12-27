#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod token_factory {
    use ink_storage::collections::HashMap as StorageHashMap;
    
    #[ink(event)]
    pub struct TokenCreated {
        #[ink(topic)]
        pub token_address: AccountId,
        pub name: String,
        pub symbol: String,
        pub max_supply: Balance,
        pub burnable: bool,
    }

    #[ink(storage)]
    pub struct TokenFactory {
        owner: AccountId,
        created_tokens: StorageHashMap<AccountId, AccountId>,
    }

    impl TokenFactory {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
                created_tokens: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn create_token(
            &mut self,
            name: String,
            symbol: String,
            decimals: u8,
            max_supply: Balance,
            burnable: bool,
        ) -> AccountId {
            let new_token = CustomToken::new(
                name.clone(),
                symbol.clone(),
                decimals,
                max_supply,
                burnable,
                self.env().caller(),
            );

            let token_address = new_token.account_id();
            self.created_tokens.insert(self.env().caller(), token_address);

            self.env().emit_event(TokenCreated {
                token_address,
                name,
                symbol,
                max_supply,
                burnable,
            });

            token_address
        }

        #[ink(message)]
        pub fn withdraw_erc20(&mut self, token_address: AccountId, amount: Balance) {
            self.only_owner();
            let token = ink_env::call::build_call::<Environment>()
                .call_type(ink_env::call::Call::FromAccountId)
                .call_address(token_address)
                .push_arg(amount)
                .call();

            token.transact();
        }

        #[ink(message)]
        pub fn withdraw_ether(&mut self, amount: Balance) {
            self.only_owner();
            ink_env::transfer(self.owner, amount).expect("Transfer failed");
        }

        fn only_owner(&self) {
            assert_eq!(self.env().caller(), self.owner, "Only the owner can execute this");
        }
    }

    #[ink::contract]
    pub mod custom_token {
        use ink_storage::collections::HashMap as StorageHashMap;

        #[ink(storage)]
        pub struct CustomToken {
            name: String,
            symbol: String,
            decimals: u8,
            max_supply: Balance,
            total_supply: Balance,
            burnable: bool,
            owner: AccountId,
            balances: StorageHashMap<AccountId, Balance>,
            allowances: StorageHashMap<(AccountId, AccountId), Balance>,
        }

        impl CustomToken {
            #[ink(constructor)]
            pub fn new(
                name: String,
                symbol: String,
                decimals: u8,
                max_supply: Balance,
                burnable: bool,
                owner: AccountId,
            ) -> Self {
                let mut balances = StorageHashMap::new();
                balances.insert(owner, max_supply);

                Self {
                    name,
                    symbol,
                    decimals,
                    max_supply,
                    total_supply: max_supply,
                    burnable,
                    owner,
                    balances,
                    allowances: StorageHashMap::new(),
                }
            }

            #[ink(message)]
            pub fn total_supply(&self) -> Balance {
                self.total_supply
            }

            #[ink(message)]
            pub fn balance_of(&self, account: AccountId) -> Balance {
                *self.balances.get(&account).unwrap_or(&0)
            }

            #[ink(message)]
            pub fn transfer(&mut self, recipient: AccountId, amount: Balance) -> bool {
                let caller = self.env().caller();
                let sender_balance = self.balance_of(caller);
                assert!(sender_balance >= amount, "Insufficient balance");

                self.balances.insert(caller, sender_balance - amount);
                let recipient_balance = self.balance_of(recipient);
                self.balances.insert(recipient, recipient_balance + amount);

                true
            }

            #[ink(message)]
            pub fn approve(&mut self, spender: AccountId, amount: Balance) -> bool {
                let caller = self.env().caller();
                self.allowances.insert((caller, spender), amount);
                true
            }

            #[ink(message)]
            pub fn transfer_from(
                &mut self,
                sender: AccountId,
                recipient: AccountId,
                amount: Balance,
            ) -> bool {
                let caller = self.env().caller();
                let sender_balance = self.balance_of(sender);
                let allowance = *self.allowances.get(&(sender, caller)).unwrap_or(&0);

                assert!(sender_balance >= amount, "Insufficient balance");
                assert!(allowance >= amount, "Allowance exceeded");

                self.balances.insert(sender, sender_balance - amount);
                let recipient_balance = self.balance_of(recipient);
                self.balances.insert(recipient, recipient_balance + amount);
                self.allowances.insert((sender, caller), allowance - amount);

                true
            }

            #[ink(message)]
            pub fn mint(&mut self, to: AccountId, amount: Balance) {
                assert_eq!(self.env().caller(), self.owner, "Only the owner can mint");

                let new_supply = self.total_supply + amount;
                assert!(new_supply <= self.max_supply, "Max supply exceeded");

                self.total_supply = new_supply;
                let to_balance = self.balance_of(to);
                self.balances.insert(to, to_balance + amount);
            }

            #[ink(message)]
            pub fn burn(&mut self, amount: Balance) {
                assert_eq!(self.env().caller(), self.owner, "Only the owner can burn");
                assert!(self.burnable, "Burning not allowed");

                let caller_balance = self.balance_of(self.env().caller());
                assert!(caller_balance >= amount, "Insufficient balance to burn");

                self.balances.insert(self.env().caller(), caller_balance - amount);
                self.total_supply -= amount;
            }

            #[ink(message)]
            pub fn burn_from(&mut self, account: AccountId, amount: Balance) {
                assert_eq!(self.env().caller(), self.owner, "Only the owner can burn");
                assert!(self.burnable, "Burning not allowed");

                let account_balance = self.balance_of(account);
                assert!(account_balance >= amount, "Insufficient balance to burn");

                self.balances.insert(account, account_balance - amount);
                self.total_supply -= amount;
            }
        }
    }
}

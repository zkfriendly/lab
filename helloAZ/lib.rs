#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod MyToken {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct MyToken {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
    }

    impl MyToken {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            balances.insert(Self::env().caller(), &total_supply);
            Self {
                total_supply,
                balances,
            }
        }

        #[ink(message)]
        pub fn balance_of(&self, id: AccountId) -> Balance {
            return self.balances.get(id).unwrap_or_default();
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            return self.total_supply;
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: Balance) -> Result<(), Error> {
            // retrieve caller
            let from = self.env().caller();
            let from_balance = self.balance_of(from);

            // check for sufficient balance
            if amount > from_balance {
                return Err(Error::InsufficientBalance);
            }

            let to_balance = self.balance_of(to);

            // update balances
            self.balances.insert(from, &(from_balance - amount));
            self.balances.insert(to, &(to_balance + amount));

            Ok(())
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[ink::test]
        fn test_total_supply() {
            let token = MyToken::new(100);
            assert_eq!(token.total_supply(), 100);
        }

        #[ink::test]
        fn test_balance_of() {
            let token = MyToken::new(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let alice_balance = token.balance_of(accounts.alice);
            assert_eq!(alice_balance, 100);
        }

        #[ink::test]
        fn test_transfer() {
            let mut token = MyToken::new(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let alice_before_balance = token.balance_of(accounts.alice);
            let bob_before_balance = token.balance_of(accounts.bob);

            // assert before balances
            assert_eq!(alice_before_balance, 100);
            assert_eq!(bob_before_balance, 0);

            // make the transer
            assert_eq!(token.transfer(accounts.bob, 50), Ok(()));

            // assert after balances
            let alice_after_balance = token.balance_of(accounts.alice);
            let bob_after_balance = token.balance_of(accounts.bob);

            assert_eq!(alice_after_balance, 50);
            assert_eq!(bob_after_balance, 50);
        }
    }
}

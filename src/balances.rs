use num_traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

/// Define the common types used in this pallet
pub trait Config {
    type AccountId: Ord + Clone;
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

/// This is the balances pallet, it handles account balances
#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<<T as Config>::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the balances pallet
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// Set the balance of an account
    pub fn set_balance(&mut self, who: &<T as Config>::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    /// Get the balance of an account
    pub fn balance(&self, who: &<T as Config>::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    /// Transfer balance from one account to another
    pub fn transfer(
        &mut self,
        caller: <T as Config>::AccountId,
        to: <T as Config>::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient balance")?;
        let to_balance = self.balance(&to);
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Balance overflow")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Pallet};
    use num_traits::{CheckedAdd, CheckedSub, Zero};

    struct TestConfig;

    impl Config for TestConfig {
        type AccountId = String;
        type Balance = u64;
    }

    #[test]
    fn set_balance() {
        let mut pallet: Pallet<TestConfig> = Pallet::new();
        let alice = String::from("alice");
        pallet.set_balance(&alice, 100);
        assert_eq!(pallet.balance(&alice), 100);
    }

    #[test]
    fn transfer_balance() {
        let mut pallet: Pallet<TestConfig> = Pallet::new();
        let alice = String::from("alice");
        let bob = String::from("bob");
        pallet.set_balance(&alice, 100);
        assert!(pallet.transfer(alice.clone(), bob.clone(), 50).is_ok());
        assert_eq!(pallet.balance(&alice), 50);
        assert_eq!(pallet.balance(&bob), 50);
    }

    #[test]
    fn transfer_insufficient_balance() {
        let mut pallet: Pallet<TestConfig> = Pallet::new();
        let alice = String::from("alice");
        let bob = String::from("bob");
        assert_eq!(
            pallet.transfer(alice, bob, 100),
            Err("Insufficient balance")
        );
    }
}

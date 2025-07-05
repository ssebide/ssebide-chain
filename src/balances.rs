use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

pub trait Config {
    type AccountId = Ord + Clone;
    type Balance = Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    ///set the balance of an account 'who' to some 'amount'
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        //Insert the amount into the BTreemap under 'who'
        self.balances.insert(who.clone(), amount);
    }

    ///Get the balance of an account 'who'
    /// if the account has no stored balance, we return zero.
    pub fn balance(&mut self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

    ///transfer amount from one account to another
    /// this function verifies that 'from' account has at least 'amount' balance to transfer and nomathematical overflows occu
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insuffiecient balance")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        let mut balances = super::Pallet::new();

        balances.set_balance(&"alice".to_string(), 100);
        let _ = balances.transfer(alice.clone(), bob.clone(), 90);

        assert_eq!(balances.balance(&alice), 10);
        assert_eq!(balances.balance(&bob), 90);
    }
}

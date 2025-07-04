use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    ///set the balance of an account 'who' to some 'amount'
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        //Insert the amount into the BTreemap under 'who'
        unimplemented!()
    }

    ///Get the balance of an account 'who'
    /// if the account has no stored balance, we return zero.
    pub fn balance(&mut self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    ///transfer amount from one account to another
    /// this function verifies that 'from' account has at least 'amount' balance to transfer and nomathematical overflows occu
    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("Insuffiecient balance")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

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

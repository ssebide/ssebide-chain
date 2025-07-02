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
}

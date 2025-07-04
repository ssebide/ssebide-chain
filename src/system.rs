use std::collections::BTreeMap;

///Define the common types used in this pallet
type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;

///this is the system pallet, it handles the low level state needed for the blockchain
#[derive(Debug)]
pub struct Pallet {
    ///the current block number
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
    // a map from an account to their nonce
}

impl Pallet {
    ///Create  a new instance of the system Pallet
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    ///get the current block number
    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    //this function can be used to increment the block number
    pub fn increment_block_number(&mut self) {
        self.block_number = self.block_number.checked_add(1).unwrap();
    }

    //increment the nounce of an account
    pub fn inc_nounce(&mut self, who: &AccountId) {
        let nonce = self.nonce.get(who).unwrap_or(&0);
        self.nonce.insert(who.clone(), nonce + 1);
    }

    pub fn get_nonce(&self, who: &AccountId) -> Nonce {
        *self.nonce.get(who).unwrap_or(&0)
    }
}

#[cfg(test)]

mod test {
    #[test]
    fn init_system() {
        let system = super::Pallet::new();
        assert_eq!(system.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut system = super::Pallet::new();
        system.increment_block_number();
        assert_eq!(system.block_number(), 1);
    }

    #[test]
    fn inc_nonce() {
        let alice = String::from("alice");
        let mut system = super::Pallet::new();
        system.inc_nounce(&alice.clone());
        assert_eq!(system.get_nonce(&alice), 1);
    }
}

use std::{collections::BTreeMap, ops::AddAssign};

use num::{One, Zero};

///Define the common types used in this pallet
pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + Copy;
}

///this is the system pallet, it handles the low level state needed for the blockchain
#[derive(Debug)]
pub struct Pallet<T: Config> {
    ///the current block number
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
    // a map from an account to their nonce
}

impl<T: Config> Pallet<T> {
    ///Create  a new instance of the system Pallet
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::Zero,
            nonce: BTreeMap::new(),
        }
    }

    ///get the current block number
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    //this function can be used to increment the block number
    pub fn increment_block_number(&mut self) {
        self.block_number = T::BlockNumber::One;
    }

    //increment the nounce of an account
    pub fn inc_nounce(&mut self, who: &T::AccountId) {
        let nonce = self.nonce.get(who).unwrap_or(&Zero);
        self.nonce.insert(who.clone(), nonce + One);
    }

    pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&Zero)
    }
}

#[cfg(test)]

mod test {
    use num::Zero;

    struct TestConfig;

    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let system: super::Pallet<TestConfig> = super::Pallet::new();
        assert_eq!(system.block_number(), Zero);
    }

    #[test]
    fn inc_block_number() {
        let mut system: super::Pallet<TestConfig> = super::Pallet::new();
        system.increment_block_number();
        assert_eq!(system.block_number(), One);
    }

    #[test]
    fn inc_nonce() {
        let alice = String::from("alice");
        let mut system: super::Pallet<TestConfig> = super::Pallet::new();
        system.inc_nounce(&alice.clone());
        assert_eq!(system.get_nonce(&alice), One);
    }
}

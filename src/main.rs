mod balances;
mod system;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
}

#[derive(Debug)]
pub struct Runtime {
    //create a field system and balance
    system: system::Pallet,
    balances: balances::Pallet<types::AccountId, types::Balance>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    //emulate the blockchain
    //increment a block number in the system
    runtime.system.increment_block_number();

    assert_eq!(runtime.system.block_number(), 1);

    //create first transaction, execute nonce of alice, execute transfer from alice to bob
    runtime.system.inc_nounce(&alice);

    let _ = runtime
        .balances
        .transfer(alice.clone(), bob.clone(), 30)
        .map_err(|e| println!("Error: {:?}", e));

    runtime.system.inc_nounce(&alice);

    let _ = runtime
        .balances
        .transfer(alice.clone(), charlie.clone(), 20)
        .map_err(|e| println!("Error: {:?}", e));

    println!("{:#?}", runtime)
}

#[test]
fn init_balances() {
    let mut balances = balances::Pallet::new();

    assert_eq!(balances.balance(&"alice".to_string()), 0);
    balances.set_balance(&"alice".to_string(), 100);
    assert_eq!(balances.balance(&"alice".to_string()), 100);
    assert_eq!(balances.balance(&"bob".to_string()), 0);
}

use std::collections::BTreeMap; // import the TreeMap struct from the std::collections module
pub struct Pallet {
    pub balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &String, amunt: u128) {}

    pub fn balance(&self, who: &String) -> u128 {}
}

use std::collections::BTreeMap;

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
pub struct Pallet {
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    /// Create a new instance of the balances module.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    /// Get the balance of an account `who`.
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balances.get(&caller);
        let to_balance = self.balances.get(&to);

        // OPTION: Check if caller_balance have a balance, if not return an error
        let caller_balance = match caller_balance {
            Some(balance) => balance,
            None => return Err("Caller has no balance"),
        };

        // Check if the caller has enough balance to transfer
        if *caller_balance < amount {
            return Err("Caller has insufficient balance");
        }

        // Check if to_balance have a balance, if not return an error
        let to_balance = match to_balance {
            Some(balance) => balance,
            None => return Err("Recipient has no balance"),
        };

        // Check for overflow when calculating the new balance for the caller
        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or_else(|| "Overflow in caller balance calculation")?;

        // Check for overflow when calculating the new balance for the recipient
        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or_else(|| "Overflow in recipient balance calculation")?;

        // Insert the new balances
        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

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
        /* TODO: Create a test that checks the following:
            - That `alice` cannot transfer funds she does not have.
            - That `alice` can successfully transfer funds to `bob`.
            - That the balance of `alice` and `bob` is correctly updated.
        */
        let mut balances = super::Pallet::new();
        balances.set_balance(&"alice".to_string(), 100);
        balances.set_balance(&"bob".to_string(), 50);
        assert!(
            balances
                .transfer("alice".to_string(), "bob".to_string(), 30)
                .is_ok()
        );
        assert_eq!(balances.balance(&"alice".to_string()), 70);
        assert_eq!(balances.balance(&"bob".to_string()), 80);
        assert!(
            balances
                .transfer("alice".to_string(), "bob".to_string(), 100)
                .is_err()
        );
        
    }
}

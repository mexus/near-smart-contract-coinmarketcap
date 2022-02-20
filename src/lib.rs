//! Storing historical price data.

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

mod fifo;

const HISTORY_DEPTH: usize = 5;

/// A contract that's able to store a historical data and making an average out
/// of it.
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct PriceHistory {
    price_history: fifo::Fifo<f64, HISTORY_DEPTH>,
    recorded: u16,
}

#[near_bindgen]
impl PriceHistory {
    /// Returns the average price.
    ///
    /// # Panics
    ///
    /// Will panic when not enough historical data has been collected.
    pub fn get_average(&self) -> f64 {
        if usize::from(self.recorded) != HISTORY_DEPTH {
            env::panic(b"Not enough historical data has been collected yet")
        }
        let sum: f64 = self.price_history.iter().sum();
        sum / HISTORY_DEPTH as f64
    }

    /// Adds the provided `price` to the storage.
    ///
    /// # Panics
    ///
    /// Will panic when called not from the account which was used to deployed
    /// the contract.
    pub fn record_price(&mut self, price: f64) {
        if env::signer_account_id() != env::current_account_id() {
            // Prevent others from adding possibly malicious records.
            env::panic(b"Sorry, you are not allowed to record a price")
        }
        if usize::from(self.recorded) < HISTORY_DEPTH {
            self.recorded += 1;
        }
        self.price_history.push(price)
    }

    /// Returns the depth of the recorded history.
    pub fn depth_so_far(&self) -> usize {
        usize::from(self.recorded)
    }

    /// Returns the amount of required historical data to calculate the average.
    pub fn required_depth(&self) -> usize {
        HISTORY_DEPTH
    }

    /// Forgets the history.
    pub fn reset(&mut self) {
        self.recorded = 0;
        env::log(b"History has been reset");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // Hint: copied from one of NEAR SDK examples.
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn record() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut counter = PriceHistory::default();

        for price in [1., 2., 3., 4., 5.] {
            counter.record_price(price);
        }

        let expected = 3.;
        assert!((counter.get_average() - expected).abs() < 1e-5);
    }

    #[test]
    #[should_panic]
    fn empty() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let counter = PriceHistory::default();
        counter.get_average();
    }
}

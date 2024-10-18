#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use std::str::FromStr;
use std::vec;

use alloy_sol_types::SolError;
use hex::NIL;
use stylus_sdk::{
    alloy_primitives::*,
    alloy_sol_types::{sol, SolEvent},
    contract::address,
    evm, msg,
    prelude::*,
    prelude::{sol_interface, sol_storage},
    storage::*,
};
use stylus_sdk::{alloy_sol_types, block};

sol_interface! {

    interface IDecrypter {
        function decrypt(uint8[] memory encrypted_data, uint8[] memory decryption_key) external view returns (uint8[] memory);
    }
}

sol_storage! {

    #[entrypoint]
    pub struct Auction {
        StorageVec<BidEntry> bid_entries;
        uint auction_deadline;
        uint auction_id;
        StorageAddress decrypter_contract;

        uint256 auction_fee;
        address auction_owner;
        bool auction_finalized;
        bool auction_initialized;
        StorageString winning_bid;
        StorageString decryption_key;
    }
}

impl Auction {}

#[external]
impl Auction {
    pub fn set_vars(
        &mut self,
        decrypter: Address,
        deadline: u128,
        id: u128,
        fee: u128,
    ) -> Result<(), stylus_sdk::call::Error> {
        if self.auction_initialized.get() {
            return Err(stylus_sdk::call::Error::Revert(vec![]));
        }
        self.decrypter_contract.set(decrypter);
        self.auction_deadline.set(U256::from(deadline));
        self.auction_id.set(U256::from(id));
        self.auction_fee.set(U256::from(fee));
        self.bid_entries = unsafe { StorageVec::new(U256::ZERO, 0) };
        self.auction_owner.set(msg::sender());
        self.auction_initialized.set(true);
        self.auction_finalized.set(false);
        self.winning_bid.set_str("0");
        self.decryption_key.set_str("0");

        Ok(())
    }

    pub fn check_condition(&mut self) -> Result<Vec<String>, Vec<u8>> {
        let condition_string = self.auction_id.to_string() + "-" + &self.auction_deadline.to_string();
        Ok(vec![condition_string])
    }

    #[payable]
    pub fn submit_enc_bid(
        &mut self,
        tx: Vec<u8>,
        condition: String,
    ) -> Result<Vec<u8>, stylus_sdk::call::Error> {
        if msg::value() < *self.auction_fee {}

        let mut bid_entry: StorageGuardMut<'_, BidEntry> = self.bid_entries.grow();
        bid_entry.encrypted_tx.set_bytes(tx);
        bid_entry.bid_condition.set_str(condition.clone());
        bid_entry.bidder_address.set(msg::sender());
        Ok(self.bid_entries.len().to_string().as_bytes().to_vec())
    }

    pub fn submit_key(
        &mut self,
        condition: String,
        key: Vec<u8>,
    ) -> Result<Vec<u8>, stylus_sdk::call::Error> {
        let mut highest_bid: u128 = 0;
        let mut winning_bidder: Address = Address::ZERO;

        for i in 0..self.bid_entries.len() {
            let encrypted_tx = self.bid_entries.get_mut(i).unwrap().encrypted_tx.get_bytes();
            let bidder = self.bid_entries.get_mut(i).unwrap().bidder_address.clone();

            let decrypted_bid = self.dec(encrypted_tx, key.clone()).unwrap();

            let bid_value_str = String::from_utf8(decrypted_bid.clone()).expect("Invalid UTF-8 sequence");
            let bid_value = string_to_u128(bid_value_str.as_str()).unwrap();
            if bid_value > highest_bid {
                highest_bid = bid_value;
                winning_bidder = bidder;
            }
        }

        self.auction_finalized.set(true);
        self.winning_bid.set_str(highest_bid.to_string());

        Ok(highest_bid.to_string().as_bytes().to_vec())
    }

    pub fn dec(&mut self, encrypted_data: Vec<u8>, decryption_key: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        let decrypter_instance: IDecrypter = IDecrypter::new(*self.decrypter_contract);

        let decrypted_data = decrypter_instance.decrypt(self, encrypted_data, decryption_key.clone()).unwrap();

        Ok(decrypted_data)
    }

    pub fn check_winner(&mut self) -> Result<String, Vec<u8>> {
        Ok(self.winning_bid.get_string())
    }

    pub fn check_finished(&mut self) -> Result<bool, Vec<u8>> {
        Ok(*self.auction_finalized)
    }

    pub fn check_deadline(&mut self) -> Result<String, Vec<u8>> {
        Ok(self.auction_deadline.to_string())
    }

    pub fn check_id(&mut self) -> Result<String, Vec<u8>> {
        Ok(self.auction_id.to_string())
    }
}

#[solidity_storage]
pub struct BidEntry {
    encrypted_tx: StorageBytes,
    bidder_address: StorageAddress,
    bid_condition: StorageString,
}

fn string_to_u128(s: &str) -> Result<u128, String> {
    match s.parse::<u128>() {
        Ok(num) => Ok(num),
        Err(e) => Err(e.to_string()),
    }
}

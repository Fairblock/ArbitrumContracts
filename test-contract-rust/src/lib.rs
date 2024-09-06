#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;


use std::str::FromStr;
use std::vec;

use alloy_sol_types::SolError;
use hex::NIL;
use stylus_sdk::{alloy_sol_types, block};
use stylus_sdk::{
    alloy_primitives::*,
    prelude::{sol_interface, sol_storage},
    alloy_sol_types::{sol,SolEvent},
    contract::address,
    evm, msg,
    prelude::*,
    storage::*,
};


sol_interface! {
  
    interface IDecrypter {
        function decrypt(uint8[] memory c, uint8[] memory skbytes) external view returns (uint8[] memory);
    }
}


sol_storage! {

    #[entrypoint]
    pub struct Auction {
        StorageVec<EncBid> bids;
        uint deadline;
        uint id;
        StorageAddress decrypter;
   
        uint256 fee;
        address owner;
        bool finished;
        bool initialized;
        StorageString winner_bid;
        StorageString dec_key;
    }
}

impl Auction {

}


#[external]
impl Auction {
    pub fn set_vars(
        &mut self,
        decrypter: Address,
        deadline: u128,
        id: u128,
        fee: u128,
    ) -> Result<(), stylus_sdk::call::Error> {
        if self.initialized.get() {
            return Err(stylus_sdk::call::Error::Revert(vec![]))
        }
        self.decrypter.set(decrypter);
        self.deadline.set(U256::from(deadline));
        self.id.set(U256::from(id));
        self.fee.set(U256::from(fee));
        self.bids = unsafe { StorageVec::new(U256::ZERO, 0) };
        self.owner.set(msg::sender());
        self.initialized.set(true);
        self.finished.set(false);
        self.winner_bid.set_str("0");
        self.dec_key.set_str("0");
     
        let owner = self.owner.clone();
       
      
        Ok(())
    }

    pub fn check_condition(&mut self) -> Result<Vec<String>, Vec<u8>> {
        // if block::timestamp().to_string() == self.deadline.to_string() {
        //     let c = self.id.to_string() + &self.deadline.to_string();
        //     return Ok(c);
        // }
        // For testing purposes
        let c = self.id.to_string() +"-"+ &self.deadline.to_string();
        return Ok(vec![c]);
    //   return Ok("".to_string());
    }
    #[payable]
    pub fn submit_enc_bid(
        &mut self,
        tx: Vec<u8>,
        condition: String,
    ) -> Result<Vec<u8>, stylus_sdk::call::Error> {
   
        if msg::value() < *self.fee {
            
        }
        let c = self.id.to_string() + "-" + &self.deadline.to_string();
        if condition == c {
            let mut inner_vec: StorageGuardMut<'_, EncBid> = self.bids.grow();
            inner_vec.tx_.set_bytes(tx);
            inner_vec.condition.set_str(condition.clone());
            inner_vec.sender.set(msg::sender());
            return Ok(self.bids.len().to_string().as_bytes().to_vec());
        }
        return Err(stylus_sdk::call::Error::Revert(vec![]));
    }

    pub fn submit_key(&mut self, k: String, condition: String) -> Result<Vec<u8>, stylus_sdk::call::Error> {
        let key =  hex::decode(k).unwrap();
     
    
        let mut winner_bid: u128 = 0;
        let mut winner: Address = Address::ZERO;
        for i in 0..self.bids.len() {
            let c = self.bids.get_mut(i).unwrap().condition.get_string();
            if c == condition{
            let enc = self.bids.get_mut(i).unwrap().tx_.get_bytes();
            
            let sender = self.bids.get_mut(i).unwrap().sender.clone();
           
                let plain_bid = self.dec(enc, key.clone()).unwrap();
            
                let bid_string =
                    String::from_utf8(plain_bid.clone()).expect("Invalid UTF-8 sequence");
                let val = string_to_u128(bid_string.as_str()).unwrap();
                if val > winner_bid {
                    winner_bid = val;
                    winner = sender;
                    
                }}

        }
        self.finished.set(true);
        self.winner_bid.set_str(winner_bid.to_string());
    
        return Ok(winner_bid.to_string().as_bytes().to_vec())
    }

    pub fn dec(
        &mut self,
        tx: Vec<u8>,
        key: Vec<u8>,
    ) -> Result<Vec<u8>, Vec<u8>> {
   
        let decrypter: IDecrypter = IDecrypter::new(*self.decrypter);
  
        let plain_tx = decrypter
            .decrypt(self, tx, key.clone()).unwrap();

        Ok(plain_tx)
        
     
    }

    pub fn check_winner(&mut self) -> Result<String, Vec<u8>> {
        return Ok(self.winner_bid.get_string());
    }

    pub fn check_finished(&mut self) -> Result<bool, Vec<u8>> {
        return Ok(*self.finished);
    }
    pub fn check_deadline(&mut self) -> Result<String, Vec<u8>> {
        return Ok(self.deadline.to_string());
    }
    pub fn check_id(&mut self) -> Result<String, Vec<u8>> {
        return Ok(self.id.to_string());
    }
}
#[solidity_storage]
pub struct EncBid {
    tx_: StorageBytes,
    sender: StorageAddress,
    condition: StorageString,
}
fn string_to_u128(s: &str) -> Result<u128, String> {
    match s.parse::<u128>() {
        Ok(num) => Ok(num),
        Err(e) => Err(e.to_string()),
    }
}


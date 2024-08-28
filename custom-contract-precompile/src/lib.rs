#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

sol_interface! {
    interface IRegistry {
        function addContract(address addr_con, address addr_own, string calldata id) external returns (uint256);
    }

}
use std::str::FromStr;

use alloy_sol_types::SolError;
use ethabi::Token;
use stylus_sdk::block;
use stylus_sdk::call::RawCall;
use stylus_sdk::{
    alloy_primitives::*,
    alloy_sol_types::{sol},
    contract::address,
    evm, msg,
    prelude::*,
    storage::*,
};
pub enum AuctionError {
    OwnerUnauthorizedAccount(OwnerUnauthorizedAccount),
    OwnerInvalidOwner(OwnerInvalidOwner),
    OwnerAlreadyInitialized(OwnerAlreadyInitialized),
    NotInitialized(NotInitialized),
    NotEnoughFee(NotEnoughFee),
    ConditionNotMatched(ConditionNotMatched),
    AuctionEnded(AuctionEnded),
}

sol_storage! {

    #[entrypoint]
    pub struct Auction {
        StorageVec<EncBid> bids;
        uint deadline;
        uint id;
        StorageAddress registry;
   
        uint256 fee;
        address owner;
        bool finished;
        bool initialized;
        StorageString winner_bid;
        StorageString dec_key;
    }
}
sol! {
    event FailedKeyRequest(string condition);
    event AuctionWinner(string sender, string winner_bid);
    event OwnershipTransferred(address indexed previous_owner, address indexed new_owner);

    error OwnerUnauthorizedAccount(address account);
    error OwnerInvalidOwner(address owner);
    error OwnerAlreadyInitialized();
    error NotInitialized();
    error NotEnoughFee();
    error ConditionNotMatched();
    error AuctionEnded();
}
impl Auction {
    pub fn only_owner(&mut self) -> Result<(), AuctionError> {
        if msg::sender() != self.owner.get() {
            return Err(AuctionError::OwnerUnauthorizedAccount(
                OwnerUnauthorizedAccount {
                    account: msg::sender(),
                },
            ));
        }

        Ok(())
    }
    pub fn if_initialized(&mut self) -> Result<(), AuctionError> {
        if !(self.initialized.get()) {
            return Err(AuctionError::NotInitialized(NotInitialized {}));
        }

        Ok(())
    }

    pub fn if_not_finished(&mut self) -> Result<(), AuctionError> {
        if self.finished.get() {
            return Err(AuctionError::AuctionEnded(AuctionEnded {}));
        }

        Ok(())
    }
}

impl From<AuctionError> for Vec<u8> {
    fn from(err: AuctionError) -> Vec<u8> {
        match err {
            AuctionError::OwnerUnauthorizedAccount(e) => e.encode(),
            AuctionError::OwnerInvalidOwner(e) => e.encode(),
            AuctionError::OwnerAlreadyInitialized(e) => e.encode(),
            AuctionError::NotInitialized(e) => e.encode(),
            AuctionError::NotEnoughFee(e) => e.encode(),
            AuctionError::ConditionNotMatched(e) => e.encode(),
            AuctionError::AuctionEnded(e) => e.encode(),
        }
    }
}

#[external]
impl Auction {
    pub fn set_vars(
        &mut self,
        registry: Address,
        deadline: u128,
        id: u128,
        fee: u128,
    ) -> Result<(), AuctionError> {
        if self.initialized.get() {
            return Err(AuctionError::OwnerAlreadyInitialized(
                OwnerAlreadyInitialized {},
            ));
        }
        self.registry.set(registry);
     
        self.deadline.set(U256::from(deadline));
        self.id.set(U256::from(id));
        self.fee.set(U256::from(fee));
        self.bids = unsafe { StorageVec::new(U256::ZERO, 0) };
        self.owner.set(msg::sender());
        self.initialized.set(true);
        self.finished.set(false);
        self.winner_bid.set_str("0");
        self.dec_key.set_str("0");
        let registry: IRegistry = IRegistry::new(*self.registry);
        let owner = self.owner.clone();
       
        let _ = registry.add_contract(self, address(), owner,id.to_string());
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
    ) -> Result<Vec<u8>, AuctionError> {
        self.if_initialized()?;
        self.if_not_finished()?;
        if msg::value() < *self.fee {
            return Err(AuctionError::NotEnoughFee(NotEnoughFee {}));
        }
        let c = self.id.to_string() + "-" + &self.deadline.to_string();
        if condition == c {
            let mut inner_vec: StorageGuardMut<'_, EncBid> = self.bids.grow();
            inner_vec.tx_.set_bytes(tx);
            inner_vec.condition.set_str(condition.clone());
            inner_vec.sender.set(msg::sender());
            return Ok(self.bids.len().to_string().as_bytes().to_vec());
        }
        return Err(AuctionError::ConditionNotMatched(ConditionNotMatched {}));
    }

    pub fn submit_key(&mut self, k: String, condition: String) -> Result<Vec<u8>, AuctionError> {
        let key =  hex::decode(k).unwrap();
        self.if_initialized()?;
        self.if_not_finished()?;
    
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
        evm::log(AuctionWinner {
            sender: winner.to_string(),
            winner_bid: winner_bid.to_string(),
        });
        return Ok(winner_bid.to_string().as_bytes().to_vec())
    }

    pub fn dec(
        &mut self,
        tx: Vec<u8>,
        key: Vec<u8>,
    ) -> Result<Vec<u8>, Vec<u8>> {
   
    let input = encode_function(key, tx.clone());
      
    let plain = RawCall::new_static()
        .call(
            Address::from_str("0x0000000000000000000000000000000000000094").unwrap(),
            &input,
        )
        .unwrap();
 
     let p= plain[64..64+plain[63] as usize].to_vec();
        Ok(p)
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

fn encode_function(privateKeyByte: Vec<u8>,cipherBytes: Vec<u8> )-> Vec<u8>{
    let function_signature: [u8; 4] = [0x98, 0xfe, 0x9d, 0xfb];
    // Prepare the inputs as Tokens
    let inputs: Vec<Token> = vec![Token::Bytes(privateKeyByte), Token::Bytes(cipherBytes)];

    // Encode the inputs with the function signature
    let mut encoded = function_signature.to_vec();
    let encoded_params = ethabi::encode(&inputs);

    // Combine the function signature with the encoded parameters
    encoded.extend_from_slice(&encoded_params);

    encoded
}
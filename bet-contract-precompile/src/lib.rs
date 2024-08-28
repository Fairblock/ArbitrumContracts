#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

sol_interface! {
    interface IRegistry {
        function addContract(address addr_con, address addr_own) external returns (uint256);
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
pub enum BetError {
    OwnerUnauthorizedAccount(OwnerUnauthorizedAccount),
    OwnerInvalidOwner(OwnerInvalidOwner),
    OwnerAlreadyInitialized(OwnerAlreadyInitialized),
    NotInitialized(NotInitialized),
    NotEnoughFee(NotEnoughFee),
    ConditionNotMatched(ConditionNotMatched),
    BetEnded(BetEnded),
}

sol_storage! {

    #[entrypoint]
    pub struct Bet {
        StorageVec<BetDetails> bets;
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
    event BetWinner(string sender, string winner_bid);
    event OwnershipTransferred(address indexed previous_owner, address indexed new_owner);

    error OwnerUnauthorizedAccount(address account);
    error OwnerInvalidOwner(address owner);
    error OwnerAlreadyInitialized();
    error NotInitialized();
    error NotEnoughFee();
    error ConditionNotMatched();
    error BetEnded();
}
impl Bet {
    pub fn only_owner(&mut self) -> Result<(), BetError> {
        if msg::sender() != self.owner.get() {
            return Err(BetError::OwnerUnauthorizedAccount(
                OwnerUnauthorizedAccount {
                    account: msg::sender(),
                },
            ));
        }

        Ok(())
    }
    pub fn if_initialized(&mut self) -> Result<(), BetError> {
        if !(self.initialized.get()) {
            return Err(BetError::NotInitialized(NotInitialized {}));
        }

        Ok(())
    }

    pub fn if_not_finished(&mut self) -> Result<(), BetError> {
        if self.finished.get() {
            return Err(BetError::BetEnded(BetEnded {}));
        }

        Ok(())
    }
}

impl From<BetError> for Vec<u8> {
    fn from(err: BetError) -> Vec<u8> {
        match err {
            BetError::OwnerUnauthorizedAccount(e) => e.encode(),
            BetError::OwnerInvalidOwner(e) => e.encode(),
            BetError::OwnerAlreadyInitialized(e) => e.encode(),
            BetError::NotInitialized(e) => e.encode(),
            BetError::NotEnoughFee(e) => e.encode(),
            BetError::ConditionNotMatched(e) => e.encode(),
            BetError::BetEnded(e) => e.encode(),
        }
    }
}

#[external]
impl Bet {
    pub fn set_vars(
        &mut self,
        registry: Address,
        id: u128,
        fee: u128,
    ) -> Result<(), BetError> {
        if self.initialized.get() {
            return Err(BetError::OwnerAlreadyInitialized(
                OwnerAlreadyInitialized {},
            ));
        }
        self.registry.set(registry);
     
     
        self.id.set(U256::from(id));
        self.fee.set(U256::from(fee));
        self.bets = unsafe { StorageVec::new(U256::ZERO, 0) };
        self.owner.set(msg::sender());
        self.initialized.set(true);
        self.finished.set(false);
    
        let registry: IRegistry = IRegistry::new(*self.registry);
        let owner = self.owner.clone();
       
        let _ = registry.add_contract(self, address(), owner);
        Ok(())
    }

    pub fn check_condition(&mut self) -> Result<Vec<String>, Vec<u8>> {
        // if block::timestamp().to_string() == self.deadline.to_string() {
        //     let c = self.id.to_string() + &self.deadline.to_string();
        //     return Ok(c);
        // }
        // For testing purposes
      
        return Ok(vec![]);
    //   return Ok("".to_string());
    }
    #[payable]
    pub fn submit_bet(
        &mut self,
        condition: String,
    ) -> Result<Vec<u8>, BetError> {
        self.if_initialized()?;
        self.if_not_finished()?;
        if msg::value() < *self.fee {
            return Err(BetError::NotEnoughFee(NotEnoughFee {}));
        }
        let c = self.id.to_string() +"-"+ &condition;
       
            let mut inner_vec: StorageGuardMut<'_, BetDetails> = self.bets.grow();
         
            inner_vec.condition.set_str(c.clone());
            inner_vec.sender.set(msg::sender());
            return Ok(self.bets.len().to_string().as_bytes().to_vec());
        
        
    }

    pub fn submit_key(&mut self, k: String, condition: String) -> Result<Vec<u8>, BetError> {
        for i in 0..self.bets.len() {
            let c = self.bets.get_mut(i).unwrap().condition.get_string();
            if c == condition{
                self.bets.get_mut(i).unwrap().key.set_str(k.clone());
    }}
return Ok(vec![])
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
}
#[solidity_storage]
pub struct BetDetails {
    key: StorageString,
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
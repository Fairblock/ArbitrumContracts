#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

sol_interface! {
    interface IFairBlock {
        function requestKey(string calldata condition) external view returns (bool memory);
    }
    interface IDecrypter {
        function decryptTx(uint8[] memory tx, uint8[] memory skbytes, address ibe_contract, address decrypter_contract, address mac_contract) external view returns (uint8[] memory);
    }
}

use alloy_sol_types::SolError;
/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{
    alloy_primitives::*,
    alloy_sol_types::{sol, SolEvent},
    evm, msg,
    prelude::*,
    storage::*,
};
pub enum OwnerError {
    OwnerUnauthorizedAccount(OwnerUnauthorizedAccount),
    OwnerInvalidOwner(OwnerInvalidOwner),
    OwnerAlreadyInitialized(OwnerAlreadyInitialized),
}

sol_storage! {

    #[entrypoint]
    pub struct Auction {
        //mapping(uint256 => EncBid) bids;
        StorageVec<EncBid> bids;
        uint total;
        StorageAddress fairblock ;
        StorageAddress decrypter;
        StorageAddress ibe_contract;
        StorageAddress decrypter_contract;
        StorageAddress mac_contract;
        uint256 number;
        address owner;
        bool initialized;
    }
}
sol! {
    event FailedKeyRequest(string condition);
    event AuctionWinner(string condition, string sender, string winner_bid);
    event OwnershipTransferred(address indexed previous_owner, address indexed new_owner);

    error OwnerUnauthorizedAccount(address account);
    error OwnerInvalidOwner(address owner);
    error OwnerAlreadyInitialized();
}
impl Auction {

    pub fn only_owner(
        &mut self,
    ) -> Result<(), OwnerError> {
        if msg::sender() != self.owner.get() {
            return Err(OwnerError::OwnerUnauthorizedAccount(OwnerUnauthorizedAccount {
                account: msg::sender()
            }))
        }

        Ok(())
    }


}

impl From<OwnerError> for Vec<u8> {
    fn from(err: OwnerError) -> Vec<u8> {
        match err {
            OwnerError::OwnerUnauthorizedAccount(e) => e.encode(),
            OwnerError::OwnerInvalidOwner(e) => e.encode(),
            OwnerError::OwnerAlreadyInitialized(e) => e.encode(),
        }
    }
}

#[external]
impl Auction {

    pub fn set_vars(&mut self, fairblock:Address,decrypter: Address,ibe_contract:Address, decrypter_contract:Address, mac_contract: Address, total: u128)-> Result<(), OwnerError> {
        if (self.initialized.get()) {
            return Err(OwnerError::OwnerAlreadyInitialized(OwnerAlreadyInitialized {}));
        }
        self.fairblock.set(fairblock);
        self.decrypter.set(decrypter);
        self.ibe_contract.set(ibe_contract);
        self.decrypter_contract.set(decrypter_contract);
        self.mac_contract.set(mac_contract);
        self.total.set(U256::from(total));
        self.bids = unsafe { StorageVec::new(U256::ZERO, 0) };
        self.owner.set(msg::sender());
        self.initialized.set(true);
        Ok(())
    }

    pub fn check_condition() -> Result<bool, Vec<u8>>{
    //todo : add check for condition
    Ok(true)
    }
  
    pub fn submit_enc_bid(&mut self, tx: Vec<u8>, condition: String) -> Result<Vec<u8>, Vec<u8>> {
     
        let mut inner_vec: StorageGuardMut<'_, EncBid> = self.bids.grow();
        inner_vec.tx_.set_bytes(tx);
        inner_vec.condition.set_str(condition.clone());
        inner_vec.sender.set(msg::sender());
        Ok(self.bids.len().to_string().as_bytes().to_vec())

    }
    pub fn submit_key(&mut self, condition: String, key: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
       
        let mac_c = *self.mac_contract;
        let dec_c = *self.decrypter_contract;
        let ibe_c = *self.ibe_contract;
        let mut winner_bid: u128 = 0;
        let mut winner: Address = Address::ZERO;
        for i in 0..self.bids.len() {
            
            let c = self.bids.get_mut(i).unwrap().condition.get_string();
            let enc = self.bids.get_mut(i).unwrap().tx_.get_bytes();
            let sender = self.bids.get_mut(i).unwrap().sender.clone();
            if c == condition {
              
                let plain_bid = self.dec(enc, key.clone(), ibe_c, dec_c, mac_c).unwrap();
                let bid_string =String::from_utf8(plain_bid.clone()).expect("Invalid UTF-8 sequence");
                let val = string_to_u128(bid_string.as_str()).unwrap();
                if val > winner_bid {
                    winner_bid = val;
                    winner = sender;
                }
            }
        }

        evm::log(AuctionWinner {
            condition: condition,
            sender: winner.to_string(),
            winner_bid: winner_bid.to_string(),
        });
        Ok(winner_bid.to_string().as_bytes().to_vec())
    }

    fn dec(
        &mut self,
        tx: Vec<u8>,
        key: Vec<u8>,
        ibe_c: Address,
        dec_c: Address,
        mac_c: Address,
    ) -> Result<Vec<u8>, Vec<u8>> {
        let decrypter: IDecrypter = IDecrypter::new(*self.decrypter);
        // return Ok(self.decrypter.as_slice().to_vec());
        let plain_tx = decrypter
            .decrypt_tx(self, tx, key.clone(), ibe_c, dec_c, mac_c)
            .unwrap();

        Ok(plain_tx)
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
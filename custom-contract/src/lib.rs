#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

sol_interface! {
    interface IRegistry {
        function addContract(address addr_con, address addr_own) external returns (uint256);
    }
    interface IDecrypter {
        function decrypt(uint8[] memory c, uint8[] memory skbytes, address ibe_contract, address decrypter_contract, address mac_contract) external view returns (uint8[] memory);
    }
}
use alloy_sol_types::SolError;
use stylus_sdk::block;
use stylus_sdk::{
    alloy_primitives::*,
    alloy_sol_types::{sol, SolEvent},
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
        StorageAddress decrypter;
        StorageAddress ibe_contract;
        StorageAddress decrypter_contract;
        StorageAddress mac_contract;
        uint256 fee;
        address owner;
        bool finished;
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
        if !self.finished.get() {
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
        decrypter: Address,
        ibe_contract: Address,
        decrypter_contract: Address,
        mac_contract: Address,
        deadline: u128,
        id: u128,
        fee: u128,
    ) -> Result<(), AuctionError> {
        if (self.initialized.get()) {
            return Err(AuctionError::OwnerAlreadyInitialized(
                OwnerAlreadyInitialized {},
            ));
        }
        self.registry.set(registry);
        self.decrypter.set(decrypter);
        self.ibe_contract.set(ibe_contract);
        self.decrypter_contract.set(decrypter_contract);
        self.mac_contract.set(mac_contract);
        self.deadline.set(U256::from(deadline));
        self.id.set(U256::from(id));
        self.fee.set(U256::from(fee));
        self.bids = unsafe { StorageVec::new(U256::ZERO, 0) };
        self.owner.set(msg::sender());
        self.initialized.set(true);
        self.finished.set(false);
        let registry: IRegistry = IRegistry::new(*self.registry);
        let owner = self.owner.clone();
       // let _ = registry.add_contract(self, address(), owner);
        Ok(())
    }

    pub fn check_condition(&mut self) -> Result<bool, Vec<u8>> {
        if block::timestamp().to_string() == self.deadline.to_string() {
            return Ok(true);
        }
        Ok(false)
    }
    #[payable]
    pub fn submit_enc_bid(
        &mut self,
        tx: Vec<u8>,
        condition: String,
    ) -> Result<Vec<u8>, AuctionError> {
        // self.if_initialized()?;
        // self.if_not_finished()?;
        // if msg::value() < *self.fee {
        //     return Err(AuctionError::NotEnoughFee(NotEnoughFee {}));
        // }
        let c = self.id.to_string() + &self.deadline.to_string();
        if condition == c {
            let mut inner_vec: StorageGuardMut<'_, EncBid> = self.bids.grow();
            inner_vec.tx_.set_bytes(tx);
            inner_vec.condition.set_str(condition.clone());
            inner_vec.sender.set(msg::sender());
            return Ok(self.bids.len().to_string().as_bytes().to_vec());
        }
        return Err(AuctionError::ConditionNotMatched(ConditionNotMatched {}));
    }

    pub fn submit_key(&mut self, condition: String, key: Vec<u8>) -> Result<Vec<u8>, AuctionError> {
        // self.if_initialized()?;
        // self.if_not_finished()?;
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
               
                let bid_string =
                    String::from_utf8(plain_bid.clone()).expect("Invalid UTF-8 sequence");
                let val = string_to_u128(bid_string.as_str()).unwrap();
                if val > winner_bid {
                    winner_bid = val;
                    winner = sender;
                    self.finished.set(true);
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
        //self.if_initialized()?;
        let decrypter: IDecrypter = IDecrypter::new(*self.decrypter);

        let plain_tx = decrypter
            .decrypt(self, tx, key.clone(), ibe_c, dec_c, mac_c)
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

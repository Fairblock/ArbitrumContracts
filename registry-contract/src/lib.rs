// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
use stylus_sdk::storage::*;
use stylus_sdk::{alloy_primitives::*, alloy_sol_types::sol, evm, prelude::*};

extern crate alloc;

sol_storage! {
    #[entrypoint]
    pub struct Registry {
       StorageVec<Contract> list;
       uint256 count;

    }

    pub struct Contract {
        StorageAddress contract_address;
        StorageAddress owner;
        StorageString id;
        
    }


}

sol_interface! {
    interface IContract {
        function checkCondition() external returns (string[] memory);
        function submitKey(string calldata k, string calldata condition) external returns (uint8[] memory);
    }
}

sol! {
    event RegisterContract(address indexed owner, address indexed contract, uint256 id);
    event ExecuteContract(address indexed contract, string condition, bool satisfaction );
}

#[external]
impl Registry {
    fn get_contract(&self, addr: Address) -> Result<(Address, String), Vec<u8>> {
        let mut index: usize = 0;
        for i in 0..self.list.len() {
            if *self.list.get(i).unwrap().contract_address == addr {
                index = i;
            }
        }
        let c = &self.list.get(index).unwrap();
        Ok((*c.owner, c.id.get_string()))
    }
    fn get_all_contracts(&self) -> Result<Vec<Address>, Vec<u8>> {
        let mut address_list = vec![];
        for i in 0..self.list.len() {
            address_list.push(*self.list.get(i).unwrap().contract_address)
        }

        Ok(address_list)
    }

    fn add_contract(
        &mut self,
        addr_con: Address,
        addr_own: Address,
        id: String
    ) -> Result<U256, Vec<u8>> {
        let mut id_count = self.count.get();
        id_count = id_count + U256::from(1);

        self.count.set(id_count);
        let mut inner_vec: StorageGuardMut<'_, Contract> = self.list.grow();
        inner_vec.contract_address.set(addr_con);
       
        inner_vec.owner.set(addr_own);
        inner_vec.id.set_str(id);

        evm::log(RegisterContract {
            owner: addr_own,
            contract: addr_con,
            id: id_count,
        });
        Ok(id_count)
    }

    fn check_condition_proxy(&mut self, _contract: Address) -> Result<Vec<String>, Vec<u8>> {
        let contract: IContract = IContract::new(_contract);
        let rs = contract.check_condition(self).unwrap();
    
        Ok(rs)
    }

    fn send_key(&mut self, key: String, id: String) -> Result<Vec<u8>, Vec<u8>> {
        let mut _address = Address::ZERO;
        match id.find('-') {
            Some(index) => {
                let contract_id = &id[0..index];
                for i in 0..self.list.len() {
                    unsafe {
                        if self.list.get(i).unwrap().into_raw().id.get_string() == contract_id {
                            _address = *self.list.get(i).unwrap().contract_address;
                            break;
                        }
                    }
                }
            },
            None => return Ok(vec![]),
        }
        
       
        let c = IContract::new(_address);
        let res = c.submit_key(self, key, id);
        Ok(res.unwrap())
    }
}

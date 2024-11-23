#![no_std]
#![no_main]

use core::default::Default;

use contract_derive::{contract, payable};
use eth_riscv_runtime::types::Mapping;

use alloy_core::primitives::{Address, address, U256};

extern crate alloc;
use alloc::string::String;

mod erc20;
use erc20::Erc20;

#[derive(Default)]
pub struct MyToken {
    balances: Mapping<Address, u64>,
    allowances: Mapping<Address, Mapping<Address, u64>>,
    total_supply: U256,
    name: String,
    symbol: String,
    decimals: u8,
}

#[contract]
impl Erc20 for MyToken {
    fn balances(&self) -> Mapping<Address, u64> {
        self.balances
    }

    fn allowances(&self) -> Mapping<Address, Mapping<Address, u64>> {
        self.allowances
    }

    fn total_supply(&self) -> U256 {
        self.total_supply
    }
}

use alloy_core::primitives::{Address, U256};
use eth_riscv_runtime::{types::Mapping, msg_sender, revert};

pub trait Erc20 {
    fn total_supply(&self) -> U256;

    fn balances(&self) -> Mapping<Address, u64>;

    fn allowances(&self) -> Mapping<Address, Mapping<Address, u64>>;

    fn transfer(&self, recipient: Address, amount: u64) -> bool {
        let from = msg_sender();
        let from_balance = self.balances().read(from);
        let to_balance = self.balances().read(recipient);
        if from == recipient || from_balance < amount {
            revert();
        }

        self.balances().write(from, from_balance - amount);
        self.balances().write(recipient, to_balance + amount);

        true
    }

    fn allowance(&self, owner: Address, spender: Address) -> u64 {
        self.allowances().read(owner).read(spender)
    }

    fn approve(&self, spender: Address, amount: u64) -> bool {
        let spender_allowances = self.allowances().read(msg_sender());
        spender_allowances.write(spender, amount);
        true
    }

    fn transfer_from(&self, sender: Address, recipient: Address, amount: u64) -> bool {
        let spender_allowances = self.allowances().read(sender).read(msg_sender());
        let sender_balance = self.balances().read(sender);
        let recipient_balance = self.balances().read(recipient);

        self.allowances().read(sender).write(msg_sender(), spender_allowances - amount);
        self.balances().write(sender, sender_balance - amount);
        self.balances().write(recipient, recipient_balance + amount);

        true
    }

    fn mint(&self, to: Address, value: u64) {
        let owner = msg_sender();
        let to_balance = self.balances().read(to);
        self.balances().write(to, to_balance + value);
    }
}
use crate::{
    quantities::UnsidedAtoms,
    state::{erc20_store::ERC20Store, eth_store::EthStore},
};

pub trait MakerStore {
    fn reduce_locked(&mut self, amount: UnsidedAtoms);
    fn add_free(&mut self, amount: UnsidedAtoms);
}

impl MakerStore for EthStore {
    fn reduce_locked(&mut self, amount: UnsidedAtoms) {
        self.atoms_locked -= amount;
    }

    fn add_free(&mut self, amount: UnsidedAtoms) {
        self.atoms_free += amount;
    }
}

impl MakerStore for ERC20Store {
    fn reduce_locked(&mut self, amount: UnsidedAtoms) {
        self.atoms_locked -= amount;
    }

    fn add_free(&mut self, amount: UnsidedAtoms) {
        self.atoms_free += amount;
    }
}

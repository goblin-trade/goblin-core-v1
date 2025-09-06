use crate::{
    quantities::Atoms,
    state::{ERC20Store, EthStore},
};

pub trait MakerStore {
    fn reduce_locked(&mut self, amount: Atoms);
    fn add_free(&mut self, amount: Atoms);
}

impl MakerStore for EthStore {
    fn reduce_locked(&mut self, amount: Atoms) {
        self.atoms_locked -= amount;
    }

    fn add_free(&mut self, amount: Atoms) {
        self.atoms_free += amount;
    }
}

impl MakerStore for ERC20Store {
    fn reduce_locked(&mut self, amount: Atoms) {
        self.atoms_locked -= amount;
    }

    fn add_free(&mut self, amount: Atoms) {
        self.atoms_free += amount;
    }
}

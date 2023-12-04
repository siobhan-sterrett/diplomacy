use core::fmt::{Debug, Display};
use core::hash::Hash;

pub trait Space: Copy + Clone + Debug + Display + PartialEq + Eq + Hash {
    fn province(&self) -> super::province::Province;
    fn neighbors(&self) -> &[Self];
}

impl Space for super::army_space::ArmySpace {
    fn province(&self) -> super::province::Province {
        self.province()
    }

    fn neighbors(&self) -> &[Self] {
        self.neighbors()
    }
}

impl Space for super::fleet_space::FleetSpace {
    fn province(&self) -> super::province::Province {
        self.province()
    }

    fn neighbors(&self) -> &[Self] {
        self.neighbors()
    }
}

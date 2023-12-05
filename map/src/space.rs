use crate::province::Province;

use super::army_space::ArmySpace;
use super::fleet_space::FleetSpace;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Space {
    Army(ArmySpace),
    Fleet(FleetSpace),
}

impl Space {
    pub fn ident(&self) -> &'static str {
        match &self {
            &Space::Army(space) => space.ident(),
            &Space::Fleet(space) => space.ident(),
        }
    }

    pub fn province(&self) -> Province {
        match &self {
            &Space::Army(space) => space.province(),
            &Space::Fleet(space) => space.province(),
        }
    }

    pub fn neighbors(&self) -> Vec<Space> {
        match &self {
            &Space::Army(space) => space
                .neighbors()
                .iter()
                .map(|space| Space::Army(*space))
                .collect(),
            &Space::Fleet(space) => space
                .neighbors()
                .iter()
                .map(|space| Space::Fleet(*space))
                .collect(),
        }
    }
}

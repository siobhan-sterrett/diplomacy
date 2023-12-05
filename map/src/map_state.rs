use serde::Deserialize;
use std::collections::HashMap;
use std::ops::Deref;

use crate::{
    army_space::ArmySpace, country::Country, fleet_space::FleetSpace, supply_center::SupplyCenter,
};

#[derive(Deserialize)]
pub struct MapState(HashMap<Country, MapStatePerCountry>);

impl Deref for MapState {
    type Target = HashMap<Country, MapStatePerCountry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize)]
pub struct MapStatePerCountry {
    pub armies: Vec<ArmySpace>,
    pub fleets: Vec<FleetSpace>,
    pub supply_centers: Vec<SupplyCenter>,
}

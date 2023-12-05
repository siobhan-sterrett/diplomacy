#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FleetSpace {
    AdriaticSea,
    AegeanSea,
    Albania,
    Ankara,
    Apulia,
    Armenia,
    BalticSea,
    BarentsSea,
    Belgium,
    Berlin,
    BlackSea,
    Brest,
    BulgariaEastCoast,
    BulgariaSouthCoast,
    Clyde,
    Constantinople,
    Denmark,
    EasternMediterranean,
    Edinburgh,
    EnglishChannel,
    Finland,
    Gascony,
    Greece,
    GulfOfBothnia,
    GulfOfLyon,
    HeligolandBight,
    Holland,
    IonianSea,
    IrishSea,
    Kiel,
    Liverpool,
    Livonia,
    London,
    Marseilles,
    MidAtlanticOcean,
    Naples,
    NorthAfrica,
    NorthAtlanticOcean,
    NorthSea,
    Norway,
    NorwegianSea,
    Picardy,
    Piedmont,
    Portugal,
    Prussia,
    Rome,
    Rumania,
    Sevastopol,
    Skagerrak,
    Smyrna,
    SpainNorthCoast,
    SpainSouthCoast,
    StPetersburgNorthCoast,
    StPetersburgSouthCoast,
    Sweden,
    Syria,
    Trieste,
    Tunis,
    Tuscany,
    TyrrhenianSea,
    Venice,
    Wales,
    WesternMediterranean,
    Yorkshire,
}

impl core::fmt::Display for FleetSpace {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match &self {
            &FleetSpace::AdriaticSea => write!(f, "{}", "Adriatic Sea"),
            &FleetSpace::AegeanSea => write!(f, "{}", "Aegean Sea"),
            &FleetSpace::Albania => write!(f, "{}", "Albania"),
            &FleetSpace::Ankara => write!(f, "{}", "Ankara"),
            &FleetSpace::Apulia => write!(f, "{}", "Apulia"),
            &FleetSpace::Armenia => write!(f, "{}", "Armenia"),
            &FleetSpace::BalticSea => write!(f, "{}", "Baltic Sea"),
            &FleetSpace::BarentsSea => write!(f, "{}", "Barents Sea"),
            &FleetSpace::Belgium => write!(f, "{}", "Belgium"),
            &FleetSpace::Berlin => write!(f, "{}", "Berlin"),
            &FleetSpace::BlackSea => write!(f, "{}", "Black Sea"),
            &FleetSpace::Brest => write!(f, "{}", "Brest"),
            &FleetSpace::BulgariaEastCoast => write!(f, "{}", "Bulgaria (East Coast)"),
            &FleetSpace::BulgariaSouthCoast => write!(f, "{}", "Bulgaria (South Coast)"),
            &FleetSpace::Clyde => write!(f, "{}", "Clyde"),
            &FleetSpace::Constantinople => write!(f, "{}", "Constantinople"),
            &FleetSpace::Denmark => write!(f, "{}", "Denmark"),
            &FleetSpace::EasternMediterranean => write!(f, "{}", "Eastern Mediterranean"),
            &FleetSpace::Edinburgh => write!(f, "{}", "Edinburgh"),
            &FleetSpace::EnglishChannel => write!(f, "{}", "English Channel"),
            &FleetSpace::Finland => write!(f, "{}", "Finland"),
            &FleetSpace::Gascony => write!(f, "{}", "Gascony"),
            &FleetSpace::Greece => write!(f, "{}", "Greece"),
            &FleetSpace::GulfOfBothnia => write!(f, "{}", "Gulf of Bothnia"),
            &FleetSpace::GulfOfLyon => write!(f, "{}", "Gulf of Lyon"),
            &FleetSpace::HeligolandBight => write!(f, "{}", "Heligoland Bight"),
            &FleetSpace::Holland => write!(f, "{}", "Holland"),
            &FleetSpace::IonianSea => write!(f, "{}", "Ionian Sea"),
            &FleetSpace::IrishSea => write!(f, "{}", "Irish Sea"),
            &FleetSpace::Kiel => write!(f, "{}", "Kiel"),
            &FleetSpace::Liverpool => write!(f, "{}", "Liverpool"),
            &FleetSpace::Livonia => write!(f, "{}", "Livonia"),
            &FleetSpace::London => write!(f, "{}", "London"),
            &FleetSpace::Marseilles => write!(f, "{}", "Marseilles"),
            &FleetSpace::MidAtlanticOcean => write!(f, "{}", "Mid-Atlantic Ocean"),
            &FleetSpace::Naples => write!(f, "{}", "Naples"),
            &FleetSpace::NorthAfrica => write!(f, "{}", "North Africa"),
            &FleetSpace::NorthAtlanticOcean => write!(f, "{}", "North Atlantic Ocean"),
            &FleetSpace::NorthSea => write!(f, "{}", "North Sea"),
            &FleetSpace::Norway => write!(f, "{}", "Norway"),
            &FleetSpace::NorwegianSea => write!(f, "{}", "Norwegian Sea"),
            &FleetSpace::Picardy => write!(f, "{}", "Picardy"),
            &FleetSpace::Piedmont => write!(f, "{}", "Piedmont"),
            &FleetSpace::Portugal => write!(f, "{}", "Portugal"),
            &FleetSpace::Prussia => write!(f, "{}", "Prussia"),
            &FleetSpace::Rome => write!(f, "{}", "Rome"),
            &FleetSpace::Rumania => write!(f, "{}", "Rumania"),
            &FleetSpace::Sevastopol => write!(f, "{}", "Sevastopol"),
            &FleetSpace::Skagerrak => write!(f, "{}", "Skagerrak"),
            &FleetSpace::Smyrna => write!(f, "{}", "Smyrna"),
            &FleetSpace::SpainNorthCoast => write!(f, "{}", "Spain (North Coast)"),
            &FleetSpace::SpainSouthCoast => write!(f, "{}", "Spain (South Coast)"),
            &FleetSpace::StPetersburgNorthCoast => {
                write!(f, "{}", "St Petersburg (North Coast)")
            }
            &FleetSpace::StPetersburgSouthCoast => {
                write!(f, "{}", "St Petersburg (South Coast)")
            }
            &FleetSpace::Sweden => write!(f, "{}", "Sweden"),
            &FleetSpace::Syria => write!(f, "{}", "Syria"),
            &FleetSpace::Trieste => write!(f, "{}", "Trieste"),
            &FleetSpace::Tunis => write!(f, "{}", "Tunis"),
            &FleetSpace::Tuscany => write!(f, "{}", "Tuscany"),
            &FleetSpace::TyrrhenianSea => write!(f, "{}", "Tyrrhenian Sea"),
            &FleetSpace::Venice => write!(f, "{}", "Venice"),
            &FleetSpace::Wales => write!(f, "{}", "Wales"),
            &FleetSpace::WesternMediterranean => write!(f, "{}", "Western Mediterranean"),
            &FleetSpace::Yorkshire => write!(f, "{}", "Yorkshire"),
        }
    }
}

impl FleetSpace {
    pub fn province(&self) -> super::province::Province {
        match &self {
            FleetSpace::AdriaticSea => super::province::Province::AdriaticSea,
            FleetSpace::AegeanSea => super::province::Province::AegeanSea,
            FleetSpace::Albania => super::province::Province::Albania,
            FleetSpace::Ankara => super::province::Province::Ankara,
            FleetSpace::Apulia => super::province::Province::Apulia,
            FleetSpace::Armenia => super::province::Province::Armenia,
            FleetSpace::BalticSea => super::province::Province::BalticSea,
            FleetSpace::BarentsSea => super::province::Province::BarentsSea,
            FleetSpace::Belgium => super::province::Province::Belgium,
            FleetSpace::Berlin => super::province::Province::Berlin,
            FleetSpace::BlackSea => super::province::Province::BlackSea,
            FleetSpace::Brest => super::province::Province::Brest,
            FleetSpace::BulgariaEastCoast => super::province::Province::Bulgaria,
            FleetSpace::BulgariaSouthCoast => super::province::Province::Bulgaria,
            FleetSpace::Clyde => super::province::Province::Clyde,
            FleetSpace::Constantinople => super::province::Province::Constantinople,
            FleetSpace::Denmark => super::province::Province::Denmark,
            FleetSpace::EasternMediterranean => {
                super::province::Province::EasternMediterranean
            }
            FleetSpace::Edinburgh => super::province::Province::Edinburgh,
            FleetSpace::EnglishChannel => super::province::Province::EnglishChannel,
            FleetSpace::Finland => super::province::Province::Finland,
            FleetSpace::Gascony => super::province::Province::Gascony,
            FleetSpace::Greece => super::province::Province::Greece,
            FleetSpace::GulfOfBothnia => super::province::Province::GulfOfBothnia,
            FleetSpace::GulfOfLyon => super::province::Province::GulfOfLyon,
            FleetSpace::HeligolandBight => super::province::Province::HeligolandBight,
            FleetSpace::Holland => super::province::Province::Holland,
            FleetSpace::IonianSea => super::province::Province::IonianSea,
            FleetSpace::IrishSea => super::province::Province::IrishSea,
            FleetSpace::Kiel => super::province::Province::Kiel,
            FleetSpace::Liverpool => super::province::Province::Liverpool,
            FleetSpace::Livonia => super::province::Province::Livonia,
            FleetSpace::London => super::province::Province::London,
            FleetSpace::Marseilles => super::province::Province::Marseilles,
            FleetSpace::MidAtlanticOcean => super::province::Province::MidAtlanticOcean,
            FleetSpace::Naples => super::province::Province::Naples,
            FleetSpace::NorthAfrica => super::province::Province::NorthAfrica,
            FleetSpace::NorthAtlanticOcean => {
                super::province::Province::NorthAtlanticOcean
            }
            FleetSpace::NorthSea => super::province::Province::NorthSea,
            FleetSpace::Norway => super::province::Province::Norway,
            FleetSpace::NorwegianSea => super::province::Province::NorwegianSea,
            FleetSpace::Picardy => super::province::Province::Picardy,
            FleetSpace::Piedmont => super::province::Province::Piedmont,
            FleetSpace::Portugal => super::province::Province::Portugal,
            FleetSpace::Prussia => super::province::Province::Prussia,
            FleetSpace::Rome => super::province::Province::Rome,
            FleetSpace::Rumania => super::province::Province::Rumania,
            FleetSpace::Sevastopol => super::province::Province::Sevastopol,
            FleetSpace::Skagerrak => super::province::Province::Skagerrak,
            FleetSpace::Smyrna => super::province::Province::Smyrna,
            FleetSpace::SpainNorthCoast => super::province::Province::Spain,
            FleetSpace::SpainSouthCoast => super::province::Province::Spain,
            FleetSpace::StPetersburgNorthCoast => super::province::Province::StPetersburg,
            FleetSpace::StPetersburgSouthCoast => super::province::Province::StPetersburg,
            FleetSpace::Sweden => super::province::Province::Sweden,
            FleetSpace::Syria => super::province::Province::Syria,
            FleetSpace::Trieste => super::province::Province::Trieste,
            FleetSpace::Tunis => super::province::Province::Tunis,
            FleetSpace::Tuscany => super::province::Province::Tuscany,
            FleetSpace::TyrrhenianSea => super::province::Province::TyrrhenianSea,
            FleetSpace::Venice => super::province::Province::Venice,
            FleetSpace::Wales => super::province::Province::Wales,
            FleetSpace::WesternMediterranean => {
                super::province::Province::WesternMediterranean
            }
            FleetSpace::Yorkshire => super::province::Province::Yorkshire,
        }
    }
    pub fn neighbors(&self) -> &'static [FleetSpace] {
        match &self {
            FleetSpace::AdriaticSea => {
                &[
                    FleetSpace::Trieste,
                    FleetSpace::Venice,
                    FleetSpace::Apulia,
                    FleetSpace::Albania,
                    FleetSpace::IonianSea,
                ]
            }
            FleetSpace::AegeanSea => {
                &[
                    FleetSpace::Greece,
                    FleetSpace::BulgariaSouthCoast,
                    FleetSpace::Constantinople,
                    FleetSpace::Smyrna,
                    FleetSpace::EasternMediterranean,
                    FleetSpace::IonianSea,
                ]
            }
            FleetSpace::Albania => {
                &[
                    FleetSpace::AdriaticSea,
                    FleetSpace::Trieste,
                    FleetSpace::Greece,
                    FleetSpace::IonianSea,
                ]
            }
            FleetSpace::Ankara => {
                &[
                    FleetSpace::BlackSea,
                    FleetSpace::Armenia,
                    FleetSpace::Smyrna,
                    FleetSpace::Constantinople,
                ]
            }
            FleetSpace::Apulia => {
                &[
                    FleetSpace::AdriaticSea,
                    FleetSpace::IonianSea,
                    FleetSpace::Naples,
                    FleetSpace::Rome,
                    FleetSpace::Venice,
                ]
            }
            FleetSpace::Armenia => {
                &[
                    FleetSpace::BlackSea,
                    FleetSpace::Sevastopol,
                    FleetSpace::Syria,
                    FleetSpace::Ankara,
                    FleetSpace::Smyrna,
                ]
            }
            FleetSpace::BalticSea => {
                &[
                    FleetSpace::Sweden,
                    FleetSpace::GulfOfBothnia,
                    FleetSpace::Livonia,
                    FleetSpace::Prussia,
                    FleetSpace::Berlin,
                    FleetSpace::Kiel,
                    FleetSpace::Denmark,
                ]
            }
            FleetSpace::BarentsSea => {
                &[
                    FleetSpace::StPetersburgNorthCoast,
                    FleetSpace::Norway,
                    FleetSpace::NorwegianSea,
                ]
            }
            FleetSpace::Belgium => {
                &[
                    FleetSpace::Holland,
                    FleetSpace::Picardy,
                    FleetSpace::EnglishChannel,
                    FleetSpace::NorthSea,
                ]
            }
            FleetSpace::Berlin => {
                &[FleetSpace::BalticSea, FleetSpace::Prussia, FleetSpace::Kiel]
            }
            FleetSpace::BlackSea => {
                &[
                    FleetSpace::Sevastopol,
                    FleetSpace::Armenia,
                    FleetSpace::Ankara,
                    FleetSpace::Constantinople,
                    FleetSpace::BulgariaEastCoast,
                    FleetSpace::Rumania,
                ]
            }
            FleetSpace::Brest => {
                &[
                    FleetSpace::EnglishChannel,
                    FleetSpace::MidAtlanticOcean,
                    FleetSpace::Picardy,
                    FleetSpace::Gascony,
                ]
            }
            FleetSpace::BulgariaEastCoast => {
                &[FleetSpace::Constantinople, FleetSpace::BlackSea, FleetSpace::Rumania]
            }
            FleetSpace::BulgariaSouthCoast => {
                &[FleetSpace::Constantinople, FleetSpace::AegeanSea, FleetSpace::Greece]
            }
            FleetSpace::Clyde => {
                &[
                    FleetSpace::NorthAtlanticOcean,
                    FleetSpace::NorwegianSea,
                    FleetSpace::Edinburgh,
                    FleetSpace::Liverpool,
                ]
            }
            FleetSpace::Constantinople => {
                &[
                    FleetSpace::BlackSea,
                    FleetSpace::Ankara,
                    FleetSpace::Smyrna,
                    FleetSpace::BulgariaEastCoast,
                    FleetSpace::BulgariaSouthCoast,
                    FleetSpace::AegeanSea,
                ]
            }
            FleetSpace::Denmark => {
                &[
                    FleetSpace::BalticSea,
                    FleetSpace::Skagerrak,
                    FleetSpace::HeligolandBight,
                    FleetSpace::Kiel,
                    FleetSpace::NorthSea,
                    FleetSpace::Sweden,
                ]
            }
            FleetSpace::EasternMediterranean => {
                &[
                    FleetSpace::Syria,
                    FleetSpace::IonianSea,
                    FleetSpace::AegeanSea,
                    FleetSpace::Smyrna,
                ]
            }
            FleetSpace::Edinburgh => {
                &[
                    FleetSpace::Clyde,
                    FleetSpace::NorwegianSea,
                    FleetSpace::NorthSea,
                    FleetSpace::Yorkshire,
                    FleetSpace::Liverpool,
                ]
            }
            FleetSpace::EnglishChannel => {
                &[
                    FleetSpace::London,
                    FleetSpace::Belgium,
                    FleetSpace::Picardy,
                    FleetSpace::Brest,
                    FleetSpace::MidAtlanticOcean,
                    FleetSpace::IrishSea,
                    FleetSpace::Wales,
                    FleetSpace::NorthSea,
                ]
            }
            FleetSpace::Finland => {
                &[
                    FleetSpace::StPetersburgSouthCoast,
                    FleetSpace::Sweden,
                    FleetSpace::Norway,
                    FleetSpace::GulfOfBothnia,
                ]
            }
            FleetSpace::Gascony => {
                &[
                    FleetSpace::MidAtlanticOcean,
                    FleetSpace::SpainNorthCoast,
                    FleetSpace::Brest,
                    FleetSpace::Marseilles,
                ]
            }
            FleetSpace::Greece => {
                &[
                    FleetSpace::IonianSea,
                    FleetSpace::AegeanSea,
                    FleetSpace::Albania,
                    FleetSpace::BulgariaSouthCoast,
                ]
            }
            FleetSpace::GulfOfBothnia => {
                &[
                    FleetSpace::Sweden,
                    FleetSpace::Finland,
                    FleetSpace::Livonia,
                    FleetSpace::StPetersburgSouthCoast,
                    FleetSpace::BalticSea,
                ]
            }
            FleetSpace::GulfOfLyon => {
                &[
                    FleetSpace::Marseilles,
                    FleetSpace::Piedmont,
                    FleetSpace::Tuscany,
                    FleetSpace::TyrrhenianSea,
                    FleetSpace::WesternMediterranean,
                    FleetSpace::SpainSouthCoast,
                ]
            }
            FleetSpace::HeligolandBight => {
                &[
                    FleetSpace::Denmark,
                    FleetSpace::Kiel,
                    FleetSpace::Holland,
                    FleetSpace::NorthSea,
                ]
            }
            FleetSpace::Holland => {
                &[
                    FleetSpace::Belgium,
                    FleetSpace::NorthSea,
                    FleetSpace::Kiel,
                    FleetSpace::HeligolandBight,
                ]
            }
            FleetSpace::IonianSea => {
                &[
                    FleetSpace::Tunis,
                    FleetSpace::TyrrhenianSea,
                    FleetSpace::Naples,
                    FleetSpace::Apulia,
                    FleetSpace::AdriaticSea,
                    FleetSpace::Greece,
                    FleetSpace::Albania,
                    FleetSpace::AegeanSea,
                    FleetSpace::EasternMediterranean,
                ]
            }
            FleetSpace::IrishSea => {
                &[
                    FleetSpace::NorthAtlanticOcean,
                    FleetSpace::EnglishChannel,
                    FleetSpace::MidAtlanticOcean,
                    FleetSpace::Liverpool,
                    FleetSpace::Wales,
                ]
            }
            FleetSpace::Kiel => {
                &[
                    FleetSpace::HeligolandBight,
                    FleetSpace::Berlin,
                    FleetSpace::Holland,
                    FleetSpace::Denmark,
                    FleetSpace::BalticSea,
                ]
            }
            FleetSpace::Liverpool => {
                &[
                    FleetSpace::NorthAtlanticOcean,
                    FleetSpace::IrishSea,
                    FleetSpace::Clyde,
                    FleetSpace::Edinburgh,
                    FleetSpace::Yorkshire,
                    FleetSpace::Wales,
                ]
            }
            FleetSpace::Livonia => {
                &[
                    FleetSpace::BalticSea,
                    FleetSpace::GulfOfBothnia,
                    FleetSpace::StPetersburgSouthCoast,
                    FleetSpace::Prussia,
                ]
            }
            FleetSpace::London => {
                &[
                    FleetSpace::NorthSea,
                    FleetSpace::EnglishChannel,
                    FleetSpace::Wales,
                    FleetSpace::Yorkshire,
                ]
            }
            FleetSpace::Marseilles => {
                &[
                    FleetSpace::GulfOfLyon,
                    FleetSpace::SpainSouthCoast,
                    FleetSpace::Gascony,
                    FleetSpace::Piedmont,
                ]
            }
            FleetSpace::MidAtlanticOcean => {
                &[
                    FleetSpace::NorthAtlanticOcean,
                    FleetSpace::IrishSea,
                    FleetSpace::EnglishChannel,
                    FleetSpace::Brest,
                    FleetSpace::Gascony,
                    FleetSpace::SpainNorthCoast,
                    FleetSpace::SpainSouthCoast,
                    FleetSpace::Portugal,
                    FleetSpace::WesternMediterranean,
                    FleetSpace::NorthAfrica,
                ]
            }
            FleetSpace::Naples => {
                &[
                    FleetSpace::IonianSea,
                    FleetSpace::TyrrhenianSea,
                    FleetSpace::Apulia,
                    FleetSpace::Rome,
                ]
            }
            FleetSpace::NorthAfrica => {
                &[
                    FleetSpace::MidAtlanticOcean,
                    FleetSpace::WesternMediterranean,
                    FleetSpace::Tunis,
                ]
            }
            FleetSpace::NorthAtlanticOcean => {
                &[
                    FleetSpace::NorwegianSea,
                    FleetSpace::Clyde,
                    FleetSpace::Liverpool,
                    FleetSpace::IrishSea,
                    FleetSpace::MidAtlanticOcean,
                ]
            }
            FleetSpace::NorthSea => {
                &[
                    FleetSpace::NorwegianSea,
                    FleetSpace::Skagerrak,
                    FleetSpace::Denmark,
                    FleetSpace::HeligolandBight,
                    FleetSpace::Holland,
                    FleetSpace::Belgium,
                    FleetSpace::EnglishChannel,
                    FleetSpace::London,
                    FleetSpace::Yorkshire,
                    FleetSpace::Edinburgh,
                    FleetSpace::Norway,
                ]
            }
            FleetSpace::Norway => {
                &[
                    FleetSpace::NorwegianSea,
                    FleetSpace::NorthSea,
                    FleetSpace::Sweden,
                    FleetSpace::Finland,
                    FleetSpace::Skagerrak,
                    FleetSpace::BarentsSea,
                    FleetSpace::StPetersburgNorthCoast,
                ]
            }
            FleetSpace::NorwegianSea => {
                &[
                    FleetSpace::NorthAtlanticOcean,
                    FleetSpace::Norway,
                    FleetSpace::BarentsSea,
                    FleetSpace::NorthSea,
                    FleetSpace::Clyde,
                    FleetSpace::Edinburgh,
                ]
            }
            FleetSpace::Picardy => {
                &[FleetSpace::EnglishChannel, FleetSpace::Belgium, FleetSpace::Brest]
            }
            FleetSpace::Piedmont => {
                &[
                    FleetSpace::Marseilles,
                    FleetSpace::GulfOfLyon,
                    FleetSpace::Venice,
                    FleetSpace::Tuscany,
                ]
            }
            FleetSpace::Portugal => {
                &[
                    FleetSpace::MidAtlanticOcean,
                    FleetSpace::SpainNorthCoast,
                    FleetSpace::SpainSouthCoast,
                ]
            }
            FleetSpace::Prussia => {
                &[FleetSpace::BalticSea, FleetSpace::Livonia, FleetSpace::Berlin]
            }
            FleetSpace::Rome => {
                &[
                    FleetSpace::TyrrhenianSea,
                    FleetSpace::Naples,
                    FleetSpace::Tuscany,
                    FleetSpace::Venice,
                    FleetSpace::Apulia,
                ]
            }
            FleetSpace::Rumania => {
                &[
                    FleetSpace::BlackSea,
                    FleetSpace::BulgariaEastCoast,
                    FleetSpace::Sevastopol,
                ]
            }
            FleetSpace::Sevastopol => {
                &[FleetSpace::Armenia, FleetSpace::BlackSea, FleetSpace::Rumania]
            }
            FleetSpace::Skagerrak => {
                &[
                    FleetSpace::Norway,
                    FleetSpace::Sweden,
                    FleetSpace::Denmark,
                    FleetSpace::NorthSea,
                ]
            }
            FleetSpace::Smyrna => {
                &[
                    FleetSpace::EasternMediterranean,
                    FleetSpace::AegeanSea,
                    FleetSpace::Constantinople,
                    FleetSpace::Ankara,
                    FleetSpace::Armenia,
                    FleetSpace::Syria,
                ]
            }
            FleetSpace::SpainNorthCoast => {
                &[
                    FleetSpace::Gascony,
                    FleetSpace::MidAtlanticOcean,
                    FleetSpace::Portugal,
                ]
            }
            FleetSpace::SpainSouthCoast => {
                &[
                    FleetSpace::Portugal,
                    FleetSpace::MidAtlanticOcean,
                    FleetSpace::WesternMediterranean,
                    FleetSpace::GulfOfLyon,
                    FleetSpace::Marseilles,
                ]
            }
            FleetSpace::StPetersburgNorthCoast => {
                &[FleetSpace::Norway, FleetSpace::BarentsSea]
            }
            FleetSpace::StPetersburgSouthCoast => {
                &[FleetSpace::Livonia, FleetSpace::GulfOfBothnia, FleetSpace::Finland]
            }
            FleetSpace::Sweden => {
                &[
                    FleetSpace::GulfOfBothnia,
                    FleetSpace::Finland,
                    FleetSpace::Norway,
                    FleetSpace::BalticSea,
                    FleetSpace::Skagerrak,
                    FleetSpace::Denmark,
                ]
            }
            FleetSpace::Syria => {
                &[
                    FleetSpace::Armenia,
                    FleetSpace::Smyrna,
                    FleetSpace::EasternMediterranean,
                ]
            }
            FleetSpace::Trieste => {
                &[FleetSpace::AdriaticSea, FleetSpace::Venice, FleetSpace::Albania]
            }
            FleetSpace::Tunis => {
                &[
                    FleetSpace::NorthAfrica,
                    FleetSpace::WesternMediterranean,
                    FleetSpace::IonianSea,
                    FleetSpace::TyrrhenianSea,
                ]
            }
            FleetSpace::Tuscany => {
                &[
                    FleetSpace::GulfOfLyon,
                    FleetSpace::Piedmont,
                    FleetSpace::Venice,
                    FleetSpace::Rome,
                    FleetSpace::TyrrhenianSea,
                ]
            }
            FleetSpace::TyrrhenianSea => {
                &[
                    FleetSpace::GulfOfLyon,
                    FleetSpace::WesternMediterranean,
                    FleetSpace::Tunis,
                    FleetSpace::Tuscany,
                    FleetSpace::Rome,
                    FleetSpace::Naples,
                    FleetSpace::IonianSea,
                ]
            }
            FleetSpace::Venice => {
                &[
                    FleetSpace::Piedmont,
                    FleetSpace::Trieste,
                    FleetSpace::AdriaticSea,
                    FleetSpace::Apulia,
                    FleetSpace::Tuscany,
                    FleetSpace::Rome,
                ]
            }
            FleetSpace::Wales => {
                &[
                    FleetSpace::IrishSea,
                    FleetSpace::EnglishChannel,
                    FleetSpace::London,
                    FleetSpace::Yorkshire,
                    FleetSpace::Liverpool,
                ]
            }
            FleetSpace::WesternMediterranean => {
                &[
                    FleetSpace::NorthAfrica,
                    FleetSpace::MidAtlanticOcean,
                    FleetSpace::GulfOfLyon,
                    FleetSpace::SpainSouthCoast,
                    FleetSpace::Tunis,
                    FleetSpace::TyrrhenianSea,
                ]
            }
            FleetSpace::Yorkshire => {
                &[
                    FleetSpace::Liverpool,
                    FleetSpace::Edinburgh,
                    FleetSpace::London,
                    FleetSpace::Wales,
                    FleetSpace::NorthSea,
                ]
            }
        }
    }
}


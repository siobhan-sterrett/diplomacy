use crate::country::Country;
use crate::terrain::Terrain;
use std::fmt;
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Province {
    Bohemia,
    Budapest,
    Galicia,
    Trieste,
    Tyrolia,
    Vienna,
    Clyde,
    Edinburgh,
    Liverpool,
    London,
    Wales,
    Yorkshire,
    Brest,
    Burgundy,
    Gascony,
    Marseilles,
    Paris,
    Picardy,
    Berlin,
    Kiel,
    Munich,
    Prussia,
    Ruhr,
    Silesia,
    Apulia,
    Naples,
    Piedmont,
    Rome,
    Tuscany,
    Venice,
    Livonia,
    Moscow,
    Sevastopol,
    StPetersburg,
    Ukraine,
    Warsaw,
    Ankara,
    Armenia,
    Constantinople,
    Smyrna,
    Syria,
    Albania,
    Belgium,
    Bulgaria,
    Finland,
    Greece,
    Holland,
    Norway,
    NorthAfrica,
    Portugal,
    Rumania,
    Serbia,
    Spain,
    Sweden,
    Tunis,
    Denmark,
    AdriaticSea,
    AegeanSea,
    BalticSea,
    BarentsSea,
    BlackSea,
    EasternMediterranean,
    EnglishChannel,
    GulfOfBothnia,
    GulfOfLyon,
    HeligolandBight,
    IonianSea,
    IrishSea,
    MidAtlanticOcean,
    NorthAtlanticOcean,
    NorthSea,
    NorwegianSea,
    Skagerrak,
    TyrrhenianSea,
    WesternMediterranean,
}
impl fmt::Display for Province {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            &Province::Bohemia => write!(f, "{}", "Bohemia"),
            &Province::Budapest => write!(f, "{}", "Budapest"),
            &Province::Galicia => write!(f, "{}", "Galicia"),
            &Province::Trieste => write!(f, "{}", "Trieste"),
            &Province::Tyrolia => write!(f, "{}", "Tyrolia"),
            &Province::Vienna => write!(f, "{}", "Vienna"),
            &Province::Clyde => write!(f, "{}", "Clyde"),
            &Province::Edinburgh => write!(f, "{}", "Edinburgh"),
            &Province::Liverpool => write!(f, "{}", "Liverpool"),
            &Province::London => write!(f, "{}", "London"),
            &Province::Wales => write!(f, "{}", "Wales"),
            &Province::Yorkshire => write!(f, "{}", "Yorkshire"),
            &Province::Brest => write!(f, "{}", "Brest"),
            &Province::Burgundy => write!(f, "{}", "Burgundy"),
            &Province::Gascony => write!(f, "{}", "Gascony"),
            &Province::Marseilles => write!(f, "{}", "Marseilles"),
            &Province::Paris => write!(f, "{}", "Paris"),
            &Province::Picardy => write!(f, "{}", "Picardy"),
            &Province::Berlin => write!(f, "{}", "Berlin"),
            &Province::Kiel => write!(f, "{}", "Kiel"),
            &Province::Munich => write!(f, "{}", "Munich"),
            &Province::Prussia => write!(f, "{}", "Prussia"),
            &Province::Ruhr => write!(f, "{}", "Ruhr"),
            &Province::Silesia => write!(f, "{}", "Silesia"),
            &Province::Apulia => write!(f, "{}", "Apulia"),
            &Province::Naples => write!(f, "{}", "Naples"),
            &Province::Piedmont => write!(f, "{}", "Piedmont"),
            &Province::Rome => write!(f, "{}", "Rome"),
            &Province::Tuscany => write!(f, "{}", "Tuscany"),
            &Province::Venice => write!(f, "{}", "Venice"),
            &Province::Livonia => write!(f, "{}", "Livonia"),
            &Province::Moscow => write!(f, "{}", "Moscow"),
            &Province::Sevastopol => write!(f, "{}", "Sevastopol"),
            &Province::StPetersburg => write!(f, "{}", "St Petersburg"),
            &Province::Ukraine => write!(f, "{}", "Ukraine"),
            &Province::Warsaw => write!(f, "{}", "Warsaw"),
            &Province::Ankara => write!(f, "{}", "Ankara"),
            &Province::Armenia => write!(f, "{}", "Armenia"),
            &Province::Constantinople => write!(f, "{}", "Constantinople"),
            &Province::Smyrna => write!(f, "{}", "Smyrna"),
            &Province::Syria => write!(f, "{}", "Syria"),
            &Province::Albania => write!(f, "{}", "Albania"),
            &Province::Belgium => write!(f, "{}", "Belgium"),
            &Province::Bulgaria => write!(f, "{}", "Bulgaria"),
            &Province::Finland => write!(f, "{}", "Finland"),
            &Province::Greece => write!(f, "{}", "Greece"),
            &Province::Holland => write!(f, "{}", "Holland"),
            &Province::Norway => write!(f, "{}", "Norway"),
            &Province::NorthAfrica => write!(f, "{}", "North Africa"),
            &Province::Portugal => write!(f, "{}", "Portugal"),
            &Province::Rumania => write!(f, "{}", "Rumania"),
            &Province::Serbia => write!(f, "{}", "Serbia"),
            &Province::Spain => write!(f, "{}", "Spain"),
            &Province::Sweden => write!(f, "{}", "Sweden"),
            &Province::Tunis => write!(f, "{}", "Tunis"),
            &Province::Denmark => write!(f, "{}", "Denmark"),
            &Province::AdriaticSea => write!(f, "{}", "Adriatic Sea"),
            &Province::AegeanSea => write!(f, "{}", "Aegean Sea"),
            &Province::BalticSea => write!(f, "{}", "Baltic Sea"),
            &Province::BarentsSea => write!(f, "{}", "Barents Sea"),
            &Province::BlackSea => write!(f, "{}", "Black Sea"),
            &Province::EasternMediterranean => write!(f, "{}", "Eastern Mediterranean"),
            &Province::EnglishChannel => write!(f, "{}", "English Channel"),
            &Province::GulfOfBothnia => write!(f, "{}", "Gulf of Bothnia"),
            &Province::GulfOfLyon => write!(f, "{}", "Gulf of Lyon"),
            &Province::HeligolandBight => write!(f, "{}", "Heligoland Bight"),
            &Province::IonianSea => write!(f, "{}", "Ionian Sea"),
            &Province::IrishSea => write!(f, "{}", "Irish Sea"),
            &Province::MidAtlanticOcean => write!(f, "{}", "Mid-Atlantic Ocean"),
            &Province::NorthAtlanticOcean => write!(f, "{}", "North Atlantic Ocean"),
            &Province::NorthSea => write!(f, "{}", "North Sea"),
            &Province::NorwegianSea => write!(f, "{}", "Norwegian Sea"),
            &Province::Skagerrak => write!(f, "{}", "Skagerrak"),
            &Province::TyrrhenianSea => write!(f, "{}", "Tyrrhenian Sea"),
            &Province::WesternMediterranean => write!(f, "{}", "Western Mediterranean"),
        }
    }
}
impl Province {
    pub fn country(&self) -> Option<Country> {
        match &self {
            &Province::Bohemia => Some(Country::Austria),
            &Province::Budapest => Some(Country::Austria),
            &Province::Galicia => Some(Country::Austria),
            &Province::Trieste => Some(Country::Austria),
            &Province::Tyrolia => Some(Country::Austria),
            &Province::Vienna => Some(Country::Austria),
            &Province::Clyde => Some(Country::England),
            &Province::Edinburgh => Some(Country::England),
            &Province::Liverpool => Some(Country::England),
            &Province::London => Some(Country::England),
            &Province::Wales => Some(Country::England),
            &Province::Yorkshire => Some(Country::England),
            &Province::Brest => Some(Country::France),
            &Province::Burgundy => Some(Country::France),
            &Province::Gascony => Some(Country::France),
            &Province::Marseilles => Some(Country::France),
            &Province::Paris => Some(Country::France),
            &Province::Picardy => Some(Country::France),
            &Province::Berlin => Some(Country::Germany),
            &Province::Kiel => Some(Country::Germany),
            &Province::Munich => Some(Country::Germany),
            &Province::Prussia => Some(Country::Germany),
            &Province::Ruhr => Some(Country::Germany),
            &Province::Silesia => Some(Country::Germany),
            &Province::Apulia => Some(Country::Italy),
            &Province::Naples => Some(Country::Italy),
            &Province::Piedmont => Some(Country::Italy),
            &Province::Rome => Some(Country::Italy),
            &Province::Tuscany => Some(Country::Italy),
            &Province::Venice => Some(Country::Italy),
            &Province::Livonia => Some(Country::Russia),
            &Province::Moscow => Some(Country::Russia),
            &Province::Sevastopol => Some(Country::Russia),
            &Province::StPetersburg => Some(Country::Russia),
            &Province::Ukraine => Some(Country::Russia),
            &Province::Warsaw => Some(Country::Russia),
            &Province::Ankara => Some(Country::Turkey),
            &Province::Armenia => Some(Country::Turkey),
            &Province::Constantinople => Some(Country::Turkey),
            &Province::Smyrna => Some(Country::Turkey),
            &Province::Syria => Some(Country::Turkey),
            &Province::Albania => None,
            &Province::Belgium => None,
            &Province::Bulgaria => None,
            &Province::Finland => None,
            &Province::Greece => None,
            &Province::Holland => None,
            &Province::Norway => None,
            &Province::NorthAfrica => None,
            &Province::Portugal => None,
            &Province::Rumania => None,
            &Province::Serbia => None,
            &Province::Spain => None,
            &Province::Sweden => None,
            &Province::Tunis => None,
            &Province::Denmark => None,
            &Province::AdriaticSea => None,
            &Province::AegeanSea => None,
            &Province::BalticSea => None,
            &Province::BarentsSea => None,
            &Province::BlackSea => None,
            &Province::EasternMediterranean => None,
            &Province::EnglishChannel => None,
            &Province::GulfOfBothnia => None,
            &Province::GulfOfLyon => None,
            &Province::HeligolandBight => None,
            &Province::IonianSea => None,
            &Province::IrishSea => None,
            &Province::MidAtlanticOcean => None,
            &Province::NorthAtlanticOcean => None,
            &Province::NorthSea => None,
            &Province::NorwegianSea => None,
            &Province::Skagerrak => None,
            &Province::TyrrhenianSea => None,
            &Province::WesternMediterranean => None,
        }
    }
    pub fn neighbors(&self) -> &'static [Province] {
        match &self {
            &Province::Bohemia => {
                &[
                    Province::Munich,
                    Province::Tyrolia,
                    Province::Vienna,
                    Province::Silesia,
                    Province::Galicia,
                ]
            }
            &Province::Budapest => {
                &[
                    Province::Vienna,
                    Province::Galicia,
                    Province::Rumania,
                    Province::Serbia,
                    Province::Trieste,
                ]
            }
            &Province::Galicia => {
                &[
                    Province::Warsaw,
                    Province::Silesia,
                    Province::Ukraine,
                    Province::Rumania,
                    Province::Budapest,
                    Province::Vienna,
                    Province::Bohemia,
                ]
            }
            &Province::Trieste => {
                &[
                    Province::AdriaticSea,
                    Province::Venice,
                    Province::Tyrolia,
                    Province::Vienna,
                    Province::Budapest,
                    Province::Serbia,
                    Province::Albania,
                ]
            }
            &Province::Tyrolia => {
                &[
                    Province::Piedmont,
                    Province::Munich,
                    Province::Bohemia,
                    Province::Vienna,
                    Province::Trieste,
                    Province::Venice,
                ]
            }
            &Province::Vienna => {
                &[
                    Province::Trieste,
                    Province::Tyrolia,
                    Province::Bohemia,
                    Province::Galicia,
                    Province::Budapest,
                ]
            }
            &Province::Clyde => {
                &[
                    Province::NorthAtlanticOcean,
                    Province::NorwegianSea,
                    Province::Edinburgh,
                    Province::Liverpool,
                ]
            }
            &Province::Edinburgh => {
                &[
                    Province::Clyde,
                    Province::NorwegianSea,
                    Province::NorthSea,
                    Province::Yorkshire,
                    Province::Liverpool,
                ]
            }
            &Province::Liverpool => {
                &[
                    Province::NorthAtlanticOcean,
                    Province::IrishSea,
                    Province::Clyde,
                    Province::Edinburgh,
                    Province::Yorkshire,
                    Province::Wales,
                ]
            }
            &Province::London => {
                &[
                    Province::NorthSea,
                    Province::EnglishChannel,
                    Province::Wales,
                    Province::Yorkshire,
                ]
            }
            &Province::Wales => {
                &[
                    Province::IrishSea,
                    Province::EnglishChannel,
                    Province::London,
                    Province::Yorkshire,
                    Province::Liverpool,
                ]
            }
            &Province::Yorkshire => {
                &[
                    Province::Liverpool,
                    Province::Edinburgh,
                    Province::London,
                    Province::Wales,
                    Province::NorthSea,
                ]
            }
            &Province::Brest => {
                &[
                    Province::EnglishChannel,
                    Province::MidAtlanticOcean,
                    Province::Picardy,
                    Province::Paris,
                    Province::Gascony,
                ]
            }
            &Province::Burgundy => {
                &[
                    Province::Paris,
                    Province::Picardy,
                    Province::Belgium,
                    Province::Ruhr,
                    Province::Munich,
                    Province::Marseilles,
                    Province::Gascony,
                ]
            }
            &Province::Gascony => {
                &[
                    Province::MidAtlanticOcean,
                    Province::Spain,
                    Province::Brest,
                    Province::Paris,
                    Province::Burgundy,
                    Province::Marseilles,
                ]
            }
            &Province::Marseilles => {
                &[
                    Province::GulfOfLyon,
                    Province::Spain,
                    Province::Gascony,
                    Province::Burgundy,
                    Province::Piedmont,
                ]
            }
            &Province::Paris => {
                &[
                    Province::Brest,
                    Province::Picardy,
                    Province::Burgundy,
                    Province::Gascony,
                ]
            }
            &Province::Picardy => {
                &[
                    Province::EnglishChannel,
                    Province::Belgium,
                    Province::Burgundy,
                    Province::Paris,
                    Province::Brest,
                ]
            }
            &Province::Berlin => {
                &[
                    Province::BalticSea,
                    Province::Prussia,
                    Province::Silesia,
                    Province::Munich,
                    Province::Kiel,
                ]
            }
            &Province::Kiel => {
                &[
                    Province::HeligolandBight,
                    Province::Berlin,
                    Province::Munich,
                    Province::Ruhr,
                    Province::Holland,
                    Province::Denmark,
                    Province::BalticSea,
                ]
            }
            &Province::Munich => {
                &[
                    Province::Ruhr,
                    Province::Kiel,
                    Province::Berlin,
                    Province::Silesia,
                    Province::Bohemia,
                    Province::Tyrolia,
                    Province::Burgundy,
                ]
            }
            &Province::Prussia => {
                &[
                    Province::BalticSea,
                    Province::Livonia,
                    Province::Warsaw,
                    Province::Silesia,
                    Province::Berlin,
                ]
            }
            &Province::Ruhr => {
                &[
                    Province::Belgium,
                    Province::Holland,
                    Province::Kiel,
                    Province::Munich,
                    Province::Burgundy,
                ]
            }
            &Province::Silesia => {
                &[
                    Province::Munich,
                    Province::Berlin,
                    Province::Prussia,
                    Province::Warsaw,
                    Province::Galicia,
                    Province::Bohemia,
                ]
            }
            &Province::Apulia => {
                &[
                    Province::AdriaticSea,
                    Province::IonianSea,
                    Province::Naples,
                    Province::Rome,
                    Province::Venice,
                ]
            }
            &Province::Naples => {
                &[
                    Province::IonianSea,
                    Province::TyrrhenianSea,
                    Province::Apulia,
                    Province::Rome,
                ]
            }
            &Province::Piedmont => {
                &[
                    Province::Marseilles,
                    Province::Tyrolia,
                    Province::GulfOfLyon,
                    Province::Venice,
                    Province::Tuscany,
                ]
            }
            &Province::Rome => {
                &[
                    Province::TyrrhenianSea,
                    Province::Naples,
                    Province::Tuscany,
                    Province::Venice,
                    Province::Apulia,
                ]
            }
            &Province::Tuscany => {
                &[
                    Province::GulfOfLyon,
                    Province::Piedmont,
                    Province::Venice,
                    Province::Rome,
                    Province::TyrrhenianSea,
                ]
            }
            &Province::Venice => {
                &[
                    Province::Piedmont,
                    Province::Tyrolia,
                    Province::Trieste,
                    Province::AdriaticSea,
                    Province::Apulia,
                    Province::Tuscany,
                    Province::Rome,
                ]
            }
            &Province::Livonia => {
                &[
                    Province::BalticSea,
                    Province::GulfOfBothnia,
                    Province::StPetersburg,
                    Province::Moscow,
                    Province::Warsaw,
                    Province::Prussia,
                ]
            }
            &Province::Moscow => {
                &[
                    Province::StPetersburg,
                    Province::Sevastopol,
                    Province::Ukraine,
                    Province::Warsaw,
                    Province::Livonia,
                ]
            }
            &Province::Sevastopol => {
                &[
                    Province::Armenia,
                    Province::BlackSea,
                    Province::Rumania,
                    Province::Ukraine,
                    Province::Moscow,
                ]
            }
            &Province::StPetersburg => {
                &[
                    Province::BarentsSea,
                    Province::Moscow,
                    Province::Livonia,
                    Province::GulfOfBothnia,
                    Province::Finland,
                    Province::Norway,
                ]
            }
            &Province::Ukraine => {
                &[
                    Province::Moscow,
                    Province::Sevastopol,
                    Province::Rumania,
                    Province::Galicia,
                    Province::Warsaw,
                ]
            }
            &Province::Warsaw => {
                &[
                    Province::Prussia,
                    Province::Livonia,
                    Province::Moscow,
                    Province::Ukraine,
                    Province::Galicia,
                    Province::Silesia,
                ]
            }
            &Province::Ankara => {
                &[
                    Province::BlackSea,
                    Province::Armenia,
                    Province::Smyrna,
                    Province::Constantinople,
                ]
            }
            &Province::Armenia => {
                &[
                    Province::BlackSea,
                    Province::Sevastopol,
                    Province::Syria,
                    Province::Ankara,
                    Province::Smyrna,
                ]
            }
            &Province::Constantinople => {
                &[
                    Province::BlackSea,
                    Province::Ankara,
                    Province::Smyrna,
                    Province::Bulgaria,
                    Province::AegeanSea,
                ]
            }
            &Province::Smyrna => {
                &[
                    Province::EasternMediterranean,
                    Province::AegeanSea,
                    Province::Constantinople,
                    Province::Ankara,
                    Province::Armenia,
                    Province::Syria,
                ]
            }
            &Province::Syria => {
                &[Province::Armenia, Province::Smyrna, Province::EasternMediterranean]
            }
            &Province::Albania => {
                &[
                    Province::AdriaticSea,
                    Province::Trieste,
                    Province::Serbia,
                    Province::Greece,
                    Province::IonianSea,
                ]
            }
            &Province::Belgium => {
                &[
                    Province::Holland,
                    Province::Ruhr,
                    Province::Burgundy,
                    Province::Picardy,
                    Province::EnglishChannel,
                    Province::NorthSea,
                ]
            }
            &Province::Bulgaria => {
                &[
                    Province::Rumania,
                    Province::BlackSea,
                    Province::Constantinople,
                    Province::AegeanSea,
                    Province::Greece,
                    Province::Serbia,
                ]
            }
            &Province::Finland => {
                &[
                    Province::StPetersburg,
                    Province::Sweden,
                    Province::Norway,
                    Province::GulfOfBothnia,
                ]
            }
            &Province::Greece => {
                &[
                    Province::IonianSea,
                    Province::AegeanSea,
                    Province::Albania,
                    Province::Serbia,
                    Province::Bulgaria,
                ]
            }
            &Province::Holland => {
                &[
                    Province::Belgium,
                    Province::NorthSea,
                    Province::Kiel,
                    Province::Ruhr,
                    Province::HeligolandBight,
                ]
            }
            &Province::Norway => {
                &[
                    Province::NorwegianSea,
                    Province::NorthSea,
                    Province::Sweden,
                    Province::Finland,
                    Province::Skagerrak,
                    Province::BarentsSea,
                    Province::StPetersburg,
                ]
            }
            &Province::NorthAfrica => {
                &[
                    Province::MidAtlanticOcean,
                    Province::WesternMediterranean,
                    Province::Tunis,
                ]
            }
            &Province::Portugal => &[Province::MidAtlanticOcean, Province::Spain],
            &Province::Rumania => {
                &[
                    Province::BlackSea,
                    Province::Bulgaria,
                    Province::Serbia,
                    Province::Budapest,
                    Province::Galicia,
                    Province::Ukraine,
                    Province::Sevastopol,
                ]
            }
            &Province::Serbia => {
                &[
                    Province::Trieste,
                    Province::Budapest,
                    Province::Rumania,
                    Province::Bulgaria,
                    Province::Greece,
                    Province::Albania,
                ]
            }
            &Province::Spain => {
                &[
                    Province::Portugal,
                    Province::MidAtlanticOcean,
                    Province::Gascony,
                    Province::GulfOfLyon,
                    Province::WesternMediterranean,
                    Province::Marseilles,
                ]
            }
            &Province::Sweden => {
                &[
                    Province::GulfOfBothnia,
                    Province::Finland,
                    Province::Norway,
                    Province::BalticSea,
                    Province::Skagerrak,
                    Province::Denmark,
                ]
            }
            &Province::Tunis => {
                &[
                    Province::NorthAfrica,
                    Province::WesternMediterranean,
                    Province::IonianSea,
                    Province::TyrrhenianSea,
                ]
            }
            &Province::Denmark => {
                &[
                    Province::BalticSea,
                    Province::Skagerrak,
                    Province::HeligolandBight,
                    Province::Kiel,
                    Province::NorthSea,
                    Province::Sweden,
                ]
            }
            &Province::AdriaticSea => {
                &[
                    Province::Trieste,
                    Province::Venice,
                    Province::Apulia,
                    Province::Albania,
                    Province::IonianSea,
                ]
            }
            &Province::AegeanSea => {
                &[
                    Province::Greece,
                    Province::Bulgaria,
                    Province::Constantinople,
                    Province::Smyrna,
                    Province::EasternMediterranean,
                    Province::IonianSea,
                ]
            }
            &Province::BalticSea => {
                &[
                    Province::Sweden,
                    Province::GulfOfBothnia,
                    Province::Livonia,
                    Province::Prussia,
                    Province::Berlin,
                    Province::Kiel,
                    Province::Denmark,
                ]
            }
            &Province::BarentsSea => {
                &[Province::StPetersburg, Province::Norway, Province::NorwegianSea]
            }
            &Province::BlackSea => {
                &[
                    Province::Sevastopol,
                    Province::Armenia,
                    Province::Ankara,
                    Province::Constantinople,
                    Province::Bulgaria,
                    Province::Rumania,
                ]
            }
            &Province::EasternMediterranean => {
                &[
                    Province::Syria,
                    Province::IonianSea,
                    Province::AegeanSea,
                    Province::Smyrna,
                ]
            }
            &Province::EnglishChannel => {
                &[
                    Province::London,
                    Province::Belgium,
                    Province::Picardy,
                    Province::Brest,
                    Province::MidAtlanticOcean,
                    Province::IrishSea,
                    Province::Wales,
                    Province::NorthSea,
                ]
            }
            &Province::GulfOfBothnia => {
                &[
                    Province::Sweden,
                    Province::Finland,
                    Province::Livonia,
                    Province::StPetersburg,
                    Province::BalticSea,
                ]
            }
            &Province::GulfOfLyon => {
                &[
                    Province::Marseilles,
                    Province::Piedmont,
                    Province::Tuscany,
                    Province::TyrrhenianSea,
                    Province::WesternMediterranean,
                    Province::Spain,
                ]
            }
            &Province::HeligolandBight => {
                &[
                    Province::Denmark,
                    Province::Kiel,
                    Province::Holland,
                    Province::NorthSea,
                ]
            }
            &Province::IonianSea => {
                &[
                    Province::Tunis,
                    Province::TyrrhenianSea,
                    Province::Naples,
                    Province::Apulia,
                    Province::AdriaticSea,
                    Province::Greece,
                    Province::Albania,
                    Province::AegeanSea,
                    Province::EasternMediterranean,
                ]
            }
            &Province::IrishSea => {
                &[
                    Province::NorthAtlanticOcean,
                    Province::EnglishChannel,
                    Province::MidAtlanticOcean,
                    Province::Liverpool,
                    Province::Wales,
                ]
            }
            &Province::MidAtlanticOcean => {
                &[
                    Province::NorthAtlanticOcean,
                    Province::IrishSea,
                    Province::EnglishChannel,
                    Province::Brest,
                    Province::Gascony,
                    Province::Spain,
                    Province::Portugal,
                    Province::WesternMediterranean,
                    Province::NorthAfrica,
                ]
            }
            &Province::NorthAtlanticOcean => {
                &[
                    Province::NorwegianSea,
                    Province::Clyde,
                    Province::Liverpool,
                    Province::IrishSea,
                    Province::MidAtlanticOcean,
                ]
            }
            &Province::NorthSea => {
                &[
                    Province::NorwegianSea,
                    Province::Skagerrak,
                    Province::Denmark,
                    Province::HeligolandBight,
                    Province::Holland,
                    Province::Belgium,
                    Province::EnglishChannel,
                    Province::London,
                    Province::Yorkshire,
                    Province::Edinburgh,
                    Province::Norway,
                ]
            }
            &Province::NorwegianSea => {
                &[
                    Province::NorthAtlanticOcean,
                    Province::Norway,
                    Province::BarentsSea,
                    Province::NorthSea,
                    Province::Clyde,
                    Province::Edinburgh,
                ]
            }
            &Province::Skagerrak => {
                &[
                    Province::Norway,
                    Province::Sweden,
                    Province::Denmark,
                    Province::NorthSea,
                ]
            }
            &Province::TyrrhenianSea => {
                &[
                    Province::GulfOfLyon,
                    Province::WesternMediterranean,
                    Province::Tunis,
                    Province::Tuscany,
                    Province::Rome,
                    Province::Naples,
                    Province::IonianSea,
                ]
            }
            &Province::WesternMediterranean => {
                &[
                    Province::NorthAfrica,
                    Province::MidAtlanticOcean,
                    Province::GulfOfLyon,
                    Province::Spain,
                    Province::Tunis,
                    Province::TyrrhenianSea,
                ]
            }
        }
    }
    pub fn has_supply_center(&self) -> bool {
        match &self {
            &Province::Bohemia => false,
            &Province::Budapest => true,
            &Province::Galicia => false,
            &Province::Trieste => true,
            &Province::Tyrolia => false,
            &Province::Vienna => true,
            &Province::Clyde => false,
            &Province::Edinburgh => true,
            &Province::Liverpool => true,
            &Province::London => true,
            &Province::Wales => false,
            &Province::Yorkshire => false,
            &Province::Brest => true,
            &Province::Burgundy => false,
            &Province::Gascony => false,
            &Province::Marseilles => true,
            &Province::Paris => true,
            &Province::Picardy => false,
            &Province::Berlin => true,
            &Province::Kiel => true,
            &Province::Munich => true,
            &Province::Prussia => false,
            &Province::Ruhr => false,
            &Province::Silesia => false,
            &Province::Apulia => false,
            &Province::Naples => true,
            &Province::Piedmont => false,
            &Province::Rome => true,
            &Province::Tuscany => false,
            &Province::Venice => true,
            &Province::Livonia => false,
            &Province::Moscow => true,
            &Province::Sevastopol => true,
            &Province::StPetersburg => true,
            &Province::Ukraine => false,
            &Province::Warsaw => true,
            &Province::Ankara => true,
            &Province::Armenia => false,
            &Province::Constantinople => true,
            &Province::Smyrna => true,
            &Province::Syria => false,
            &Province::Albania => false,
            &Province::Belgium => true,
            &Province::Bulgaria => true,
            &Province::Finland => false,
            &Province::Greece => true,
            &Province::Holland => true,
            &Province::Norway => true,
            &Province::NorthAfrica => false,
            &Province::Portugal => true,
            &Province::Rumania => true,
            &Province::Serbia => true,
            &Province::Spain => true,
            &Province::Sweden => true,
            &Province::Tunis => true,
            &Province::Denmark => true,
            &Province::AdriaticSea => false,
            &Province::AegeanSea => false,
            &Province::BalticSea => false,
            &Province::BarentsSea => false,
            &Province::BlackSea => false,
            &Province::EasternMediterranean => false,
            &Province::EnglishChannel => false,
            &Province::GulfOfBothnia => false,
            &Province::GulfOfLyon => false,
            &Province::HeligolandBight => false,
            &Province::IonianSea => false,
            &Province::IrishSea => false,
            &Province::MidAtlanticOcean => false,
            &Province::NorthAtlanticOcean => false,
            &Province::NorthSea => false,
            &Province::NorwegianSea => false,
            &Province::Skagerrak => false,
            &Province::TyrrhenianSea => false,
            &Province::WesternMediterranean => false,
        }
    }
    pub fn terrain(&self) -> Terrain {
        match &self {
            &Province::Bohemia => Terrain::Inland,
            &Province::Budapest => Terrain::Inland,
            &Province::Galicia => Terrain::Inland,
            &Province::Trieste => Terrain::Coastal,
            &Province::Tyrolia => Terrain::Inland,
            &Province::Vienna => Terrain::Inland,
            &Province::Clyde => Terrain::Coastal,
            &Province::Edinburgh => Terrain::Coastal,
            &Province::Liverpool => Terrain::Coastal,
            &Province::London => Terrain::Coastal,
            &Province::Wales => Terrain::Coastal,
            &Province::Yorkshire => Terrain::Coastal,
            &Province::Brest => Terrain::Coastal,
            &Province::Burgundy => Terrain::Inland,
            &Province::Gascony => Terrain::Coastal,
            &Province::Marseilles => Terrain::Coastal,
            &Province::Paris => Terrain::Inland,
            &Province::Picardy => Terrain::Coastal,
            &Province::Berlin => Terrain::Coastal,
            &Province::Kiel => Terrain::Coastal,
            &Province::Munich => Terrain::Inland,
            &Province::Prussia => Terrain::Coastal,
            &Province::Ruhr => Terrain::Inland,
            &Province::Silesia => Terrain::Inland,
            &Province::Apulia => Terrain::Coastal,
            &Province::Naples => Terrain::Coastal,
            &Province::Piedmont => Terrain::Coastal,
            &Province::Rome => Terrain::Coastal,
            &Province::Tuscany => Terrain::Coastal,
            &Province::Venice => Terrain::Coastal,
            &Province::Livonia => Terrain::Coastal,
            &Province::Moscow => Terrain::Inland,
            &Province::Sevastopol => Terrain::Coastal,
            &Province::StPetersburg => Terrain::Coastal,
            &Province::Ukraine => Terrain::Inland,
            &Province::Warsaw => Terrain::Inland,
            &Province::Ankara => Terrain::Coastal,
            &Province::Armenia => Terrain::Coastal,
            &Province::Constantinople => Terrain::Coastal,
            &Province::Smyrna => Terrain::Coastal,
            &Province::Syria => Terrain::Coastal,
            &Province::Albania => Terrain::Coastal,
            &Province::Belgium => Terrain::Coastal,
            &Province::Bulgaria => Terrain::Coastal,
            &Province::Finland => Terrain::Coastal,
            &Province::Greece => Terrain::Coastal,
            &Province::Holland => Terrain::Coastal,
            &Province::Norway => Terrain::Coastal,
            &Province::NorthAfrica => Terrain::Coastal,
            &Province::Portugal => Terrain::Coastal,
            &Province::Rumania => Terrain::Coastal,
            &Province::Serbia => Terrain::Inland,
            &Province::Spain => Terrain::Coastal,
            &Province::Sweden => Terrain::Coastal,
            &Province::Tunis => Terrain::Coastal,
            &Province::Denmark => Terrain::Coastal,
            &Province::AdriaticSea => Terrain::Water,
            &Province::AegeanSea => Terrain::Water,
            &Province::BalticSea => Terrain::Water,
            &Province::BarentsSea => Terrain::Water,
            &Province::BlackSea => Terrain::Water,
            &Province::EasternMediterranean => Terrain::Water,
            &Province::EnglishChannel => Terrain::Water,
            &Province::GulfOfBothnia => Terrain::Water,
            &Province::GulfOfLyon => Terrain::Water,
            &Province::HeligolandBight => Terrain::Water,
            &Province::IonianSea => Terrain::Water,
            &Province::IrishSea => Terrain::Water,
            &Province::MidAtlanticOcean => Terrain::Water,
            &Province::NorthAtlanticOcean => Terrain::Water,
            &Province::NorthSea => Terrain::Water,
            &Province::NorwegianSea => Terrain::Water,
            &Province::Skagerrak => Terrain::Water,
            &Province::TyrrhenianSea => Terrain::Water,
            &Province::WesternMediterranean => Terrain::Water,
        }
    }
}

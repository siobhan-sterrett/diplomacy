#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ArmySpace {
    Albania,
    Ankara,
    Apulia,
    Armenia,
    Belgium,
    Berlin,
    Bohemia,
    Brest,
    Budapest,
    Bulgaria,
    Burgundy,
    Clyde,
    Constantinople,
    Denmark,
    Edinburgh,
    Finland,
    Galicia,
    Gascony,
    Greece,
    Holland,
    Kiel,
    Liverpool,
    Livonia,
    London,
    Marseilles,
    Moscow,
    Munich,
    Naples,
    NorthAfrica,
    Norway,
    Paris,
    Picardy,
    Piedmont,
    Portugal,
    Prussia,
    Rome,
    Ruhr,
    Rumania,
    Serbia,
    Sevastopol,
    Silesia,
    Smyrna,
    Spain,
    StPetersburg,
    Sweden,
    Syria,
    Trieste,
    Tunis,
    Tuscany,
    Tyrolia,
    Ukraine,
    Venice,
    Vienna,
    Wales,
    Warsaw,
    Yorkshire,
}

impl core::fmt::Display for ArmySpace {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match &self {
            &ArmySpace::Albania => write!(f, "{}", "Albania"),
            &ArmySpace::Ankara => write!(f, "{}", "Ankara"),
            &ArmySpace::Apulia => write!(f, "{}", "Apulia"),
            &ArmySpace::Armenia => write!(f, "{}", "Armenia"),
            &ArmySpace::Belgium => write!(f, "{}", "Belgium"),
            &ArmySpace::Berlin => write!(f, "{}", "Berlin"),
            &ArmySpace::Bohemia => write!(f, "{}", "Bohemia"),
            &ArmySpace::Brest => write!(f, "{}", "Brest"),
            &ArmySpace::Budapest => write!(f, "{}", "Budapest"),
            &ArmySpace::Bulgaria => write!(f, "{}", "Bulgaria"),
            &ArmySpace::Burgundy => write!(f, "{}", "Burgundy"),
            &ArmySpace::Clyde => write!(f, "{}", "Clyde"),
            &ArmySpace::Constantinople => write!(f, "{}", "Constantinople"),
            &ArmySpace::Denmark => write!(f, "{}", "Denmark"),
            &ArmySpace::Edinburgh => write!(f, "{}", "Edinburgh"),
            &ArmySpace::Finland => write!(f, "{}", "Finland"),
            &ArmySpace::Galicia => write!(f, "{}", "Galicia"),
            &ArmySpace::Gascony => write!(f, "{}", "Gascony"),
            &ArmySpace::Greece => write!(f, "{}", "Greece"),
            &ArmySpace::Holland => write!(f, "{}", "Holland"),
            &ArmySpace::Kiel => write!(f, "{}", "Kiel"),
            &ArmySpace::Liverpool => write!(f, "{}", "Liverpool"),
            &ArmySpace::Livonia => write!(f, "{}", "Livonia"),
            &ArmySpace::London => write!(f, "{}", "London"),
            &ArmySpace::Marseilles => write!(f, "{}", "Marseilles"),
            &ArmySpace::Moscow => write!(f, "{}", "Moscow"),
            &ArmySpace::Munich => write!(f, "{}", "Munich"),
            &ArmySpace::Naples => write!(f, "{}", "Naples"),
            &ArmySpace::NorthAfrica => write!(f, "{}", "North Africa"),
            &ArmySpace::Norway => write!(f, "{}", "Norway"),
            &ArmySpace::Paris => write!(f, "{}", "Paris"),
            &ArmySpace::Picardy => write!(f, "{}", "Picardy"),
            &ArmySpace::Piedmont => write!(f, "{}", "Piedmont"),
            &ArmySpace::Portugal => write!(f, "{}", "Portugal"),
            &ArmySpace::Prussia => write!(f, "{}", "Prussia"),
            &ArmySpace::Rome => write!(f, "{}", "Rome"),
            &ArmySpace::Ruhr => write!(f, "{}", "Ruhr"),
            &ArmySpace::Rumania => write!(f, "{}", "Rumania"),
            &ArmySpace::Serbia => write!(f, "{}", "Serbia"),
            &ArmySpace::Sevastopol => write!(f, "{}", "Sevastopol"),
            &ArmySpace::Silesia => write!(f, "{}", "Silesia"),
            &ArmySpace::Smyrna => write!(f, "{}", "Smyrna"),
            &ArmySpace::Spain => write!(f, "{}", "Spain"),
            &ArmySpace::StPetersburg => write!(f, "{}", "St Petersburg"),
            &ArmySpace::Sweden => write!(f, "{}", "Sweden"),
            &ArmySpace::Syria => write!(f, "{}", "Syria"),
            &ArmySpace::Trieste => write!(f, "{}", "Trieste"),
            &ArmySpace::Tunis => write!(f, "{}", "Tunis"),
            &ArmySpace::Tuscany => write!(f, "{}", "Tuscany"),
            &ArmySpace::Tyrolia => write!(f, "{}", "Tyrolia"),
            &ArmySpace::Ukraine => write!(f, "{}", "Ukraine"),
            &ArmySpace::Venice => write!(f, "{}", "Venice"),
            &ArmySpace::Vienna => write!(f, "{}", "Vienna"),
            &ArmySpace::Wales => write!(f, "{}", "Wales"),
            &ArmySpace::Warsaw => write!(f, "{}", "Warsaw"),
            &ArmySpace::Yorkshire => write!(f, "{}", "Yorkshire"),
        }
    }
}

impl ArmySpace {
    pub fn province(&self) -> super::province::Province {
        match &self {
            ArmySpace::Albania => super::province::Province::Albania,
            ArmySpace::Ankara => super::province::Province::Ankara,
            ArmySpace::Apulia => super::province::Province::Apulia,
            ArmySpace::Armenia => super::province::Province::Armenia,
            ArmySpace::Belgium => super::province::Province::Belgium,
            ArmySpace::Berlin => super::province::Province::Berlin,
            ArmySpace::Bohemia => super::province::Province::Bohemia,
            ArmySpace::Brest => super::province::Province::Brest,
            ArmySpace::Budapest => super::province::Province::Budapest,
            ArmySpace::Bulgaria => super::province::Province::Bulgaria,
            ArmySpace::Burgundy => super::province::Province::Burgundy,
            ArmySpace::Clyde => super::province::Province::Clyde,
            ArmySpace::Constantinople => super::province::Province::Constantinople,
            ArmySpace::Denmark => super::province::Province::Denmark,
            ArmySpace::Edinburgh => super::province::Province::Edinburgh,
            ArmySpace::Finland => super::province::Province::Finland,
            ArmySpace::Galicia => super::province::Province::Galicia,
            ArmySpace::Gascony => super::province::Province::Gascony,
            ArmySpace::Greece => super::province::Province::Greece,
            ArmySpace::Holland => super::province::Province::Holland,
            ArmySpace::Kiel => super::province::Province::Kiel,
            ArmySpace::Liverpool => super::province::Province::Liverpool,
            ArmySpace::Livonia => super::province::Province::Livonia,
            ArmySpace::London => super::province::Province::London,
            ArmySpace::Marseilles => super::province::Province::Marseilles,
            ArmySpace::Moscow => super::province::Province::Moscow,
            ArmySpace::Munich => super::province::Province::Munich,
            ArmySpace::Naples => super::province::Province::Naples,
            ArmySpace::NorthAfrica => super::province::Province::NorthAfrica,
            ArmySpace::Norway => super::province::Province::Norway,
            ArmySpace::Paris => super::province::Province::Paris,
            ArmySpace::Picardy => super::province::Province::Picardy,
            ArmySpace::Piedmont => super::province::Province::Piedmont,
            ArmySpace::Portugal => super::province::Province::Portugal,
            ArmySpace::Prussia => super::province::Province::Prussia,
            ArmySpace::Rome => super::province::Province::Rome,
            ArmySpace::Ruhr => super::province::Province::Ruhr,
            ArmySpace::Rumania => super::province::Province::Rumania,
            ArmySpace::Serbia => super::province::Province::Serbia,
            ArmySpace::Sevastopol => super::province::Province::Sevastopol,
            ArmySpace::Silesia => super::province::Province::Silesia,
            ArmySpace::Smyrna => super::province::Province::Smyrna,
            ArmySpace::Spain => super::province::Province::Spain,
            ArmySpace::StPetersburg => super::province::Province::StPetersburg,
            ArmySpace::Sweden => super::province::Province::Sweden,
            ArmySpace::Syria => super::province::Province::Syria,
            ArmySpace::Trieste => super::province::Province::Trieste,
            ArmySpace::Tunis => super::province::Province::Tunis,
            ArmySpace::Tuscany => super::province::Province::Tuscany,
            ArmySpace::Tyrolia => super::province::Province::Tyrolia,
            ArmySpace::Ukraine => super::province::Province::Ukraine,
            ArmySpace::Venice => super::province::Province::Venice,
            ArmySpace::Vienna => super::province::Province::Vienna,
            ArmySpace::Wales => super::province::Province::Wales,
            ArmySpace::Warsaw => super::province::Province::Warsaw,
            ArmySpace::Yorkshire => super::province::Province::Yorkshire,
        }
    }
    pub fn neighbors(&self) -> &'static [ArmySpace] {
        match (&self) {
            ArmySpace::Albania => {
                &[ArmySpace::Trieste, ArmySpace::Serbia, ArmySpace::Greece]
            }
            ArmySpace::Ankara => {
                &[ArmySpace::Armenia, ArmySpace::Smyrna, ArmySpace::Constantinople]
            }
            ArmySpace::Apulia => &[ArmySpace::Naples, ArmySpace::Rome, ArmySpace::Venice],
            ArmySpace::Armenia => {
                &[
                    ArmySpace::Sevastopol,
                    ArmySpace::Syria,
                    ArmySpace::Ankara,
                    ArmySpace::Smyrna,
                ]
            }
            ArmySpace::Belgium => {
                &[
                    ArmySpace::Holland,
                    ArmySpace::Ruhr,
                    ArmySpace::Burgundy,
                    ArmySpace::Picardy,
                ]
            }
            ArmySpace::Berlin => {
                &[
                    ArmySpace::Prussia,
                    ArmySpace::Silesia,
                    ArmySpace::Munich,
                    ArmySpace::Kiel,
                ]
            }
            ArmySpace::Bohemia => {
                &[
                    ArmySpace::Munich,
                    ArmySpace::Tyrolia,
                    ArmySpace::Vienna,
                    ArmySpace::Silesia,
                    ArmySpace::Galicia,
                ]
            }
            ArmySpace::Brest => {
                &[ArmySpace::Picardy, ArmySpace::Paris, ArmySpace::Gascony]
            }
            ArmySpace::Budapest => {
                &[
                    ArmySpace::Vienna,
                    ArmySpace::Galicia,
                    ArmySpace::Rumania,
                    ArmySpace::Serbia,
                    ArmySpace::Trieste,
                ]
            }
            ArmySpace::Bulgaria => {
                &[
                    ArmySpace::Rumania,
                    ArmySpace::Constantinople,
                    ArmySpace::Greece,
                    ArmySpace::Serbia,
                ]
            }
            ArmySpace::Burgundy => {
                &[
                    ArmySpace::Paris,
                    ArmySpace::Picardy,
                    ArmySpace::Belgium,
                    ArmySpace::Ruhr,
                    ArmySpace::Munich,
                    ArmySpace::Marseilles,
                    ArmySpace::Gascony,
                ]
            }
            ArmySpace::Clyde => &[ArmySpace::Edinburgh, ArmySpace::Liverpool],
            ArmySpace::Constantinople => {
                &[ArmySpace::Ankara, ArmySpace::Smyrna, ArmySpace::Bulgaria]
            }
            ArmySpace::Denmark => &[ArmySpace::Kiel, ArmySpace::Sweden],
            ArmySpace::Edinburgh => {
                &[ArmySpace::Clyde, ArmySpace::Yorkshire, ArmySpace::Liverpool]
            }
            ArmySpace::Finland => {
                &[ArmySpace::StPetersburg, ArmySpace::Sweden, ArmySpace::Norway]
            }
            ArmySpace::Galicia => {
                &[
                    ArmySpace::Warsaw,
                    ArmySpace::Silesia,
                    ArmySpace::Ukraine,
                    ArmySpace::Rumania,
                    ArmySpace::Budapest,
                    ArmySpace::Vienna,
                    ArmySpace::Bohemia,
                ]
            }
            ArmySpace::Gascony => {
                &[
                    ArmySpace::Spain,
                    ArmySpace::Brest,
                    ArmySpace::Paris,
                    ArmySpace::Burgundy,
                    ArmySpace::Marseilles,
                ]
            }
            ArmySpace::Greece => {
                &[ArmySpace::Albania, ArmySpace::Serbia, ArmySpace::Bulgaria]
            }
            ArmySpace::Holland => &[ArmySpace::Belgium, ArmySpace::Kiel, ArmySpace::Ruhr],
            ArmySpace::Kiel => {
                &[
                    ArmySpace::Berlin,
                    ArmySpace::Munich,
                    ArmySpace::Ruhr,
                    ArmySpace::Holland,
                    ArmySpace::Denmark,
                ]
            }
            ArmySpace::Liverpool => {
                &[
                    ArmySpace::Clyde,
                    ArmySpace::Edinburgh,
                    ArmySpace::Yorkshire,
                    ArmySpace::Wales,
                ]
            }
            ArmySpace::Livonia => {
                &[
                    ArmySpace::StPetersburg,
                    ArmySpace::Moscow,
                    ArmySpace::Warsaw,
                    ArmySpace::Prussia,
                ]
            }
            ArmySpace::London => &[ArmySpace::Wales, ArmySpace::Yorkshire],
            ArmySpace::Marseilles => {
                &[
                    ArmySpace::Spain,
                    ArmySpace::Gascony,
                    ArmySpace::Burgundy,
                    ArmySpace::Piedmont,
                ]
            }
            ArmySpace::Moscow => {
                &[
                    ArmySpace::StPetersburg,
                    ArmySpace::Sevastopol,
                    ArmySpace::Ukraine,
                    ArmySpace::Warsaw,
                    ArmySpace::Livonia,
                ]
            }
            ArmySpace::Munich => {
                &[
                    ArmySpace::Ruhr,
                    ArmySpace::Kiel,
                    ArmySpace::Berlin,
                    ArmySpace::Silesia,
                    ArmySpace::Bohemia,
                    ArmySpace::Tyrolia,
                    ArmySpace::Burgundy,
                ]
            }
            ArmySpace::Naples => &[ArmySpace::Apulia, ArmySpace::Rome],
            ArmySpace::NorthAfrica => &[ArmySpace::Tunis],
            ArmySpace::Norway => {
                &[ArmySpace::Sweden, ArmySpace::Finland, ArmySpace::StPetersburg]
            }
            ArmySpace::Paris => {
                &[
                    ArmySpace::Brest,
                    ArmySpace::Picardy,
                    ArmySpace::Burgundy,
                    ArmySpace::Gascony,
                ]
            }
            ArmySpace::Picardy => {
                &[
                    ArmySpace::Belgium,
                    ArmySpace::Burgundy,
                    ArmySpace::Paris,
                    ArmySpace::Brest,
                ]
            }
            ArmySpace::Piedmont => {
                &[
                    ArmySpace::Marseilles,
                    ArmySpace::Tyrolia,
                    ArmySpace::Venice,
                    ArmySpace::Tuscany,
                ]
            }
            ArmySpace::Portugal => &[ArmySpace::Spain],
            ArmySpace::Prussia => {
                &[
                    ArmySpace::Livonia,
                    ArmySpace::Warsaw,
                    ArmySpace::Silesia,
                    ArmySpace::Berlin,
                ]
            }
            ArmySpace::Rome => {
                &[
                    ArmySpace::Naples,
                    ArmySpace::Tuscany,
                    ArmySpace::Venice,
                    ArmySpace::Apulia,
                ]
            }
            ArmySpace::Ruhr => {
                &[
                    ArmySpace::Belgium,
                    ArmySpace::Holland,
                    ArmySpace::Kiel,
                    ArmySpace::Munich,
                    ArmySpace::Burgundy,
                ]
            }
            ArmySpace::Rumania => {
                &[
                    ArmySpace::Bulgaria,
                    ArmySpace::Serbia,
                    ArmySpace::Budapest,
                    ArmySpace::Galicia,
                    ArmySpace::Ukraine,
                    ArmySpace::Sevastopol,
                ]
            }
            ArmySpace::Serbia => {
                &[
                    ArmySpace::Trieste,
                    ArmySpace::Budapest,
                    ArmySpace::Rumania,
                    ArmySpace::Bulgaria,
                    ArmySpace::Greece,
                    ArmySpace::Albania,
                ]
            }
            ArmySpace::Sevastopol => {
                &[
                    ArmySpace::Armenia,
                    ArmySpace::Rumania,
                    ArmySpace::Ukraine,
                    ArmySpace::Moscow,
                ]
            }
            ArmySpace::Silesia => {
                &[
                    ArmySpace::Munich,
                    ArmySpace::Berlin,
                    ArmySpace::Prussia,
                    ArmySpace::Warsaw,
                    ArmySpace::Galicia,
                    ArmySpace::Bohemia,
                ]
            }
            ArmySpace::Smyrna => {
                &[
                    ArmySpace::Constantinople,
                    ArmySpace::Ankara,
                    ArmySpace::Armenia,
                    ArmySpace::Syria,
                ]
            }
            ArmySpace::Spain => {
                &[ArmySpace::Portugal, ArmySpace::Gascony, ArmySpace::Marseilles]
            }
            ArmySpace::StPetersburg => {
                &[
                    ArmySpace::Moscow,
                    ArmySpace::Livonia,
                    ArmySpace::Finland,
                    ArmySpace::Norway,
                ]
            }
            ArmySpace::Sweden => {
                &[ArmySpace::Finland, ArmySpace::Norway, ArmySpace::Denmark]
            }
            ArmySpace::Syria => &[ArmySpace::Armenia, ArmySpace::Smyrna],
            ArmySpace::Trieste => {
                &[
                    ArmySpace::Venice,
                    ArmySpace::Tyrolia,
                    ArmySpace::Vienna,
                    ArmySpace::Budapest,
                    ArmySpace::Serbia,
                    ArmySpace::Albania,
                ]
            }
            ArmySpace::Tunis => &[ArmySpace::NorthAfrica],
            ArmySpace::Tuscany => {
                &[ArmySpace::Piedmont, ArmySpace::Venice, ArmySpace::Rome]
            }
            ArmySpace::Tyrolia => {
                &[
                    ArmySpace::Piedmont,
                    ArmySpace::Munich,
                    ArmySpace::Bohemia,
                    ArmySpace::Vienna,
                    ArmySpace::Trieste,
                    ArmySpace::Venice,
                ]
            }
            ArmySpace::Ukraine => {
                &[
                    ArmySpace::Moscow,
                    ArmySpace::Sevastopol,
                    ArmySpace::Rumania,
                    ArmySpace::Galicia,
                    ArmySpace::Warsaw,
                ]
            }
            ArmySpace::Venice => {
                &[
                    ArmySpace::Piedmont,
                    ArmySpace::Tyrolia,
                    ArmySpace::Trieste,
                    ArmySpace::Apulia,
                    ArmySpace::Tuscany,
                    ArmySpace::Rome,
                ]
            }
            ArmySpace::Vienna => {
                &[
                    ArmySpace::Trieste,
                    ArmySpace::Tyrolia,
                    ArmySpace::Bohemia,
                    ArmySpace::Galicia,
                    ArmySpace::Budapest,
                ]
            }
            ArmySpace::Wales => {
                &[ArmySpace::London, ArmySpace::Yorkshire, ArmySpace::Liverpool]
            }
            ArmySpace::Warsaw => {
                &[
                    ArmySpace::Prussia,
                    ArmySpace::Livonia,
                    ArmySpace::Moscow,
                    ArmySpace::Ukraine,
                    ArmySpace::Galicia,
                    ArmySpace::Silesia,
                ]
            }
            ArmySpace::Yorkshire => {
                &[
                    ArmySpace::Liverpool,
                    ArmySpace::Edinburgh,
                    ArmySpace::London,
                    ArmySpace::Wales,
                ]
            }
        }
    }
}

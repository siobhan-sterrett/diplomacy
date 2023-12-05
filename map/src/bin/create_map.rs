use std::{error::Error, fs, path};

use askama::Template;
use clap::Parser;
use map::{
    country::Country,
    map_state::{MapState, MapStatePerCountry},
    space::Space,
    supply_center::SupplyCenter,
};

#[derive(Template)]
#[template(path = "map.xml")]
struct MapTemplate {
    units: Vec<UnitSpec>,
    supply_centers: Vec<SupplyCenterSpec>,
}

impl MapTemplate {
    fn load(map_state: &MapState) -> MapTemplate {
        let mut template = MapTemplate {
            units: Vec::new(),
            supply_centers: Vec::new(),
        };

        for (
            country,
            MapStatePerCountry {
                armies,
                fleets,
                supply_centers,
            },
        ) in map_state.iter()
        {
            for army in armies {
                template.units.push(UnitSpec {
                    country: *country,
                    space: Space::Army(*army),
                });
            }

            for fleet in fleets {
                template.units.push(UnitSpec {
                    country: *country,
                    space: Space::Fleet(*fleet),
                });
            }

            for supply_center in supply_centers {
                template.supply_centers.push(SupplyCenterSpec {
                    country: *country,
                    supply_center: *supply_center,
                })
            }
        }

        template
    }
}

struct UnitSpec {
    country: Country,
    space: Space,
}

struct SupplyCenterSpec {
    country: Country,
    supply_center: SupplyCenter,
}

#[derive(Parser)]
struct Cli {
    state_file: path::PathBuf,
    #[arg(short, long)]
    output: Option<path::PathBuf>,
}

impl Cli {
    fn load_map_state(&self) -> Result<MapState, Box<dyn Error>> {
        let f = fs::File::open(&self.state_file)?;
        let map_state = serde_yaml::from_reader(f)?;
        Ok(map_state)
    }

    fn write_map(&self, template: &MapTemplate) -> Result<(), Box<dyn Error>> {
        let mut f = if let Some(output_path) = &self.output {
            fs::File::create(output_path)?
        } else {
            fs::File::create("map.svg")?
        };
        template.write_into(&mut f)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let map_state = args.load_map_state()?;

    let template = MapTemplate::load(&map_state);

    args.write_map(&template)?;

    Ok(())
}

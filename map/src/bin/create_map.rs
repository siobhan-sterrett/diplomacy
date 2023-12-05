use std::{fs, path};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let f = fs::File::open(args.state_file)?;
    let map_state: MapState = serde_yaml::from_reader(f)?;

    let mut template = MapTemplate {
        units: vec![],
        supply_centers: vec![],
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

    let output_path = args.output.unwrap_or(path::PathBuf::from("map.svg"));
    let mut f = fs::File::create(output_path)?;
    template.write_into(&mut f)?;

    Ok(())
}

//! Build script for Diplomacy lib.
//! Reads in the map definition from defs/map.yaml, and generates Rust code from those definitions.

use std::{collections::BTreeMap, error::Error, fmt, fs, io::Write};

use proc_macro2::{Ident, Span};
use syn::{parse_quote, Expr};

extern crate quote;
#[macro_use]
extern crate serde;

struct Province {
    ident: Ident,
    display_name: String,
    country: Expr,
    supply_center: bool,
    terrain: Expr,
}

struct ArmySpace {
    ident: Ident,
    display_name: String,
    province: Ident,
    neighbors: Vec<Ident>,
}

struct FleetSpace {
    ident: Ident,
    display_name: String,
    province: Ident,
    neighbors: Vec<Ident>,
}

#[derive(Clone, Debug, Deserialize)]
struct ProvinceDefinition {
    display_name: String,
    country: Option<String>,
    neighbors: Vec<String>,
    #[serde(default)]
    supply_center: bool,
    terrain: String,
    coasts: Option<BTreeMap<String, Vec<String>>>,
}

type ProvinceDefinitionMap = BTreeMap<String, ProvinceDefinition>;

fn load_province_definitions() -> Result<ProvinceDefinitionMap, Box<dyn Error>> {
    let f = fs::File::open("defs/map.yaml")?;
    let province_definitions = serde_yaml::from_reader(f)?;
    Ok(province_definitions)
}

fn get_provinces(
    province_definitions: &ProvinceDefinitionMap,
) -> Result<Vec<Province>, Box<dyn Error>> {
    let provinces = province_definitions
        .iter()
        .map(|(ident, def)| {
            let ident = Ident::new(ident, Span::call_site());
            let display_name = def.display_name.clone();
            let country = match &def.country {
                Some(name) => {
                    let country_ident = Ident::new(&name, Span::call_site());
                    parse_quote! {
                        Some(super::country::Country::#country_ident)
                    }
                }
                None => parse_quote! { None },
            };
            let supply_center = def.supply_center;
            let terrain = Ident::new(&def.terrain, Span::call_site());
            let terrain = parse_quote! { super::terrain::Terrain::#terrain };

            Province {
                ident,
                display_name,
                country,
                supply_center,
                terrain,
            }
        })
        .collect();

    Ok(provinces)
}

fn build_province_enum(provinces: &[Province]) -> String {
    let idents = provinces.iter().map(|p| &p.ident);
    let file: syn::File = parse_quote! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub enum Province {
            #(#idents),*
        }
    };
    prettyplease::unparse(&file)
}

fn build_province_display_impl(provinces: &[Province]) -> String {
    let idents = provinces.iter().map(|p| &p.ident);
    let display_names = provinces.iter().map(|p| &p.display_name);
    let file: syn::File = parse_quote! {
        impl core::fmt::Display for Province {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match &self {
                    #(&Province::#idents => write!(f, "{}", #display_names)),*
                }
            }
        }
    };
    prettyplease::unparse(&file)
}

fn build_province_fn_country(provinces: &[Province]) -> syn::File {
    let idents = provinces.iter().map(|p| &p.ident);
    let countries = provinces.iter().map(|p| &p.country);
    parse_quote! {
        pub fn country(&self) -> Option<super::country::Country> {
            match &self {
                #(&Province::#idents => #countries),*
            }
        }
    }
}

fn build_province_fn_supply_center(provinces: &[Province]) -> syn::File {
    let idents = provinces.iter().map(|p| &p.ident);
    let supply_centers = provinces.iter().map(|p| &p.supply_center);
    parse_quote! {
        pub fn supply_center(&self) -> bool {
            match &self {
                #(&Province::#idents => #supply_centers),*
            }
        }
    }
}

fn build_province_fn_terrain(provinces: &[Province]) -> syn::File {
    let idents = provinces.iter().map(|p| &p.ident);
    let terrains = provinces.iter().map(|p| &p.terrain);
    parse_quote! {
        pub fn terrain(&self) -> super::terrain::Terrain {
            match &self {
                #(&Province::#idents => #terrains),*
            }
        }
    }
}

fn build_province_impl(provinces: &[Province]) -> String {
    let fn_countries = build_province_fn_country(provinces);
    let fn_supply_center = build_province_fn_supply_center(provinces);
    let fn_terrain = build_province_fn_terrain(provinces);
    let file = parse_quote! {
        impl Province {
            #fn_countries
            #fn_supply_center
            #fn_terrain
        }
    };
    prettyplease::unparse(&file)
}

fn build_province_rs(provinces: &[Province]) -> Result<(), Box<dyn Error>> {
    let mut f = fs::File::create("src/province.rs")?;

    writeln!(f, "{}", &build_province_enum(&provinces))?;
    writeln!(f, "{}", &build_province_display_impl(&provinces))?;
    writeln!(f, "{}", &build_province_impl(&provinces))?;

    Ok(())
}

#[derive(Clone, Debug)]
struct NoSuchProvince(String);

impl fmt::Display for NoSuchProvince {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No such province: {}", self.0)
    }
}

impl Error for NoSuchProvince {}

fn get_army_spaces(
    province_definitions: &ProvinceDefinitionMap,
) -> Result<Vec<ArmySpace>, Box<dyn Error>> {
    let mut army_spaces = Vec::new();

    for (ident, def) in province_definitions {
        if def.terrain == "Water" {
            continue;
        }
        let display_name = def.display_name.clone();
        let mut neighbors = Vec::new();
        for neighbor in &def.neighbors {
            if let Some(neighbor_def) = province_definitions.get(neighbor) {
                if neighbor_def.terrain == "Water" {
                    continue;
                }
                neighbors.push(Ident::new(&neighbor, Span::call_site()));
            } else {
                return Err(NoSuchProvince(neighbor.clone()).into());
            }
        }
        let province = Ident::new(&ident, Span::call_site());
        let ident = Ident::new(&ident, Span::call_site());
        army_spaces.push(ArmySpace {
            ident,
            display_name,
            province,
            neighbors,
        });
    }

    Ok(army_spaces)
}

fn build_army_space_enum(army_spaces: &[ArmySpace]) -> String {
    let idents = army_spaces.iter().map(|space| &space.ident);
    let file: syn::File = parse_quote! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub enum ArmySpace {
            #(#idents),*
        }
    };
    prettyplease::unparse(&file)
}

fn build_army_space_display_impl(army_spaces: &[ArmySpace]) -> String {
    let idents = army_spaces.iter().map(|space| &space.ident);
    let display_names = army_spaces.iter().map(|space| &space.display_name);
    let file: syn::File = parse_quote! {
        impl core::fmt::Display for ArmySpace {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match &self {
                    #(&ArmySpace::#idents => write!(f, "{}", #display_names)),*
                }
            }
        }
    };
    prettyplease::unparse(&file)
}

fn build_army_space_fn_province(army_spaces: &[ArmySpace]) -> syn::File {
    let idents = army_spaces.iter().map(|space| &space.ident);
    let provinces = army_spaces.iter().map(|space| &space.province);

    parse_quote! {
        pub fn province(&self) -> super::province::Province {
            match &self {
                #(ArmySpace::#idents => super::province::Province::#provinces),*
            }
        }
    }
}

fn build_army_space_fn_neighbors(army_spaces: &[ArmySpace]) -> syn::File {
    let idents = army_spaces.iter().map(|space| &space.ident);
    let neighbors = army_spaces
        .iter()
        .map(|space| &space.neighbors)
        .map(|neighbors| {
            let neighbors: syn::Expr = parse_quote! {
                &[#(ArmySpace::#neighbors),*]
            };
            neighbors
        });

    parse_quote! {
        pub fn neighbors(&self) -> &'static [ArmySpace] {
            match (&self) {
                #(ArmySpace::#idents => #neighbors),*
            }
        }
    }
}

fn build_army_space_impl(army_spaces: &[ArmySpace]) -> String {
    let fn_province = build_army_space_fn_province(army_spaces);
    let fn_neighbors = build_army_space_fn_neighbors(army_spaces);
    let file: syn::File = parse_quote! {
        impl ArmySpace {
            #fn_province
            #fn_neighbors
        }
    };
    prettyplease::unparse(&file)
}

fn build_army_space_rs(army_spaces: &[ArmySpace]) -> Result<(), Box<dyn Error>> {
    let mut f = fs::File::create("src/army_space.rs")?;

    writeln!(f, "{}", &build_army_space_enum(army_spaces))?;
    writeln!(f, "{}", &build_army_space_display_impl(army_spaces))?;
    writeln!(f, "{}", &build_army_space_impl(army_spaces))?;

    Ok(())
}

fn get_fleet_spaces(
    province_definitions: &ProvinceDefinitionMap,
) -> Result<Vec<FleetSpace>, Box<dyn Error>> {
    let mut fleet_spaces = Vec::new();

    for (ident, def) in province_definitions {
        if def.terrain == "Inland" {
            continue;
        }

        let province = Ident::new(&ident, Span::call_site());
        let display_name = def.display_name.clone();

        if let Some(coasts) = &def.coasts {
            for (coast, neighbors) in coasts {
                let display_name = format!("{} ({} Coast)", display_name, coast);
                let neighbors = neighbors
                    .iter()
                    .map(|neighbor_ident| Ident::new(neighbor_ident, Span::call_site()))
                    .collect();
                let ident = Ident::new(&format!("{}{}Coast", ident, coast), Span::call_site());

                fleet_spaces.push(FleetSpace {
                    ident,
                    display_name,
                    province: province.clone(),
                    neighbors,
                });
            }
        } else {
            let mut neighbors = Vec::new();
            for neighbor in &def.neighbors {
                if let Some(neighbor_def) = province_definitions.get(neighbor) {
                    if neighbor_def.terrain == "Inland" {
                        continue;
                    }

                    if let Some(coasts) = &neighbor_def.coasts {
                        for (coast, coast_neighbors) in coasts {
                            if coast_neighbors.contains(ident) {
                                let neighbor_ident = Ident::new(
                                    &format!("{}{}Coast", neighbor, coast),
                                    Span::call_site(),
                                );
                                neighbors.push(neighbor_ident);
                            }
                        }
                    } else {
                        neighbors.push(Ident::new(&neighbor, Span::call_site()));
                    }
                } else {
                    return Err(NoSuchProvince(neighbor.clone()).into());
                }
            }
            let ident = Ident::new(&ident, Span::call_site());
            fleet_spaces.push(FleetSpace {
                ident,
                display_name,
                province,
                neighbors,
            });
        }
    }

    Ok(fleet_spaces)
}

fn build_fleet_space_enum(fleet_spaces: &[FleetSpace]) -> String {
    let idents = fleet_spaces.iter().map(|space| &space.ident);
    let file: syn::File = parse_quote! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub enum FleetSpace {
            #(#idents),*
        }
    };
    prettyplease::unparse(&file)
}

fn build_fleet_space_display_impl(fleet_spaces: &[FleetSpace]) -> String {
    let idents = fleet_spaces.iter().map(|space| &space.ident);
    let display_names = fleet_spaces.iter().map(|space| &space.display_name);
    let file: syn::File = parse_quote! {
        impl core::fmt::Display for FleetSpace {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match &self {
                    #(&FleetSpace::#idents => write!(f, "{}", #display_names)),*
                }
            }
        }
    };
    prettyplease::unparse(&file)
}

fn build_fleet_space_fn_province(fleet_spaces: &[FleetSpace]) -> syn::File {
    let idents = fleet_spaces.iter().map(|space| &space.ident);
    let provinces = fleet_spaces.iter().map(|space| &space.province);

    parse_quote! {
        pub fn province(&self) -> super::province::Province {
            match &self {
                #(FleetSpace::#idents => super::province::Province::#provinces),*
            }
        }
    }
}

fn build_fleet_space_fn_neighbors(fleet_spaces: &[FleetSpace]) -> syn::File {
    let idents = fleet_spaces.iter().map(|space| &space.ident);
    let neighbors = fleet_spaces
        .iter()
        .map(|space| &space.neighbors)
        .map(|neighbors| {
            let neighbors: syn::Expr = parse_quote! {
                &[#(FleetSpace::#neighbors),*]
            };
            neighbors
        });

    parse_quote! {
        pub fn neighbors(&self) -> &'static [FleetSpace] {
            match (&self) {
                #(FleetSpace::#idents => #neighbors),*
            }
        }
    }
}

fn build_fleet_space_impl(fleet_spaces: &[FleetSpace]) -> String {
    let fn_province = build_fleet_space_fn_province(fleet_spaces);
    let fn_neighbors = build_fleet_space_fn_neighbors(fleet_spaces);
    let file: syn::File = parse_quote! {
        impl FleetSpace {
            #fn_province
            #fn_neighbors
        }
    };
    prettyplease::unparse(&file)
}

fn build_fleet_space_rs(fleet_spaces: &[FleetSpace]) -> Result<(), Box<dyn Error>> {
    let mut f = fs::File::create("src/fleet_space.rs")?;

    writeln!(f, "{}", &build_fleet_space_enum(fleet_spaces))?;
    writeln!(f, "{}", &build_fleet_space_display_impl(fleet_spaces))?;
    writeln!(f, "{}", &build_fleet_space_impl(fleet_spaces))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=defs/map.yaml");

    let province_definitions = load_province_definitions()?;
    let provinces = get_provinces(&province_definitions)?;

    build_province_rs(&provinces)?;

    let army_spaces = get_army_spaces(&province_definitions)?;
    build_army_space_rs(&army_spaces)?;

    let fleet_spaces = get_fleet_spaces(&province_definitions)?;
    build_fleet_space_rs(&fleet_spaces)?;

    Ok(())
}

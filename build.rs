//! Build script for Diplomacy lib.
//! Reads in the map definition from defs/map.yaml, and generates Rust code from those definitions.

use std::{
    collections::HashMap,
    error::Error,
    fs,
    io::{self, Write},
};

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
    province: Province,
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
    coasts: Option<HashMap<String, Vec<String>>>,
}

type ProvinceDefinitionMap = HashMap<String, ProvinceDefinition>;

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
        fn country(&self) -> Option<super::country::Country> {
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
        fn supply_center(&self) -> bool {
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
        fn terrain(&self) -> super::terrain::Terrain {
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

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=defs/map.yaml");

    let province_definitions = load_province_definitions()?;
    let provinces = get_provinces(&province_definitions)?;

    build_province_rs(&provinces)?;

    Ok(())
}

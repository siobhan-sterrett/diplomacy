//! Build script for Diplomacy lib.
//! Reads in the map definition from defs/map.yaml, and generates Rust code from those definitions.

use std::{collections::HashMap, error::Error, fs, io::Write};

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
    ident: String,
    country: Option<String>,
    neighbors: Vec<String>,
    #[serde(default)]
    supply_center: bool,
    terrain: String,
    coasts: Option<HashMap<String, Vec<String>>>,
}

fn load_province_definitions() -> Result<Vec<ProvinceDefinition>, Box<dyn Error>> {
    let f = fs::File::open("defs/map.yaml")?;
    let province_definitions = serde_yaml::from_reader(f)?;
    Ok(province_definitions)
}

fn get_provinces() -> Result<Vec<Province>, Box<dyn Error>> {
    let province_definitions = load_province_definitions()?;

    let provinces = province_definitions
        .iter()
        .map(|def| {
            let ident = Ident::new(&def.ident, Span::call_site());
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

fn write_province_enum(province_definitions: &[ProvinceDefinition]) -> Result<(), Box<dyn Error>> {
    let idents: Vec<_> = province_definitions
        .into_iter()
        .map(|def| Ident::new(&def.ident, Span::call_site()))
        .collect();

    let countries: Vec<Expr> = province_definitions
        .into_iter()
        .map(|def| &def.country)
        .map(|country| match country {
            Some(name) => {
                let country_ident = Ident::new(&name, Span::call_site());
                parse_quote! {
                    Some(Country::#country_ident)
                }
            }
            None => parse_quote! {
                None
            },
        })
        .collect();

    let display_names: Vec<&String> = province_definitions
        .into_iter()
        .map(|def| &def.display_name)
        .collect();

    let neighbors: Vec<Expr> = province_definitions
        .iter()
        .map(|def| &def.neighbors)
        .map(|neighbors| {
            let idents: Vec<_> = neighbors
                .iter()
                .map(|p| Ident::new(&p, Span::call_site()))
                .collect();
            parse_quote! {
                &[#(Province::#idents),*]
            }
        })
        .collect();

    let supply_centers: Vec<bool> = province_definitions
        .iter()
        .map(|def| def.supply_center)
        .collect();

    let terrains: Vec<Ident> = province_definitions
        .iter()
        .map(|def| Ident::new(&def.terrain, Span::call_site()))
        .collect();

    let file: syn::File = parse_quote! {
        use crate::country::Country;
        use crate::terrain::Terrain;
        use std::fmt;

        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub enum Province {
            #(#idents),*
        }

        impl fmt::Display for Province {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match &self {
                    #(&Province::#idents => write!(f, "{}", #display_names)),*
                }
            }
        }

        impl Province {
            pub fn country(&self) -> Option<Country> {
                match &self {
                    #(&Province::#idents => #countries),*
                }
            }

            pub fn neighbors(&self) -> &'static [Province] {
                match &self {
                    #(&Province::#idents => #neighbors),*
                }
            }

            pub fn has_supply_center(&self) -> bool {
                match &self {
                    #(&Province::#idents => #supply_centers),*
                }
            }

            pub fn terrain(&self) -> Terrain {
                match &self {
                    #(&Province::#idents => Terrain::#terrains),*
                }
            }
        }
    };

    fs::write("src/province.rs", prettyplease::unparse(&file))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=defs/map.yaml");

    let provinces = get_provinces()?;

    let mut f = fs::File::create("src/province.rs")?;

    writeln!(f, "{}", &build_province_enum(&provinces))?;
    writeln!(f, "{}", &build_province_display_impl(&provinces))?;
    writeln!(f, "{}", &build_province_impl(&provinces))?;

    Ok(())
}

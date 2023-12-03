//! Build script for Diplomacy lib.
//! Reads in the map definition from defs/map.yaml, and generates Rust code from those definitions.

use std::{error::Error, fs, fs::File};

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_quote, Expr};

extern crate quote;
#[macro_use]
extern crate serde;

#[derive(Clone, Debug, Deserialize)]
struct ProvinceDefinition {
    display_name: String,
    ident: String,
    country: Option<String>,
    neighbors: Vec<String>,
    #[serde(default)]
    supply_center: bool,
    terrain: String,
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
            let enum_variants: Vec<_> = neighbors
                .iter()
                .map(|p| Ident::new(&p, Span::call_site()))
                .collect();
            parse_quote! {
                &[#(Province::#enum_variants),*]
            }
        })
        .collect();

    let supply_centers: Vec<Expr> = province_definitions
        .iter()
        .map(|def| &def.supply_center)
        .map(|supply_center| parse_quote! { #supply_center })
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

    let f = File::open("defs/map.yaml")?;
    let province_definitions: Vec<ProvinceDefinition> = serde_yaml::from_reader(f)?;

    let enum_variants: Vec<_> = province_definitions
        .iter()
        .map(|def| Ident::new(&def.ident, Span::call_site()))
        .collect();

    let display_names: Vec<_> = province_definitions
        .iter()
        .map(|def| &def.display_name)
        .collect();

    let countries: Vec<_> = province_definitions
        .iter()
        .map(|def| &def.country)
        .map(|country| match country {
            Some(name) => {
                let country_ident = Ident::new(&name, Span::call_site());
                quote! {
                    Some(Country::#country_ident)
                }
            }
            None => quote! {
                None
            },
        })
        .collect();

    let neighbors: Vec<_> = province_definitions
        .iter()
        .map(|def| &def.neighbors)
        .map(|neighbors| {
            let enum_variants: Vec<_> = neighbors
                .iter()
                .map(|p| Ident::new(&p, Span::call_site()))
                .collect();
            quote! {
                vec![Province::#(#enum_variants),*]
            }
        })
        .collect();

    write_province_enum(&province_definitions)?;

    Ok(())
}

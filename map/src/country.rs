use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub enum Country {
    Austria,
    England,
    France,
    Germany,
    Italy,
    Russia,
    Turkey,
}

impl core::fmt::Display for Country {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match &self {
            &Country::Austria => write!(f, "Austria"),
            &Country::England => write!(f, "England"),
            &Country::France => write!(f, "France"),
            &Country::Germany => write!(f, "Germany"),
            &Country::Italy => write!(f, "Italy"),
            &Country::Russia => write!(f, "Russia"),
            &Country::Turkey => write!(f, "Turkey"),
        }
    }
}

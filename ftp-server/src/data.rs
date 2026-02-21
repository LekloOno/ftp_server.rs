use strum_macros::{Display, EnumString};

#[derive(Debug, EnumString, Clone, Display)]
pub enum DataType {
    #[strum(serialize = "I")]
    Binary,
    #[strum(serialize = "A")]
    Ascii,
    #[strum(serialize = "E")]
    Ebcdic, // Probably not used ... 
}
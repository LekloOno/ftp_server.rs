use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum DataType {
    #[strum(serialize = "I")]
    Binary,
    #[strum(serialize = "A")]
    Ascii,
    #[strum(serialize = "E")]
    Ebcdic, // Probably not used ... 
}
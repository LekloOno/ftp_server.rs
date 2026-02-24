//! Provides an explicit enum definition for FTP data transfer types.
use strum_macros::{Display, EnumString};

/// Represents effective data transfer types.
/// 
/// All might not necessarily be supported, although represented.
#[derive(Debug, EnumString, Clone, Display)]
pub enum DataType {
    #[strum(serialize = "I")]
    Binary,
    #[strum(serialize = "A")]
    Ascii,
    #[strum(serialize = "E")]
    Ebcdic, // Probably not used ... 
}
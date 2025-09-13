use serde::Deserialize;

#[derive(
    Debug,
    Default,
    strum::Display,
    strum::EnumString,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
)]
pub enum Profile {
    #[serde(rename = "test")]
    #[strum(serialize = "test")]
    Test,

    #[default]
    #[serde(rename = "development")]
    #[strum(serialize = "development")]
    Development,

    #[serde(rename = "production")]
    #[strum(serialize = "production")]
    Production,
}

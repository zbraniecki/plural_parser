extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    pub supplemental: Supplemental 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    #[serde(rename = "_number")]
    pub number: String,
    #[serde(rename = "_unicodeVersion")]
    pub unicode_version: String,
    #[serde(rename = "_cldrVersion")]
    pub cldr_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Supplemental {
    pub version: Version,
    #[serde(rename = "plurals-type-cardinal")]
    pub rules: HashMap<String, HashMap<String, String>>
}

pub fn parse_plurals_resource<P: AsRef<Path>>(path: P) -> Result<Resource, Box<Error>> {
    let file = File::open(path)?;
    let u = serde_json::from_reader(file)?;
    Ok(u)
}

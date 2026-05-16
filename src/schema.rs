use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{bridge, error::Result, ffi};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct SchemaCatalog {
    pub reserved: BTreeMap<String, String>,
    pub preferences: BTreeMap<String, String>,
    pub components: BTreeMap<String, String>,
    pub entities: BTreeMap<String, String>,
    pub generic_properties: BTreeMap<String, String>,
    pub ipv4: BTreeMap<String, String>,
    pub ipv6: BTreeMap<String, String>,
    pub dns: BTreeMap<String, String>,
    pub proxies: BTreeMap<String, String>,
    pub interface_types: BTreeMap<String, String>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Schema;

impl Schema {
    pub fn catalog() -> Result<SchemaCatalog> {
        bridge::parse_json("sc_schema_copy_catalog", unsafe { ffi::schema::sc_schema_copy_catalog() })
    }
}

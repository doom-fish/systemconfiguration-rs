use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{bridge, error::Result, ffi};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
/// Wraps grouped symbols from `SCSchemaDefinitions.h`.
pub struct SchemaCatalog {
    /// Wraps all exported symbols from `SCSchemaDefinitions.h`.
    pub all: BTreeMap<String, String>,
    /// Wraps reserved symbols from `SCSchemaDefinitions.h`.
    pub reserved: BTreeMap<String, String>,
    /// Wraps preference symbols from `SCSchemaDefinitions.h`.
    pub preferences: BTreeMap<String, String>,
    /// Wraps component symbols from `SCSchemaDefinitions.h`.
    pub components: BTreeMap<String, String>,
    /// Wraps entity symbols from `SCSchemaDefinitions.h`.
    pub entities: BTreeMap<String, String>,
    /// Wraps generic property symbols from `SCSchemaDefinitions.h`.
    pub generic_properties: BTreeMap<String, String>,
    /// Wraps IPv4 symbols from `SCSchemaDefinitions.h`.
    pub ipv4: BTreeMap<String, String>,
    /// Wraps IPv6 symbols from `SCSchemaDefinitions.h`.
    pub ipv6: BTreeMap<String, String>,
    /// Wraps DNS symbols from `SCSchemaDefinitions.h`.
    pub dns: BTreeMap<String, String>,
    /// Wraps proxy symbols from `SCSchemaDefinitions.h`.
    pub proxies: BTreeMap<String, String>,
    /// Wraps interface-type symbols from `SCSchemaDefinitions.h`.
    pub interface_types: BTreeMap<String, String>,
}

impl SchemaCatalog {
    /// Wraps symbol lookup within `SCSchemaDefinitions.h`.
    pub fn get(&self, symbol: &str) -> Option<&str> {
        self.all.get(symbol).map(String::as_str)
    }

    /// Wraps symbol membership checks within `SCSchemaDefinitions.h`.
    pub fn contains(&self, symbol: &str) -> bool {
        self.all.contains_key(symbol)
    }
}

#[derive(Clone, Copy, Debug, Default)]
/// Wraps the `SCSchemaDefinitions.h` catalog helpers.
pub struct Schema;

impl Schema {
    /// Wraps grouped constants from `SCSchemaDefinitions.h`.
    pub fn catalog() -> Result<SchemaCatalog> {
        bridge::parse_json("sc_schema_copy_catalog", unsafe {
            ffi::schema::sc_schema_copy_catalog()
        })
    }
}

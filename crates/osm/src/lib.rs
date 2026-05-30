//! # quantum-doom-osm
//!
//! Fetches OpenStreetMap features and builds a [`LevelBlueprint`] for
//! **[Quantum Town: LA](https://github.com/thepriben/quantum-town-la)**.
//!
//! Sprint 2 scope:
//! - `building=*` footprints extruded into Rapier colliders
//! - `highway=*` polylines as navigable zones
//! - 200 m radius around a Los Angeles GPS anchor

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Map center and fetch radius (metres).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoAnchor {
    /// WGS84 latitude
    pub lat: f64,
    /// WGS84 longitude
    pub lon: f64,
    /// Overpass search radius in metres
    pub radius_m: f32,
}

impl Default for GeoAnchor {
    fn default() -> Self {
        Self {
            lat: 34.0522,
            lon: -118.2437,
            radius_m: 200.0,
        }
    }
}

/// One OSM building footprint ready for mesh extrusion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingFootprint {
    pub osm_id: i64,
    pub outer_ring: Vec<[f64; 2]>,
    pub height_m: f32,
    pub tags: BTreeMap<String, String>,
}

/// Serializable level description consumed by the game crate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelBlueprint {
    pub anchor: GeoAnchor,
    pub buildings: Vec<BuildingFootprint>,
    pub navigable_polylines: Vec<Vec<[f64; 2]>>,
}

/// OSM fetch / parse failures.
#[derive(Debug, thiserror::Error)]
pub enum OsmError {
    #[error("overpass request failed: {0}")]
    Overpass(String),
    #[error("parse error: {0}")]
    Parse(String),
}

/// Download OSM data for `anchor` and return a gameplay blueprint.
///
/// Sprint 2 implementation will call Overpass with `building` + `highway` filters.
pub fn fetch_level(_anchor: &GeoAnchor) -> Result<LevelBlueprint, OsmError> {
    Err(OsmError::Overpass(
        "Sprint 2: implement Overpass fetch in crates/osm/src/overpass.rs".into(),
    ))
}

use diesel::SqliteConnection;

use crate::data::buildings::DEFAULT_BUILDINGS;
use crate::data::factions::DEFAULT_FACTIONS;
use crate::db::{buildings, factions};

pub fn initialize_database(conn: &mut SqliteConnection) {
    if !has_all_factions(conn) {
        log::info!("Creating default factions");
        for fac in DEFAULT_FACTIONS.iter() {
            factions::create_faction(conn, fac);
        }
    }

    if !has_all_buildings(conn) {
        log::info!("Creating default buildings");
        for building in DEFAULT_BUILDINGS.iter() {
            buildings::create_building(conn, building);
        }
    }

    log::info!("Database initialized");
}

fn has_all_factions(conn: &mut SqliteConnection) -> bool {
    let facs: Vec<String> = factions::get_all(conn)
        .unwrap_or_default()
        .iter()
        .map(|f| f.id.clone())
        .collect();
    if facs.is_empty() || facs.len() != DEFAULT_FACTIONS.len() {
        return false;
    }
    for fac in DEFAULT_FACTIONS.iter() {
        if !facs.contains(&String::from(fac.id)) {
            return false;
        }
    }
    true
}

fn has_all_buildings(conn: &mut SqliteConnection) -> bool {
    let buildings: Vec<String> = buildings::get_all(conn)
        .unwrap_or_default()
        .iter()
        .map(|b| b.name.clone())
        .collect();
    if buildings.is_empty() || buildings.len() != DEFAULT_BUILDINGS.len() {
        return false;
    }
    for building in DEFAULT_BUILDINGS.iter() {
        if !buildings.contains(&String::from(building.name)) {
            return false;
        }
    }
    true
}

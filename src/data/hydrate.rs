use crate::data::buildings::DEFAULT_BUILDINGS;
use crate::data::factions::DEFAULT_FACTIONS;
use crate::db::{DbConn, Repository};
use crate::db::buildings::BuildingRepository;
use crate::db::factions::FactionRepository;
use crate::models::error::EmpResult;

pub fn initialize_database(conn: &mut DbConn) -> EmpResult<()> {
    let mut factions_repo = FactionRepository {};
    let mut building_repo = BuildingRepository {};

    if !has_all_factions(conn, &factions_repo) {
        log::info!("Creating default factions");
        for fac in DEFAULT_FACTIONS.iter() {
            factions_repo.create(conn, fac)?;
        }
    }

    if !has_all_buildings(conn, &building_repo) {
        log::info!("Creating default buildings");
        for building in DEFAULT_BUILDINGS.iter() {
            building_repo.create(conn, building)?;
        }
    }

    log::info!("Database initialized");
    Ok(())
}

fn has_all_factions(conn: &mut DbConn, repo: &FactionRepository) -> bool {
    let facs: Vec<i32> = repo
        .get_all(conn)
        .unwrap_or_default()
        .iter()
        .map(|f| f.id.clone())
        .collect();
    if facs.is_empty() || facs.len() != DEFAULT_FACTIONS.len() {
        return false;
    }
    for fac in DEFAULT_FACTIONS.iter() {
        if !facs.contains(&fac.id) {
            return false;
        }
    }
    true
}

fn has_all_buildings(conn: &mut DbConn, repo: &BuildingRepository) -> bool {
    let buildings: Vec<String> = repo
        .get_all(conn)
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

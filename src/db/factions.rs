use diesel::prelude::*;

use crate::models::faction::{Faction, NewFaction};
use crate::schema::factions;

pub fn create_faction(conn: &mut SqliteConnection, new_faction: &NewFaction) -> Faction {
    diesel::insert_into(factions::table)
        .values(new_faction)
        .returning(Faction::as_returning())
        .get_result(conn)
        .expect("Error creating faction")
}

pub fn get_all(conn: &mut SqliteConnection) -> QueryResult<Vec<Faction>> {
    factions::table.select(Faction::as_select()).load(conn)
}

use diesel::prelude::*;

use crate::models::faction::{Faction, NewFaction};

pub fn create_faction(conn: &mut SqliteConnection, name: &str) -> Faction {
    use crate::schema::factions;

    let new_building = NewFaction { name };

    diesel::insert_into(factions::table)
        .values(&new_building)
        .returning(Faction::as_returning())
        .get_result(conn)
        .expect("Error creating faction")
}

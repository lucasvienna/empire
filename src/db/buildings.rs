use diesel::prelude::*;

use crate::models::building::{Building, NewBuilding};
use crate::schema::buildings;

pub fn create_building(conn: &mut SqliteConnection, new_building: &NewBuilding) -> Building {
    diesel::insert_into(buildings::table)
        .values(new_building)
        .returning(Building::as_returning())
        .get_result(conn)
        .expect("Error creating building")
}

pub fn get_all(conn: &mut SqliteConnection) -> QueryResult<Vec<Building>> {
    buildings::table.select(Building::as_select()).load(conn)
}

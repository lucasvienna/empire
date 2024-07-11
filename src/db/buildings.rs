use diesel::prelude::*;

use crate::models::building::{Building, NewBuilding};

pub fn create_building(
    conn: &mut SqliteConnection,
    name: &str,
    max_level: i32,
    faction: &str,
) -> Building {
    use crate::schema::buildings;

    let new_building = NewBuilding {
        name,
        max_level,
        faction: Option::from(faction),
    };

    diesel::insert_into(buildings::table)
        .values(&new_building)
        .returning(Building::as_returning())
        .get_result(conn)
        .expect("Error creating building")
}

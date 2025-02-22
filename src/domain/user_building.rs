use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::user_buildings;

#[derive(Queryable, Selectable, Identifiable, AsChangeset, Debug)]
#[diesel(table_name = user_buildings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserBuilding {
    pub id: PK,
    pub user_id: Uuid,
    pub building_id: i32,
    pub level: i32,
    pub upgrade_time: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = user_buildings)]
pub struct NewUserBuilding<'a> {
    pub user_id: Uuid,
    pub building_id: i32,
    pub level: Option<i32>,
    pub upgrade_time: Option<&'a str>,
}

pub type PK = Uuid;

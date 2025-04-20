use crate::domain::user::{User, UserKey};
use crate::schema::user_buildings;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

pub type UserBuildingKey = Uuid;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_buildings, check_for_backend(diesel::pg::Pg))]
pub struct UserBuilding {
    pub id: UserBuildingKey,
    pub user_id: UserKey,
    pub building_id: i32,
    pub level: i32,
    pub upgrade_time: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = user_buildings, check_for_backend(diesel::pg::Pg))]
pub struct NewUserBuilding {
    pub user_id: UserKey,
    pub building_id: i32,
    pub level: Option<i32>,
    pub upgrade_time: Option<String>,
}

#[derive(Identifiable, AsChangeset, Debug, Clone, PartialEq, Eq, Hash)]
#[diesel(table_name = user_buildings, check_for_backend(diesel::pg::Pg))]
pub struct UpdateUserBuilding {
    pub id: UserBuildingKey,
    pub level: Option<i32>,
    pub upgrade_time: Option<String>,
}

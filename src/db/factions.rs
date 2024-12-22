use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::models::error::EmpResult;
use crate::models::faction;
use crate::models::faction::{Faction, NewFaction};
use crate::schema::factions;

pub struct FactionRepository {}

impl Repository<Faction, NewFaction<'static>, faction::PK> for FactionRepository {
    fn get_all(&self, connection: &mut DbConn) -> EmpResult<Vec<Faction>> {
        let factions = factions::table
            .select(Faction::as_select())
            .load(connection)?;
        Ok(factions)
    }

    fn get_by_id(&self, connection: &mut DbConn, id: &faction::PK) -> EmpResult<Faction> {
        let faction = factions::table.find(id).first(connection)?;
        Ok(faction)
    }

    fn create(&self, connection: &mut DbConn, entity: &NewFaction<'static>) -> EmpResult<Faction> {
        let faction = diesel::insert_into(factions::table)
            .values(entity)
            .returning(Faction::as_returning())
            .get_result(connection)?;
        Ok(faction)
    }

    fn update(&self, connection: &mut DbConn, entity: &Faction) -> EmpResult<Faction> {
        let faction = diesel::update(factions::table.find(&entity.id))
            .set(entity)
            .get_result(connection)?;
        Ok(faction)
    }

    fn delete(&self, connection: &mut DbConn, id: &faction::PK) -> EmpResult<usize> {
        let res = diesel::delete(factions::table.find(id)).execute(connection)?;
        Ok(res)
    }
}

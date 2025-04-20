use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::domain::error::Result;
use crate::domain::factions::{Faction, FactionKey, NewFaction, UpdateFaction};
use crate::schema::faction::dsl::*;

pub struct FactionRepository {}

impl Repository<Faction, NewFaction, UpdateFaction, FactionKey> for FactionRepository {
    fn get_all(&self, connection: &mut DbConn) -> Result<Vec<Faction>> {
        let fac_list = faction.select(Faction::as_select()).load(connection)?;
        Ok(fac_list)
    }

    fn get_by_id(&self, connection: &mut DbConn, faction_id: &FactionKey) -> Result<Faction> {
        let fac = faction.find(faction_id).first(connection)?;
        Ok(fac)
    }

    fn create(&self, connection: &mut DbConn, entity: NewFaction) -> Result<Faction> {
        let new_faction = diesel::insert_into(faction)
            .values(entity)
            .returning(Faction::as_returning())
            .get_result(connection)?;
        Ok(new_faction)
    }

    fn update(
        &self,
        connection: &mut DbConn,
        faction_id: &FactionKey,
        changeset: UpdateFaction,
    ) -> Result<Faction> {
        let updated_faction = diesel::update(faction.find(faction_id))
            .set(changeset)
            .get_result(connection)?;
        Ok(updated_faction)
    }

    fn delete(&self, connection: &mut DbConn, faction_id: &FactionKey) -> Result<usize> {
        let rows_deleted = diesel::delete(faction.find(faction_id)).execute(connection)?;
        Ok(rows_deleted)
    }
}

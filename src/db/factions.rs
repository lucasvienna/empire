use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::domain::error::Result;
use crate::domain::faction::{Faction, NewFaction, UpdateFaction, PK as FactionKey};
use crate::schema::factions::dsl::*;

pub struct FactionRepository {}

impl Repository<Faction, NewFaction, UpdateFaction, FactionKey> for FactionRepository {
    fn get_all(&self, connection: &mut DbConn) -> Result<Vec<Faction>> {
        let fac_list = factions.select(Faction::as_select()).load(connection)?;
        Ok(fac_list)
    }

    fn get_by_id(&self, connection: &mut DbConn, faction_id: &FactionKey) -> Result<Faction> {
        let faction = factions.find(faction_id).first(connection)?;
        Ok(faction)
    }

    fn create(&self, connection: &mut DbConn, entity: NewFaction) -> Result<Faction> {
        let faction = diesel::insert_into(factions)
            .values(entity)
            .returning(Faction::as_returning())
            .get_result(connection)?;
        Ok(faction)
    }

    fn update(
        &self,
        connection: &mut DbConn,
        faction_id: &FactionKey,
        changeset: UpdateFaction,
    ) -> Result<Faction> {
        let faction = diesel::update(factions.find(faction_id))
            .set(changeset)
            .get_result(connection)?;
        Ok(faction)
    }

    fn delete(&self, connection: &mut DbConn, faction_id: &FactionKey) -> Result<usize> {
        let res = diesel::delete(factions.find(faction_id)).execute(connection)?;
        Ok(res)
    }
}

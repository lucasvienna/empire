use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::domain::modifier::active_modifier::{
    ActiveModifier, NewActiveModifier, UpdateActiveModifier, PK as ActiveModifierKey,
};
use crate::domain::user;
use crate::schema::active_modifiers::dsl::*;
use crate::Result;

pub struct ActiveModifiersRepository {}

impl Repository<ActiveModifier, NewActiveModifier, UpdateActiveModifier, ActiveModifierKey>
    for ActiveModifiersRepository
{
    fn get_all(&self, connection: &mut DbConn) -> Result<Vec<ActiveModifier>> {
        let mod_list = active_modifiers
            .select(ActiveModifier::as_select())
            .load(connection)?;
        Ok(mod_list)
    }

    fn get_by_id(
        &self,
        connection: &mut DbConn,
        mod_id: &ActiveModifierKey,
    ) -> Result<ActiveModifier> {
        let modifier = active_modifiers.find(mod_id).first(connection)?;
        Ok(modifier)
    }

    fn create(&self, connection: &mut DbConn, entity: NewActiveModifier) -> Result<ActiveModifier> {
        let modifier = diesel::insert_into(active_modifiers)
            .values(entity)
            .returning(ActiveModifier::as_returning())
            .get_result(connection)?;
        Ok(modifier)
    }

    fn update(
        &self,
        connection: &mut DbConn,
        mod_id: &ActiveModifierKey,
        changeset: UpdateActiveModifier,
    ) -> Result<ActiveModifier> {
        let modifier = diesel::update(active_modifiers.find(mod_id))
            .set(changeset)
            .get_result(connection)?;
        Ok(modifier)
    }

    fn delete(&self, connection: &mut DbConn, mod_id: &ActiveModifierKey) -> Result<usize> {
        let deleted_count = diesel::delete(active_modifiers.find(mod_id)).execute(connection)?;
        Ok(deleted_count)
    }
}

impl ActiveModifiersRepository {
    pub fn get_by_user_id(
        &self,
        connection: &mut DbConn,
        usr_id: &user::PK,
    ) -> Result<Vec<ActiveModifier>> {
        let active_mods = active_modifiers
            .filter(user_id.eq(usr_id))
            .get_results(connection)?;
        Ok(active_mods)
    }
}

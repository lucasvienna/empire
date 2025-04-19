use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::domain::modifier::{Modifier, NewModifier, UpdateModifier, PK as ModifierKey};
use crate::schema::modifiers::dsl::*;
use crate::Result;

pub struct ModifiersRepository {}

impl Repository<Modifier, NewModifier, UpdateModifier, ModifierKey> for ModifiersRepository {
    fn get_all(&self, connection: &mut DbConn) -> Result<Vec<Modifier>> {
        let mod_list = modifiers.select(Modifier::as_select()).load(connection)?;
        Ok(mod_list)
    }

    fn get_by_id(&self, connection: &mut DbConn, modifier_id: &ModifierKey) -> Result<Modifier> {
        let modifier = modifiers.find(modifier_id).first(connection)?;
        Ok(modifier)
    }

    fn create(&self, connection: &mut DbConn, entity: NewModifier) -> Result<Modifier> {
        let modifier = diesel::insert_into(modifiers)
            .values(entity)
            .returning(Modifier::as_returning())
            .get_result(connection)?;
        Ok(modifier)
    }

    fn update(
        &self,
        connection: &mut DbConn,
        modifier_id: &ModifierKey,
        changeset: UpdateModifier,
    ) -> Result<Modifier> {
        let modifier = diesel::update(modifiers.find(modifier_id))
            .set(changeset)
            .get_result(connection)?;
        Ok(modifier)
    }

    fn delete(&self, connection: &mut DbConn, modifier_id: &ModifierKey) -> Result<usize> {
        let deleted_count = diesel::delete(modifiers.find(modifier_id)).execute(connection)?;
        Ok(deleted_count)
    }
}

use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::domain::error::Result;
use crate::domain::player::{NewPlayer, Player, PlayerKey, UpdatePlayer};
use crate::schema::player::dsl::*;

pub struct PlayerRepository;

impl Repository<Player, NewPlayer, &UpdatePlayer, PlayerKey> for PlayerRepository {
    fn get_all(&self, conn: &mut DbConn) -> Result<Vec<Player>> {
        let player_list = player.select(Player::as_select()).load(conn)?;
        Ok(player_list)
    }

    fn get_by_id(&self, conn: &mut DbConn, player_id: &PlayerKey) -> Result<Player> {
        let player_ = player.find(player_id).first(conn)?;
        Ok(player_)
    }

    fn create(&self, conn: &mut DbConn, entity: NewPlayer) -> Result<Player> {
        let player_ = diesel::insert_into(player)
            .values(entity)
            .returning(Player::as_returning())
            .get_result(conn)?;
        Ok(player_)
    }

    fn update(&self, conn: &mut DbConn, changeset: &UpdatePlayer) -> Result<Player> {
        let player_ = diesel::update(player).set(changeset).get_result(conn)?;
        Ok(player_)
    }

    fn delete(&self, conn: &mut DbConn, player_id: &PlayerKey) -> Result<usize> {
        let deleted_count = diesel::delete(player.find(player_id)).execute(conn)?;
        Ok(deleted_count)
    }
}

impl PlayerRepository {
    pub fn find_by_id(&self, conn: &mut DbConn, player_id: &PlayerKey) -> Result<Option<Player>> {
        let player_: Option<Player> = player.find(player_id).first(conn).optional()?;
        Ok(player_)
    }

    pub fn get_by_name(&self, conn: &mut DbConn, player_name: impl AsRef<str>) -> Result<Player> {
        let player_ = player.filter(name.eq(player_name.as_ref())).first(conn)?;
        Ok(player_)
    }

    pub fn find_by_name(
        &self,
        conn: &mut DbConn,
        player_name: impl AsRef<str>,
    ) -> Result<Option<Player>> {
        let player_: Option<Player> = player
            .filter(name.eq(player_name.as_ref()))
            .first(conn)
            .optional()?;
        Ok(player_)
    }

    pub fn exists_by_name(&self, conn: &mut DbConn, player_name: impl AsRef<str>) -> Result<bool> {
        let player_ = self.find_by_name(conn, player_name)?;
        Ok(player_.is_some())
    }
}

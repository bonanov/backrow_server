use super::DieselError;
use crate::schema::roles;
use crate::schema::user_roles;

use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::ToSql;
use std::io::Write;

use crate::diesel::prelude::*;
use crate::diesel::*;

use chrono::NaiveDateTime;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Copy, Clone, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[sql_type = "Integer"]
pub enum PermissionState {
    Unset = -1,
    Forbidden = 0,
    Allowed = 1,
}

impl Default for PermissionState {
    fn default() -> PermissionState {
        PermissionState::Unset
    }
}

impl From<i32> for PermissionState {
    fn from(value: i32) -> PermissionState {
        match value {
            -1 => PermissionState::Unset,
            0 => PermissionState::Forbidden,
            1 => PermissionState::Allowed,
            _ => PermissionState::Unset,
        }
    }
}
impl<ST, DB> FromSql<ST, DB> for PermissionState
where
    i32: FromSql<ST, DB>,
    DB: Backend,
{
    fn from_sql(value: Option<&<DB as Backend>::RawValue>) -> deserialize::Result<Self> {
        <i32 as FromSql<ST, DB>>::from_sql(value).map(PermissionState::from)
    }
}
impl<DB> ToSql<Integer, DB> for PermissionState
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        <i32 as ToSql<Integer, DB>>::to_sql(&(*self as i32), out)
    }
}

#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[table_name = "roles"]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: i64,
    pub room_id: i64,
    pub name: String,
    pub color: Option<String>,
    pub is_default: bool,
    pub position: i32,

    pub title_update: PermissionState,
    pub path_update: PermissionState,
    pub public_update: PermissionState,
    pub room_delete: PermissionState,
    pub room_view: PermissionState,
    pub audit_log_read: PermissionState,
    pub embed_links: PermissionState,
    pub ping_everyone: PermissionState,

    pub password_create: PermissionState,
    pub password_update: PermissionState,
    pub password_delete: PermissionState,
    pub password_bypass: PermissionState,

    pub emote_create: PermissionState,
    pub emote_update: PermissionState,
    pub emote_delete: PermissionState,
    pub emote_view: PermissionState,

    pub role_create: PermissionState,
    pub role_delete: PermissionState,
    pub role_update: PermissionState,
    pub role_view: PermissionState,

    pub video_create: PermissionState,
    pub video_delete: PermissionState,
    pub video_watch: PermissionState,
    pub video_move: PermissionState,
    pub video_iframe: PermissionState,
    pub video_raw: PermissionState,

    pub player_pause: PermissionState,
    pub player_resume: PermissionState,
    pub player_rewind: PermissionState,

    pub subtitles_file: PermissionState,
    pub subtitles_embed: PermissionState,

    pub message_create: PermissionState,
    pub message_read: PermissionState,
    pub message_history_read: PermissionState,
    // timeout in seconds between sending messages. default is -1 which means inherited.
    pub message_timeout: i32,

    pub user_kick: PermissionState,
    pub user_ban: PermissionState,
    pub user_unban: PermissionState,
    pub user_timeout: PermissionState,

    pub created_at: NaiveDateTime,
}

impl Role {
    pub fn list_by_room_id(room_id_query: i64, conn: &PgConnection) -> Result<Role, DieselError> {
        use crate::schema::roles::dsl::*;

        roles
            .filter(room_id.eq(room_id_query))
            .first::<Role>(conn)
            .map_err(|err| {
                error!(
                    "Couldn't query role by room_id {:?}: {}",
                    room_id_query, err
                );
                err
            })
            .map_err(From::from)
    }

    pub fn by_id(role_id: i64, conn: &PgConnection) -> Result<Role, DieselError> {
        use crate::schema::roles::dsl::*;

        roles
            .filter(id.eq(role_id))
            .first::<Role>(conn)
            .map_err(|err| {
                error!("Couldn't query role by id {:?}: {}", role_id, err);
                err
            })
            .map_err(From::from)
    }

    pub fn delete(&self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::roles::dsl::*;

        diesel::delete(roles.filter(id.eq(self.id)))
            .execute(conn)
            .map_err(|err| {
                error!("Couldn't delete role {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }

    pub fn update(&self, conn: &PgConnection) -> Result<Role, DieselError> {
        use crate::schema::roles::dsl::*;

        diesel::update(roles)
            .set(self)
            .get_result::<Role>(conn)
            .map_err(|err| {
                error!("Couldn't update role {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(AsChangeset, AsExpression, Insertable, Debug, Associations, Deserialize, Serialize)]
#[table_name = "roles"]
// We only need camelCase for consistent debug output
#[serde(rename_all = "camelCase")]
pub struct NewRole<'a> {
    /// You should always explicitly specify `room_id`, never use default value
    pub room_id: i64,
    pub name: &'a str,
    pub color: Option<&'a str>,
    pub is_default: bool,
    pub position: i32,

    pub title_update: PermissionState,
    pub path_update: PermissionState,
    pub public_update: PermissionState,
    pub room_delete: PermissionState,
    pub room_view: PermissionState,
    pub audit_log_read: PermissionState,
    pub embed_links: PermissionState,
    pub ping_everyone: PermissionState,

    pub password_create: PermissionState,
    pub password_update: PermissionState,
    pub password_delete: PermissionState,
    pub password_bypass: PermissionState,

    pub emote_create: PermissionState,
    pub emote_update: PermissionState,
    pub emote_delete: PermissionState,
    pub emote_view: PermissionState,

    pub role_create: PermissionState,
    pub role_delete: PermissionState,
    pub role_update: PermissionState,
    pub role_view: PermissionState,

    pub video_create: PermissionState,
    pub video_delete: PermissionState,
    pub video_watch: PermissionState,
    pub video_move: PermissionState,
    pub video_iframe: PermissionState,
    pub video_raw: PermissionState,

    pub player_pause: PermissionState,
    pub player_resume: PermissionState,
    pub player_rewind: PermissionState,

    pub subtitles_file: PermissionState,
    pub subtitles_embed: PermissionState,

    pub message_create: PermissionState,
    pub message_read: PermissionState,
    pub message_history_read: PermissionState,
    pub message_timeout: i32,

    pub user_kick: PermissionState,
    pub user_ban: PermissionState,
    pub user_unban: PermissionState,
    pub user_timeout: PermissionState,
}

impl<'a> Default for NewRole<'a> {
    /// `room_id` and `name` should always be specified.
    /// Never use default values!
    fn default() -> NewRole<'a> {
        NewRole {
            room_id: i64::default(),
            name: "",

            color: None,
            is_default: false,
            position: 999,

            title_update: PermissionState::Unset,
            path_update: PermissionState::Unset,
            public_update: PermissionState::Unset,
            room_delete: PermissionState::Unset,
            room_view: PermissionState::Unset,
            audit_log_read: PermissionState::Unset,
            embed_links: PermissionState::Unset,
            ping_everyone: PermissionState::Unset,

            password_create: PermissionState::Unset,
            password_update: PermissionState::Unset,
            password_delete: PermissionState::Unset,
            password_bypass: PermissionState::Unset,

            emote_create: PermissionState::Unset,
            emote_update: PermissionState::Unset,
            emote_delete: PermissionState::Unset,
            emote_view: PermissionState::Unset,

            role_create: PermissionState::Unset,
            role_delete: PermissionState::Unset,
            role_update: PermissionState::Unset,
            role_view: PermissionState::Unset,

            video_create: PermissionState::Unset,
            video_delete: PermissionState::Unset,
            video_watch: PermissionState::Unset,
            video_move: PermissionState::Unset,
            video_iframe: PermissionState::Unset,
            video_raw: PermissionState::Unset,

            player_pause: PermissionState::Unset,
            player_resume: PermissionState::Unset,
            player_rewind: PermissionState::Unset,

            subtitles_file: PermissionState::Unset,
            subtitles_embed: PermissionState::Unset,

            message_create: PermissionState::Unset,
            message_read: PermissionState::Unset,
            message_history_read: PermissionState::Unset,
            message_timeout: -1,

            user_kick: PermissionState::Unset,
            user_ban: PermissionState::Unset,
            user_unban: PermissionState::Unset,
            user_timeout: PermissionState::Unset,
        }
    }
}

impl<'a> NewRole<'a> {
    /// Get Owner role
    pub fn owner(room_id: i64) -> NewRole<'a> {
        NewRole {
            room_id,
            name: "Owner",

            color: Some("#ff9200"),
            is_default: true,
            position: 0,

            title_update: PermissionState::Allowed,
            path_update: PermissionState::Allowed,
            public_update: PermissionState::Allowed,
            room_delete: PermissionState::Allowed,
            room_view: PermissionState::Allowed,
            audit_log_read: PermissionState::Allowed,
            embed_links: PermissionState::Allowed,
            ping_everyone: PermissionState::Allowed,

            password_create: PermissionState::Allowed,
            password_update: PermissionState::Allowed,
            password_delete: PermissionState::Allowed,
            password_bypass: PermissionState::Allowed,

            emote_create: PermissionState::Allowed,
            emote_update: PermissionState::Allowed,
            emote_delete: PermissionState::Allowed,
            emote_view: PermissionState::Allowed,

            role_create: PermissionState::Allowed,
            role_delete: PermissionState::Allowed,
            role_update: PermissionState::Allowed,
            role_view: PermissionState::Allowed,

            video_create: PermissionState::Allowed,
            video_delete: PermissionState::Allowed,
            video_watch: PermissionState::Allowed,
            video_move: PermissionState::Allowed,
            video_iframe: PermissionState::Allowed,
            video_raw: PermissionState::Allowed,

            player_pause: PermissionState::Allowed,
            player_resume: PermissionState::Allowed,
            player_rewind: PermissionState::Allowed,

            subtitles_file: PermissionState::Allowed,
            subtitles_embed: PermissionState::Allowed,

            message_create: PermissionState::Allowed,
            message_read: PermissionState::Allowed,
            message_history_read: PermissionState::Allowed,
            message_timeout: 0,

            user_kick: PermissionState::Allowed,
            user_ban: PermissionState::Allowed,
            user_unban: PermissionState::Allowed,
            user_timeout: PermissionState::Allowed,
        }
    }

    /// Get Administator role.
    /// Just like Owner but can not delete the room
    pub fn administator(room_id: i64) -> NewRole<'a> {
        NewRole {
            room_id,
            name: "Administrator",

            color: Some("#44bd82"),
            is_default: true,
            position: 1,

            room_delete: PermissionState::Unset,

            ..NewRole::owner(room_id)
        }
    }

    /// Get Stranger role. (Someone who is authorized)
    /// Most of rules are inherited.
    pub fn stranger(room_id: i64) -> NewRole<'a> {
        NewRole {
            room_id,
            name: "Stranger",

            color: Some("#d8d8d8"),
            is_default: true,
            position: 1001,

            ping_everyone: PermissionState::Allowed,
            video_create: PermissionState::Allowed,
            message_timeout: 0,

            ..Default::default()
        }
    }

    /// Get Anonymous role. (Someone who is unauthorized)
    /// All rules are inherited.
    pub fn anonymous(room_id: i64) -> NewRole<'a> {
        NewRole {
            room_id,
            name: "Stranger",

            color: Some("#575757"),
            is_default: true,
            position: 1002,

            ..Default::default()
        }
    }

    /// Get Everyone role.
    pub fn everyone(room_id: i64) -> NewRole<'a> {
        NewRole {
            room_id,
            name: "Everyone",

            color: Some("#8e8e8e"),
            is_default: true,
            position: 1003,

            title_update: PermissionState::Forbidden,
            path_update: PermissionState::Forbidden,
            public_update: PermissionState::Forbidden,
            room_delete: PermissionState::Forbidden,
            room_view: PermissionState::Allowed,
            audit_log_read: PermissionState::Forbidden,
            embed_links: PermissionState::Forbidden,
            ping_everyone: PermissionState::Forbidden,

            password_create: PermissionState::Forbidden,
            password_update: PermissionState::Forbidden,
            password_delete: PermissionState::Forbidden,
            password_bypass: PermissionState::Forbidden,

            emote_create: PermissionState::Forbidden,
            emote_update: PermissionState::Forbidden,
            emote_delete: PermissionState::Forbidden,
            emote_view: PermissionState::Allowed,

            role_create: PermissionState::Forbidden,
            role_delete: PermissionState::Forbidden,
            role_update: PermissionState::Forbidden,
            role_view: PermissionState::Allowed,

            video_create: PermissionState::Forbidden,
            video_delete: PermissionState::Forbidden,
            video_watch: PermissionState::Allowed,
            video_move: PermissionState::Forbidden,
            video_iframe: PermissionState::Forbidden,
            video_raw: PermissionState::Forbidden,

            player_pause: PermissionState::Forbidden,
            player_resume: PermissionState::Forbidden,
            player_rewind: PermissionState::Forbidden,

            subtitles_file: PermissionState::Forbidden,
            subtitles_embed: PermissionState::Forbidden,

            message_create: PermissionState::Allowed,
            message_read: PermissionState::Allowed,
            message_history_read: PermissionState::Allowed,
            message_timeout: 1,

            user_kick: PermissionState::Forbidden,
            user_ban: PermissionState::Forbidden,
            user_unban: PermissionState::Forbidden,
            user_timeout: PermissionState::Forbidden,
        }
    }

    pub fn create(&self, conn: &PgConnection) -> Result<Role, DieselError> {
        use crate::schema::roles::dsl::*;

        diesel::insert_into(roles)
            .values(self)
            .get_result::<Role>(conn)
            .map_err(|err| {
                error!("Couldn't create role {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[table_name = "user_roles"]
#[serde(rename_all = "camelCase")]
pub struct UserRole {
    pub id: i64,
    pub role_id: i64,
    pub user_id: i64,
    pub created_at: NaiveDateTime,
}

impl UserRole {
    // TODO: implement
    // pub fn list_by_room_id(
    //     room_id_query: i64,
    //     conn: &PgConnection,
    // ) -> Result<UserRole, DieselError> {
    //     use crate::schema::user_roles::dsl::*;
    //
    // }

    // TODO: implement
    // pub fn list_by_user_id_and_room_id(
    //     room_id_query: i64,
    //     user_id_query: i64,
    //     conn: &PgConnection,
    // ) -> Result<UserRole, DieselError> {
    //     use crate::schema::user_roles::dsl::*;
    //
    // }

    pub fn by_id(user_role_id: i64, conn: &PgConnection) -> Result<UserRole, DieselError> {
        use crate::schema::user_roles::dsl::*;

        user_roles
            .filter(id.eq(user_role_id))
            .first::<UserRole>(conn)
            .map_err(|err| {
                error!("Couldn't query user role by id {:?}: {}", user_role_id, err);
                err
            })
            .map_err(From::from)
    }

    pub fn delete(&self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::user_roles::dsl::*;

        diesel::delete(user_roles.filter(id.eq(self.id)))
            .execute(conn)
            .map_err(|err| {
                error!("Couldn't delete user role {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }

    pub fn update(&self, conn: &PgConnection) -> Result<UserRole, DieselError> {
        use crate::schema::user_roles::dsl::*;

        diesel::update(user_roles)
            .set(self)
            .get_result::<UserRole>(conn)
            .map_err(|err| {
                error!("Couldn't update user role {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(AsChangeset, AsExpression, Insertable, Debug, Associations, Deserialize, Serialize)]
#[table_name = "user_roles"]
pub struct NewUserRole {
    pub role_id: i64,
    pub user_id: i64,
}

impl NewUserRole {
    pub fn create(&self, conn: &PgConnection) -> Result<UserRole, DieselError> {
        use crate::schema::user_roles::dsl::*;

        diesel::insert_into(user_roles)
            .values(self)
            .get_result::<UserRole>(conn)
            .map_err(|err| {
                error!("Couldn't create user role {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

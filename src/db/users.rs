use super::DieselError;
use crate::schema::users;

use crate::diesel::prelude::*;
use crate::diesel::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(x: &bool) -> bool {
    !x
}

#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    #[serde(skip_serializing)]
    pub discord_id: Option<String>,
    pub username: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing)]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    #[serde(skip_serializing)]
    pub file_id: Option<String>,

    #[serde(skip_serializing_if = "is_false")]
    pub is_admin: bool,

    // TODO: implement
    #[serde(skip_serializing)]
    pub last_login: Option<NaiveDateTime>,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn by_id(user_id: String, conn: &PgConnection) -> Result<User, DieselError> {
        use crate::schema::users::dsl::*;

        users
            .filter(id.eq(user_id.clone()))
            .first::<User>(conn)
            .map_err(|err| {
                error!("Couldn't query user by id {:?}: {}", user_id, err);
                err
            })
            .map_err(From::from)
    }
    pub fn by_discord_id(
        discord_id_query: String,
        conn: &PgConnection,
    ) -> Result<User, DieselError> {
        use crate::schema::users::dsl::*;

        users
            .filter(discord_id.eq(discord_id_query.clone()))
            .first::<User>(conn)
            .map_err(|err| {
                error!(
                    "Couldn't query user by discord id {:?}: {}",
                    discord_id_query, err
                );
                err
            })
            .map_err(From::from)
    }

    pub fn by_name(name: String, conn: &PgConnection) -> Result<User, DieselError> {
        use crate::schema::users::dsl::*;

        users
            .filter(username.eq(name.clone()))
            .first::<User>(conn)
            .map_err(|err| {
                error!("Couldn't query user by name {:?}: {}", name, err);
                err
            })
            .map_err(From::from)
    }

    pub fn delete(&self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.filter(id.eq(self.id.to_owned())))
            .execute(conn)
            .map_err(|err| {
                error!("Couldn't remove user {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }

    pub fn update(&self, conn: &PgConnection) -> Result<User, DieselError> {
        use crate::schema::users::dsl::*;

        diesel::update(users)
            .set(self)
            .get_result::<User>(conn)
            .map_err(|err| {
                error!("Couldn't update user {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(AsChangeset, AsExpression, Insertable, Debug, Associations, Deserialize, Serialize)]
#[table_name = "users"]
// We only need camelCase for consistent debug output
#[serde(rename_all = "camelCase")]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub discord_id: Option<String>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub color: Option<String>,
    pub file_id: Option<String>,
}

impl<'a> Default for NewUser<'a> {
    fn default() -> Self {
        Self {
            username: "",
            discord_id: None,
            nickname: None,
            email: None,
            password: None,
            color: None,
            file_id: None,
        }
    }
}

// TODO: Remove later
// type AllColumns = (users::id, users::username, users::nickname);
// pub const ALL_COLUMNS: AllColumns = (users::id, users::username, users::nickname);
// type All = diesel::dsl::Select<users::table, AllColumns>;
// type WithName<'a> = diesel::dsl::Eq<users::username, &'a str>;
// type ByName<'a> = diesel::dsl::Filter<All, WithName<'a>>;

impl<'a> NewUser<'a> {
    pub fn create(&self, conn: &PgConnection) -> Result<User, DieselError> {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users)
            .values(self)
            .get_result::<User>(conn)
            .map_err(|err| {
                error!("Couldn't create user {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

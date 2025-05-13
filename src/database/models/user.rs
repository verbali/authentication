use crate::{database::database, helpers::hash};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: i32,
    uuid: String,
    email: String,
    password: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
struct NewUser<'a> {
    uuid: &'a str,
    email: &'a str,
    password: &'a str,
}

#[derive(Debug)]
pub struct InsertableUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicUser {
    pub uuid: String,
    pub email: String,
}

impl User {
    pub fn create(user: InsertableUser) -> Result<PublicUser, diesel::result::Error> {
        use crate::schema::users;

        let connection = &mut database::establish_connection();
        let uuid = Uuid::new_v4();
        let hash = hash::encode(user.password);
        let new_user = NewUser {
            uuid: &uuid.to_string(),
            email: &user.email,
            password: &hash.to_string(),
        };

        match diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(connection)
        {
            Ok(user) => Ok(PublicUser {
                uuid: user.uuid,
                email: user.email,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn find(email: String, password: String) -> Result<PublicUser, diesel::result::Error> {
        use crate::schema::users;

        let connection = &mut database::establish_connection();
        let hashed_password = hash::encode(password);

        match users::dsl::users
            .filter(users::dsl::email.eq(email))
            .filter(users::dsl::password.eq(hashed_password))
            .limit(1)
            .select(User::as_select())
            .load(connection)
        {
            Ok(mut results) => match results.pop() {
                Some(user) => Ok(PublicUser {
                    uuid: user.uuid,
                    email: user.email,
                }),
                None => Err(diesel::result::Error::NotFound),
            },
            Err(err) => Err(err),
        }
    }
}

use crate::{database::database, helpers::hash};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
struct NewUser<'a> {
    uuid: &'a str,
    email: &'a str,
    password: &'a str,
}

pub struct InsertableUser {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn create(user: InsertableUser) -> Result<User, diesel::result::Error> {
        use crate::schema::users;

        let connection = &mut database::establish_connection();
        let uuid = Uuid::new_v4();
        let hash = hash::encode(user.password);
        let new_user = NewUser {
            uuid: &uuid.to_string(),
            email: &user.email,
            password: &hash.to_string(),
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(connection)
    }

    pub fn find(email: String, password: String) -> Result<User, diesel::result::Error> {
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
                Some(user) => Ok(user),
                None => Err(diesel::result::Error::NotFound),
            },
            Err(err) => Err(err),
        }
    }
}

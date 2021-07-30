use super::Model;
use diesel::prelude::*;

use crate::schema::users;

#[derive(Queryable, Debug)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub token: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct MutUser {
    pub username: String,
    pub password: String
}

impl Model<User, MutUser, String> for User {
    fn create(new: &MutUser, db: &diesel::MysqlConnection) -> Result<usize, diesel::result::Error> {
        unimplemented!()
    }

    fn read_one(id: String, db: &diesel::MysqlConnection) -> Result<User, diesel::result::Error> {
        use users::dsl::username;

        Ok(users::table.filter(username.eq(id)).first::<User>(db)?)
    }

    fn read_all(db: &diesel::MysqlConnection) -> Result<Vec<User>, diesel::result::Error> {
        unimplemented!()
    }

    fn update(
        id: String,
        new: &MutUser,
        db: &diesel::MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        todo!()
    }

    fn delete(id: String, db: &diesel::MysqlConnection) -> Result<usize, diesel::result::Error> {
        todo!()
    }
}
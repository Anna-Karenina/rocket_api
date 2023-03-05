use crate::models::rustacean::{NewRustacean, Rustacean};
use crate::schema::rustaceans;
use diesel::prelude::*;
use diesel::{QueryResult, SqliteConnection};
pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find(connection: &SqliteConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table
            .find(&id)
            .get_result::<Rustacean>(connection)
    }

    pub fn find_multiple(connection: &SqliteConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).load::<Rustacean>(connection)
    }

    pub fn create(
        connection: &SqliteConnection,
        new_rustacean: NewRustacean,
    ) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .execute(connection)?;

        let last_id = Self::last_inserted_id(connection)?;
        Self::find(connection, last_id)
    }

    pub fn update(
        connection: &SqliteConnection,
        id: i32,
        rustacean: Rustacean,
    ) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::email.eq(rustacean.email.to_owned()),
                rustaceans::name.eq(rustacean.name.to_owned()),
            )) //tuple syntax
            .execute(connection)?;

        let last_id = Self::last_inserted_id(connection)?;
        Self::find(connection, last_id)
    }

    pub fn delete(connection: &SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(connection)
    }

    fn last_inserted_id(connection: &SqliteConnection) -> QueryResult<i32> {
        //crutch - find better solution if do many request its return wrong id, its sqlite cant return last updated row
        rustaceans::table
            .select(rustaceans::id)
            .order(rustaceans::id.desc())
            .first(connection)
    }
}

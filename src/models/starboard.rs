use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    AsChangeset, Insertable, PgConnection, Queryable, Selectable,
};

use crate::{schema::starboard, Error};

#[derive(AsChangeset, Insertable, Queryable, Selectable)]
#[diesel(table_name = starboard)]
pub struct Starboard {
    pub message_id: i64,
    pub starboard_id: Option<i64>,
    pub stars: i32,
}

impl Starboard {
    /// Deletes an entry from the database.
    pub fn delete_one(
        connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        id: i64,
    ) -> Result<(), Error> {
        use starboard::dsl;

        diesel::delete(dsl::starboard.filter(dsl::message_id.eq(id)))
            .execute(connection)
            .expect("Failed deleting starboard entry.");

        Ok(())
    }

    /// Searches a specific registered starboard.
    pub fn find_one(
        connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        message_id: Option<i64>,
        starboard_id: Option<i64>,
    ) -> Result<Starboard, diesel::result::Error> {
        use starboard::dsl;

        let mut query = dsl::starboard.into_boxed();

        if let Some(id) = message_id {
            query = query.filter(dsl::message_id.eq(id))
        }

        if let Some(id) = starboard_id {
            query = query.filter(dsl::starboard_id.eq(id))
        }

        query.limit(1).get_result::<Starboard>(connection)
    }

    /// Upserts a `Starboard` into the database.
    pub fn upsert(
        connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        row: Starboard,
    ) -> Result<(), Error> {
        use starboard::dsl;

        connection
            .build_transaction()
            .run::<_, diesel::result::Error, _>(|connection| {
                diesel::insert_into(starboard::table)
                    .values(&row)
                    .on_conflict(dsl::message_id)
                    .filter_target(dsl::message_id.eq(row.message_id))
                    .do_update()
                    .set(&row)
                    .execute(connection)?;

                Ok(())
            })?;

        Ok(())
    }
}

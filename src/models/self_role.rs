use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    AsChangeset, Insertable, PgConnection, Queryable, Selectable,
};

use crate::{schema::self_role, Error};

#[derive(Insertable, AsChangeset, Queryable, Selectable)]
#[diesel(table_name = self_role)]
pub struct SelfRole {
    pub id: i64,
    pub style: i32,
    pub emoji: Option<String>,
}

impl SelfRole {
    /// Searches a specific registered role.
    pub fn find_one(
        connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        id: i64,
    ) -> Result<SelfRole, diesel::result::Error> {
        use self_role::dsl;

        dsl::self_role
            .filter(dsl::id.eq(id))
            .limit(1)
            .get_result::<SelfRole>(connection)
    }

    /// Gets all the roles registered.
    pub fn get_all(
        connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<Vec<SelfRole>, diesel::result::Error> {
        self_role::dsl::self_role.get_results(connection)
    }

    /// Upserts a `SelfRole` into the database.
    pub fn upsert(
        connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        row: SelfRole,
    ) -> Result<(), Error> {
        use self_role::dsl;

        connection
            .build_transaction()
            .run::<_, diesel::result::Error, _>(|connection| {
                diesel::insert_into(self_role::table)
                    .values(&row)
                    .on_conflict(dsl::id)
                    .filter_target(dsl::id.eq(row.id))
                    .do_update()
                    .set(&row)
                    .execute(connection)?;

                Ok(())
            })?;

        Ok(())
    }

    /// Removes a role by id, returning whether the role was actually deleted, otherwise it wasn't there to begin with
    pub fn remove(
        connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        id: i64,
    ) -> Result<bool, diesel::result::Error> {
        diesel::delete(self_role::table.filter(self_role::id.eq(id)))
            .execute(connection)
            .map(|count| count > 0)
    }

    /// Removes multiple roles by id
    pub fn remove_all(
        connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        ids: &Vec<i64>,
    ) -> Result<(), diesel::result::Error> {
        diesel::delete(self_role::table.filter(self_role::id.eq_any(ids)))
            .execute(connection)
            .map(|_| ())
    }
}

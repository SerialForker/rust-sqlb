////////////////////////////////////
// sqlx-exec - module for the sqlx query executor
////

use sqlx::{
	postgres::PgArguments,
	query::{Query, QueryAs},
	FromRow, Pool, Postgres,
};

use crate::{SqlBuilder, Val};

/// Build a sqlx::query_as for the E (Entity) generic type, binds the values, and does a .fetch_one and returns E
pub async fn fetch_as_one<'q, E, Q>(db_pool: &Pool<sqlx::Postgres>, sb: &Q) -> Result<E, sqlx::Error>
where
	E: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	Q: SqlBuilder,
{
	let sql = sb.sql();
	let vals = sb.vals();
	let query = sqlx::query_as::<sqlx::Postgres, E>(&sql);
	let query = sqlx_bind_vals(query, vals);
	let r = query.fetch_one(db_pool).await?;
	Ok(r)
}

/// Build a sqlx::query_as for the E (Entity) generic type, binds the values, and does a .fetch_all and returns Vec<E>
pub async fn fetch_as_all<'q, E, Q>(db_pool: &Pool<sqlx::Postgres>, sb: &Q) -> Result<Vec<E>, sqlx::Error>
where
	E: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	Q: SqlBuilder,
{
	let sql = sb.sql();
	let vals = sb.vals();
	let query = sqlx::query_as::<sqlx::Postgres, E>(&sql);
	let query = sqlx_bind_vals(query, vals);
	let r = query.fetch_all(db_pool).await?;
	Ok(r)
}

/// Build a sqlx::query, binds the values, and call .execute and return rows_affected
pub async fn exec<'q, Q>(db_pool: &Pool<sqlx::Postgres>, sb: &Q) -> Result<u64, sqlx::Error>
where
	Q: SqlBuilder,
{
	let sql = sb.sql();
	let vals = sb.vals();
	let query = sqlx::query::<sqlx::Postgres>(&sql);
	let query = sqlx_query_bind_vals(query, vals);
	let r = query.execute(db_pool).await?.rows_affected();

	Ok(r)
}

pub fn sqlx_query_bind_vals<'q>(mut query: Query<'q, Postgres, PgArguments>, vals: Vec<Val>) -> Query<'q, sqlx::Postgres, PgArguments> {
	for val in vals.into_iter() {
		// NOTE: for now, needs to duplicate with code below because .bind is not a trait.
		// TODO: Define/impl a custom trait for both Query and QueryAs to expose common bind
		match val {
			Val::BOOL(val) => query = query.bind(val),
			Val::STRING(val) => query = query.bind(val),
			Val::U32(val) => query = query.bind(val),
			Val::I32(val) => query = query.bind(val),
			Val::I64(val) => query = query.bind(val),
		};
	}
	query
}

pub fn sqlx_bind_vals<'q, E>(mut query: QueryAs<'q, Postgres, E, PgArguments>, vals: Vec<Val>) -> QueryAs<'q, sqlx::Postgres, E, PgArguments> {
	for val in vals.into_iter() {
		match val {
			Val::BOOL(val) => query = query.bind(val),
			Val::STRING(val) => query = query.bind(val),
			Val::U32(val) => query = query.bind(val),
			Val::I32(val) => query = query.bind(val),
			Val::I64(val) => query = query.bind(val),
		};
	}
	query
}

use std::error::Error;

use sqlb::{sqlx_exec, Field, GetFields};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

// region:    Test Types
#[derive(sqlx::FromRow)]
pub struct Todo {
	pub id: i64,
	pub title: String,
}

pub struct TodoPatch {
	pub title: Option<String>,
}

impl GetFields for TodoPatch {
	fn fields(&self) -> Vec<Field> {
		let mut fields = Vec::new();
		if let Some(title) = &self.title {
			fields.push(("title", title).into());
		}
		fields
	}
}
// endregion: Test Types

// region:    Test Seed Utils
#[allow(unused)] // Note: Since not used in all test/ files, remove warning
pub async fn util_insert_todo(title: &str, db_pool: &Pool<Postgres>) -> Result<i64, Box<dyn Error>> {
	let patch_data = TodoPatch {
		title: Some(title.to_string()),
	};
	let sb = sqlb::insert("todo").data(patch_data.fields());
	let sb = sb.returning(&["id"]);
	let (id,) = sqlx_exec::fetch_as_one::<(i64,), _>(db_pool, &sb).await?;
	Ok(id)
}

#[allow(unused)] // Note: Since not used in all test/ files, remove warning
pub async fn util_fetch_all_todos(db_pool: &Pool<Postgres>) -> Result<Vec<Todo>, Box<dyn Error>> {
	let sb = sqlb::select("todo").columns(&["id", "title"]).order_by("!id");
	let todos: Vec<Todo> = sqlx_exec::fetch_as_all(&db_pool, &sb).await?;
	Ok(todos)
}

#[allow(unused)] // Note: Since not used in all test/ files, remove warning
pub async fn util_fetch_todo(db_pool: &Pool<Postgres>, id: i64) -> Result<Todo, Box<dyn Error>> {
	let sb = sqlb::select("todo").columns(&["id", "title"]).and_where(&[("id", "=", id.into())]);
	let todos: Todo = sqlx_exec::fetch_as_one(&db_pool, &sb).await?;
	Ok(todos)
}
// endregion: Test Seed Utils

// region:    Test Utils
pub async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
	let pool = PgPoolOptions::new()
		.max_connections(5)
		.connect("postgres://postgres:welcome@localhost/postgres")
		.await?;

	// Create todo table
	sqlx::query("DROP TABLE IF EXISTS todo").execute(&pool).await?;
	sqlx::query(
		r#"
CREATE TABLE IF NOT EXISTS todo (
  id bigserial,
  title text
);"#,
	)
	.execute(&pool)
	.await?;

	// Create project table
	sqlx::query("DROP TABLE IF EXISTS projects").execute(&pool).await?;
	sqlx::query(
		r#"
CREATE TABLE IF NOT EXISTS project (
  id bigserial,
  name text
);"#,
	)
	.execute(&pool)
	.await?;

	Ok(pool)
}

// endregion: Test Utils
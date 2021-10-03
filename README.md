
**IMPORTANT - 0.0.x versions will be experimental and probably break APIs in each release.**

**sqlb** is (will be) be a simple, expressive, and progressive SQLBuilder for Rust.

- **Simple** - Focused on providing an expressive, composable, and reasonnably typed scheme to build and execute (via sqlx for now) parameterized SQL statements. The goal is NOT to abstract SQL but to make it expressive and composable using Rust programmatic constructs.
	- **NOT** a database **executor/driver** (SQLX and tokio-postgres to name a few are excellent)
	- **NOT** an **ORM**, although eventually, one could build an ORM on top of it. 
- **Progressive** - From arbitrary typed data in and out (list of names/values) to struct and mapping rules. 
- **Focused** 
	- **[sqlx](https://crates.io/crates/sqlx)** - The first "database executor" provided will be [sqlx](https://github.com/launchbadge/sqlx). 
	- **PostgreSQL** - First database support will be Postgres (via sqlx). Depending on interest and pull requests, other database support might be added.  
	- **[tokio-postgres](https://docs.rs/tokio-postgres/0.7.2/tokio_postgres/)** might be part of a future plan as well, as it provides some interesting concurrency benefits. 
	

> NOTE: SQL Builders are typically not used directly by application business logic, but rather to be wrapped in some Application Data Access Layer (e.g., DAOs, MAOs - Model Access Object -). In fact, even when using ORMs, it is often a good code design to wrap those access via some data access layers. Given this approach, having a more flexible but still typed database access layer gives the "model access layer" more expressiveness and power. 


Goals for first **0.1.x** releases: 

- **sqlx** - Will probably be SQLX centric. 
- **PostgreSQL** - Will probably support SQLX only. Contributions for another database (via sqlx) welcome
- **Macros** - to keep thing DRY (but they are optional, all can be implemented via trait objects)


## Early API Example (just conceptual for now)

```rust
#[derive(sqlx::FromRow)] // Optional: to be able to use the sqlx_exec::fetch_as...
pub struct Todo {
    pub id: i64,
    pub title: String,
}

#[derive(sqlb::Fields)] // implements sqlb::HasFields for dynamic binding
pub struct TodoPatch {
    pub title: Option<String>,
}

let patch_data = TodoPatch {
	title: Some("Hello Title".to_string())
};

// INSERT - Insert a new Todo from a Partial todo
let sb = sqlb::insert("todo").data(patch_data.fields());
let sb = sb.returning(&["id", "title"]);
let (_id, title) = sqlb::sqlx_exec::fetch_as_one::<(i64, String), _>(&db_pool, &sb).await?;

// SELECT - Get all todos
let sb = sqlb::select("todo").columns(&["id", "title"]).order_by("!id");
let todos: Vec<Todo> = sqlb::sqlx_exec::fetch_as_all(&db_pool, &sb).await?;
assert_eq!(1, todos.len());
```



## For sqlb Dev

Start a PostgreSQL

```sh
# In terminal 1 - start postges
docker run --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=welcome  postgres:14

# In terminal 2 - (optional) launch psql on the Postgres instance above
docker exec -it -u postgres pg psql

# In terminal 3 - MUST run with `--test-threads=1` to avoid database access conflicts
cargo test -- --test-threads=1

# or watch a particular test target
cargo watch -q -c -x 'test --test test_sb_insert -- --test-threads=1'
```
use std::sync::Arc;

use actix_web::{web, Error, HttpResponse};
use futures::future::Future;

use juniper::http::playground::playground_source;
use juniper::{http::GraphQLRequest, Executor, FieldResult};
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("src/schema.graphql");

pub struct Context {}

impl juniper::Context for Context {}

pub struct Query;
pub struct Mutation;

impl QueryFields for Query {
  fn field_ping(&self, _executor: &Executor<'_, Context>) -> FieldResult<String> {
    Ok("pong".to_string())
  }
}

impl MutationFields for Mutation {
  fn field_foo(&self, _executor: &Executor<'_, Context>, input: FooInput) -> FieldResult<String> {
    println!(">>> {:?}", input);
    Ok(format!("Name: input.bar == {}", input.bar).to_string())
  }
}

fn playground() -> HttpResponse {
  let html = playground_source("");
  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}

fn graphql(
  schema: web::Data<Arc<Schema>>,
  data: web::Json<GraphQLRequest>,
  // db_pool: web::Data<DbPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
  let ctx = Context {};
  // let ctx = Context {
  //   db_con: db_pool.get().unwrap(),
  // };

  web::block(move || {
    let res = data.execute(&schema, &ctx);
    Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
  })
  .map_err(Error::from)
  .and_then(|user| {
    Ok(
      HttpResponse::Ok()
        .content_type("application/json")
        .body(user),
    )
  })
}

pub fn register(config: &mut web::ServiceConfig) {
  let schema = std::sync::Arc::new(Schema::new(Query, Mutation));

  config
    .data(schema)
    .route("/graphql", web::post().to_async(graphql))
    .route("/graphql", web::get().to(playground));
}

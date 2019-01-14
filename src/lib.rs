// #[macro_use] extern crate juniper;
#[macro_use] extern crate juniper;
#[macro_use] extern crate juniper_from_schema;

use juniper::*;
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("src/schema.graphql");

// pub struct Query;

// impl QueryFields for Query {
//     fn field_search(
//         &self,
//         executor: &Executor<'_, Context>,
//         trail: &QueryTrail<'_, SearchResult, Walked>,
//         query: String,
//     ) -> FieldResult<Vec<SearchResult>> {
//         let article: Article = Article { id: Id::new("1"), text: "Business".to_string() };
//         let tweet: Tweet = Tweet { id: Id::new("2"), text: "1 weird tip".to_string() };

//         let posts = vec![
//             SearchResult::from(article),
//             SearchResult::from(tweet),
//         ];

//         Ok(posts)
//     }
// }



// impl QueryFields for Query {
//     fn field_hello_world(
//         &self,
//         executor: &Executor<'_, Context>,
//         name: String,
//     ) -> FieldResult<String> {
//         Ok(format!("Hello, {}!", name))
//     }
// }

// pub mod protocol;

// #[derive(GraphQLObject)]
// #[graphql(description="A resource and action pair")]
// pub struct ResourceAction {
//     #[graphql(description="Defining authority identifier")]
//     authority: String,
//     #[graphql(description="Resource path")]
//     resource: Vec<String>,
//     #[graphql(description="Action name")]
//     action: String,
// }

// pub trait Service {
//     fn foo(&self) -> u32;
// }

// type Context = Box<Service>;

// impl juniper::Context for Context {}

// struct Query {}

// graphql_object!(Query: Context |&self| {
//     field permit(&exec, subjects: Vec<String>, targets: Vec<ResourceAction>) -> FieldResult<bool> {
//         false
//     }
// });

// // #[derive(GraphQLObject)]
// // #[graphql(description="An authorization query element")]
// // pub struct Subject {
// //     id: String,
// // }

// //     field apiVersion() -> &str {
// //         "1.0"
// //     }

// //     // Arguments to resolvers can either be simple types or input objects.
// //     // The executor is a special (optional) argument that allows accessing the context.
// //     field human(&executor, id: String) -> FieldResult<Human> {
// //         // Get the context from the executor.
// //         let context = executor.context();
// //         // Get a db connection.
// //         let connection = context.pool.get_connection()?;
// //         // Execute a db query.
// //         // Note the use of `?` to propagate errors.
// //         let human = connection.find_human(&id)?;
// //         // Return the result.
// //         Ok(human)
// //     }

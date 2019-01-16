extern crate juniper;
use juniper::*;
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file! ("src/schema.graphql");

pub struct Context;
impl juniper::Context for Context {}

pub struct Query;
impl QueryFields for Query {
    fn field_can_perform(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, QueryResult, Walked>,
        _subject: Vec<String>,
        _resource_actions: Vec<ResourceActionInput>,
    ) -> FieldResult<&QueryResult> {
        unimplemented!()
    }
}

pub struct QueryResult;
impl QueryResultFields for QueryResult {
    fn field_all(
        &self,
        _executor: &Executor<'_,Context>
    ) -> FieldResult<&bool> {
        unimplemented!()
    }
    fn field_any(
        &self,
        _executor: &Executor<'_,Context>
    ) -> FieldResult<&bool> {
        unimplemented!()
    }
    fn field_allow(
        &self,
        _executor: &Executor<'_,Context>,
        _trail: &QueryTrail<'_, ResourceAction, Walked>
    ) -> FieldResult<&Vec<ResourceAction>> {
        unimplemented!()
    }
    fn field_deny(
        &self,
        _executor: &Executor<'_,Context>,
        _trail: &QueryTrail<'_, ResourceAction, Walked>
    ) -> FieldResult<&Vec<ResourceAction>> {
        unimplemented!()
    }

}

pub struct ResourceAction;
impl ResourceActionFields for ResourceAction {
    fn field_namespace(
        &self,
        _executor: &Executor<'_,Context>
    ) -> FieldResult<&String> {
        unimplemented!()
    }
    fn field_action(
        &self,
        _executor: &Executor<'_,Context>
    ) -> FieldResult<&String> {
        unimplemented!()
    }
        fn field_resource(
        &self,
        _executor: &Executor<'_,Context>
    ) -> FieldResult<&Vec<String>> {
        unimplemented!()
    }
}

pub type Schema2 = juniper::RootNode<'static, Query, juniper::EmptyMutation<Context>>;

pub fn wot() {
    let ctx = Context;
    // get the schema
    let (res, _errors) = juniper::execute(
        "{__schema {types {name description fields {name description}}}}",
        // "{ human (id: \"3\") {id name} }",
        None,
        &Schema2::new(Query, EmptyMutation::new()),
        &Variables::new(),
        &ctx,
    ).unwrap();
    let s = serde_json::to_string(&res).unwrap();
    println!("schema: {}",s);

}

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
    ) -> FieldResult<QueryResult> {
        Ok(QueryResult {
            all: false,
            any: true,
            allow: vec![ResourceAction{namespace: "hi".to_string(), action: "there".to_string(), resource: vec!["thing".to_string()]}],
            deny: vec![],
        })
    }
}


pub struct QueryResult {
    all: bool,
    any: bool,
    allow: Vec<ResourceAction>,
    deny: Vec<ResourceAction>,
}
impl QueryResultFields for QueryResult {
    fn field_all(
        &self,
        _executor: &Executor<'_,Context>
    ) -> FieldResult<bool> {
        Ok(self.all)
    }
    fn field_any(
        &self,
        _executor: &Executor<'_,Context>
    ) -> FieldResult<bool> {
        Ok(self.any)
    }
    fn field_allow(
        &self,
        _executor: &Executor<'_,Context>,
        _trail: &QueryTrail<'_, ResourceAction, Walked>
    ) -> FieldResult<Vec<ResourceAction>> {
        Ok(self.allow.clone())
    }
    fn field_deny(
        &self,
        _executor: &Executor<'_,Context>,
        _trail: &QueryTrail<'_, ResourceAction, Walked>
    ) -> FieldResult<Vec<ResourceAction>> {
        Ok(self.deny.clone())
    }

}

#[derive(Clone)]
pub struct ResourceAction {
    namespace: String,
    action: String,
    resource: Vec<String>,
    }

impl ResourceActionFields for ResourceAction {
    fn field_namespace(
        &self,
        _executor: &Executor<'_,Context>
    ) -> FieldResult<String> {
        Ok(self.namespace.clone())
    }
    fn field_action(
        &self,
        _executor: &Executor<'_,Context>
    ) -> FieldResult<String> {
        Ok(self.action.clone())
    }
        fn field_resource(
        &self,
        _executor: &Executor<'_,Context>
    ) -> FieldResult<Vec<String>> {
        Ok(self.resource.clone())
    }
}

pub type Schema2 = juniper::RootNode<'static, Query, juniper::EmptyMutation<Context>>;

pub fn execute(query: &str) {
    let ctx = Context;
    // get the schema
    let (res, _errors) = juniper::execute(query,
        None,
        &Schema2::new(Query, EmptyMutation::new()),
        &Variables::new(),
        &ctx,
    ).unwrap();
    // let s = serde_json::to_string(&res).unwrap();
    println!("schema: {:?}",res);
}


pub type XRet = Value<DefaultScalarValue>;
pub fn execute_r(query: &str) -> XRet {
    let ctx = Context;
    // get the schema
    let (res, _errors) = juniper::execute(query,
        None,
        &Schema2::new(Query, EmptyMutation::new()),
        &Variables::new(),
        &ctx,
    ).unwrap();
    println!("schema: {:?}",res);
    res
}


pub fn wot2() {
    let ctx = Context;
    // get the schema
    let (res, _errors) = juniper::execute(
        "{
            canPerform (
                subject: [\"sally\",\"tom\"]
                resourceActions: [{
                    namespace: \"tracking\"
                    action: \"supertrack\"
                    resource: [\"1930942\"]
                    }]
            ) {
                all
                any
                allow {
                    namespace
                    action
                    resource }
                deny {
                    namespace
                    action
                    resource }
                }
        }",
        None,
        &Schema2::new(Query, EmptyMutation::new()),
        &Variables::new(),
        &ctx,
    ).unwrap();
    let s = serde_json::to_string(&res).unwrap();
    println!("schema: {}",s);

}

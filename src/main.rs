#[macro_use] extern crate juniper;
extern crate juniper_iron;
extern crate iron;
extern crate mount;

use mount::Mount;
use iron::prelude::*;
use juniper::EmptyMutation;
use juniper_iron::GraphQLHandler;

fn context_factory(_: &mut Request) -> () {
    ()
}

struct RootQuery;

graphql_object!(RootQuery: () |&self| {
    field bar() -> juniper::FieldResult<String> {
        Ok("Bar".to_owned())
    }
});

struct RootMutation;

graphql_object!(RootMutation: () |&self| {
});

pub type Schema = juniper::RootNode<'static, RootQuery, RootMutation>;

fn main() {
    let mut mount = Mount::new();

    let graphql_endpoint = GraphQLHandler::new(
        context_factory,
        RootQuery,
        RootMutation,
    );

    mount.mount("/graphql", graphql_endpoint);

    let chain = Chain::new(mount);

    Iron::new(chain).http("0.0.0.0:8080").unwrap();
}
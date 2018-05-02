#[macro_use] extern crate juniper;
extern crate juniper_iron;
extern crate iron;
extern crate mount;

use mount::Mount;
use iron::prelude::*;
use juniper::EmptyMutation;
use juniper_iron::GraphQLHandler;

#[derive(GraphQLObject)]
struct Foo {
    value: i32
}

#[derive(GraphQLInputObject)]
struct Bar {
    value: i32
}

fn context_factory(_: &mut Request) -> () {
    ()
}

struct RootQuery;

graphql_object!(RootQuery: () |&self| {
    field foo() -> Foo {
        Foo { value: 0 }
    }

    field bar() -> Bar {
        Bar { value: 0 }
    }
});

fn main() {
    let mut mount = Mount::new();

    let graphql_endpoint = GraphQLHandler::new(
        context_factory,
        RootQuery,
        EmptyMutation::<()>::new(),
    );

    mount.mount("/graphql", graphql_endpoint);

    let chain = Chain::new(mount);

    Iron::new(chain).http("0.0.0.0:8080").unwrap();
}

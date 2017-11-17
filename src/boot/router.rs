use gotham::router::Router;
use gotham::router::tree::TreeBuilder;
use gotham::router::route::dispatch::{new_pipeline_set, finalize_pipeline_set};
use gotham::router::response::finalizer::ResponseFinalizerBuilder;

pub fn router() -> Router {
    // create the pipeline set builder
    let pipeline_set_builder = new_pipeline_set();

    // TODO: add pipelines to the pipeline set here

    // finalize the pipeline set
    let pipeline_set =finalize_pipeline_set(pipeline_set_builder);

    // create the route tree builder
    let mut route_tree_builder = TreeBuilder::new();

    // TODO: add routes to the route tree here

    // finalize the route tree
    let mut route_tree = route_tree_builder.finalize();

    // create the response finalizer builder
    let response_finalizer_builder = ResponseFinalizerBuilder::new();

    // add response finalizers here...

    // finalize the response finalizer
    let response_finalizer = response_finalizer_builder.finalize();

    //create the router
    Router::new(route_tree, response_finalizer)
}
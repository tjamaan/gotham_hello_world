use gotham::router::Router;
use gotham::router::tree::TreeBuilder;
use gotham::router::tree::node::{NodeBuilder, SegmentType};
use gotham::router::route::dispatch::{new_pipeline_set, finalize_pipeline_set};
use gotham::router::response::finalizer::ResponseFinalizerBuilder;
use gotham::router::route::Route;
use gotham::router::route::matcher::MethodOnlyRouteMatcher;
use gotham::router::route::dispatch::{PipelineSet, PipelineHandleChain};
use gotham::router::route::Extractors;
use gotham::router::request::path::NoopPathExtractor;
use gotham::router::request::query_string::NoopQueryStringExtractor;
use gotham::router::route::RouteImpl;
use gotham::router::route::Delegation;
use gotham::router::route::dispatch::DispatcherImpl;
use hyper;

use controllers;

fn create_welcome_route<C, P>(active_pipelines: C, pipeline_set: PipelineSet<P>) -> Box<Route + Send + Sync>
where
    C: PipelineHandleChain<P> + Send + Sync + 'static,
    P: Send + Sync + 'static
{
    // create a matcher that matches only HTTP GET requests
    let matcher = MethodOnlyRouteMatcher::new(vec![hyper::Method::Get]);

    // create a dispatcher that will use the handler `controllers::welcome` after going through the active_pipeline
    let dispatcher = DispatcherImpl::new(
        || Ok(controllers::welcome::welcome),
        active_pipelines,
        pipeline_set);

    // create the extractors that will:
    // * extract the path of the request (stuff after `/welcome`)
    // * extract the query string (stuff after `?`)
    // for now we will use extractors that do nothing (NoOp = No Operation)
    let extractors: Extractors<NoopPathExtractor, NoopQueryStringExtractor> = Extractors::new();

    // create the actual route using the default route implementation (RouteImpl)
    let route = RouteImpl::new(
        matcher,
        Box::new(dispatcher), // wrap the dispatcher in a box (i.e. put it behind a pointer)
        extractors,
        Delegation::Internal // we are handling this request in the same Router, not an external router
    );

    // wrap the route in a box (i.e. put it behind a pointer) and return it
    Box::new(route)
}

fn create_capitalize_route<C, P>(active_pipelines: C, pipeline_set: PipelineSet<P>) -> Box<Route + Send + Sync>
    where
        C: PipelineHandleChain<P> + Send + Sync + 'static,
        P: Send + Sync + 'static
{
    // create a matcher that matches only HTTP GET requests
    let matcher = MethodOnlyRouteMatcher::new(vec![hyper::Method::Get]);

    // create a dispatcher that will use the handler `controllers::welcome` after going through the active_pipeline
    let dispatcher = DispatcherImpl::new(
        || Ok(controllers::capitalize::capitalize),
        active_pipelines,
        pipeline_set);

    // create the extractors that will:
    // * extract the path of the request (stuff after `/welcome`)
    // * extract the query string (stuff after `?`)
    let extractors: Extractors<NoopPathExtractor, controllers::capitalize::CapitalizeQueryStringExtractor> = Extractors::new();

    // create the actual route using the default route implementation (RouteImpl)
    let route = RouteImpl::new(
        matcher,
        Box::new(dispatcher), // wrap the dispatcher in a box (i.e. put it behind a pointer)
        extractors,
        Delegation::Internal // we are handling this request in the same Router, not an external router
    );

    // wrap the route in a box (i.e. put it behind a pointer) and return it
    Box::new(route)
}

fn create_cube_route<C, P>(active_pipelines: C, pipeline_set: PipelineSet<P>) -> Box<Route + Send + Sync>
    where
        C: PipelineHandleChain<P> + Send + Sync + 'static,
        P: Send + Sync + 'static
{
    // create a matcher that matches only HTTP GET requests
    let matcher = MethodOnlyRouteMatcher::new(vec![hyper::Method::Get]);

    // create a dispatcher that will use the handler `controllers::welcome` after going through the active_pipeline
    let dispatcher = DispatcherImpl::new(
        || Ok(controllers::cube::cube),
        active_pipelines,
        pipeline_set);

    // create the extractors that will:
    // * extract the path of the request (stuff after `/welcome`)
    // * extract the query string (stuff after `?`)
    let extractors: Extractors<controllers::cube::CubePathExtractor, NoopQueryStringExtractor> = Extractors::new();

    // create the actual route using the default route implementation (RouteImpl)
    let route = RouteImpl::new(
        matcher,
        Box::new(dispatcher), // wrap the dispatcher in a box (i.e. put it behind a pointer)
        extractors,
        Delegation::Internal // we are handling this request in the same Router, not an external router
    );

    // wrap the route in a box (i.e. put it behind a pointer) and return it
    Box::new(route)
}

pub fn router() -> Router {
    // create the pipeline set builder
    let pipeline_set_builder = new_pipeline_set();

    // TODO: add pipelines to the pipeline set here

    // finalize the pipeline set
    let pipeline_set =finalize_pipeline_set(pipeline_set_builder);

    // create the route tree builder
    let mut route_tree_builder = TreeBuilder::new();

    // TODO: add routes to the route tree here
    // add the route for the welcome page directly to the route tree. this makes it the root "/"
    route_tree_builder.add_route(create_welcome_route((), pipeline_set.clone()));

    // create a route tree node for "/capitalize"
    let mut capitalize_node = NodeBuilder::new("capitalize", SegmentType::Static);
    capitalize_node.add_route(create_capitalize_route((), pipeline_set.clone()));
    route_tree_builder.add_child(capitalize_node);

    // create a route tree node for "/math"
    let mut cube_container_node = NodeBuilder::new("cube", SegmentType::Static);
    // create a route tree node for everything that comes after "/math/"
    let mut cube_node = NodeBuilder::new("number", SegmentType::Dynamic);
    cube_node.add_route(create_cube_route((), pipeline_set.clone()));
    cube_container_node.add_child(cube_node);
    route_tree_builder.add_child(cube_container_node);

    // finalize the route tree
    let route_tree = route_tree_builder.finalize();

    // create the response finalizer builder
    let response_finalizer_builder = ResponseFinalizerBuilder::new();

    // add response finalizers here...

    // finalize the response finalizer
    let response_finalizer = response_finalizer_builder.finalize();

    //create the router
    Router::new(route_tree, response_finalizer)
}
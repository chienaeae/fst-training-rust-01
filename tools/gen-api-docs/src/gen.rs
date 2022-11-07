use std::collections::HashMap;

use ahash::RandomState;
use http::Method;
use okapi::openapi3::{Components, Info, OpenApi, Operation, PathItem, RefOr, SecurityScheme, Tag};
use schemars::{
    gen::SchemaGenerator,
    schema::{Schema, SchemaObject},
    JsonSchema, Map, MapEntry,
};

use crate::{OpenApiSettings, OperationInfo};

/// A struct that visits all `rocket::Route`s, and aggregates information about
/// them.
#[derive(Debug, Clone)]
pub struct OpenApiGenerator {
    // FIXME: remove `#[allow(dead_code)]` while `settings` is used
    #[allow(dead_code)]
    settings: OpenApiSettings,

    schema_generator: SchemaGenerator,

    operations: Map<String, HashMap<Method, Operation, RandomState>>,
}

impl OpenApiGenerator {
    /// Create a new `OpenApiGenerator` from the settings provided.
    #[must_use]
    pub fn new(settings: OpenApiSettings) -> Self {
        Self {
            schema_generator: settings.schema_settings.clone().into_generator(),
            settings,
            operations: Map::default(),
        }
    }

    /// Add a new `HTTP Method` to the collection of endpoints in the
    /// `OpenApiGenerator`.
    pub fn add_operation(&mut self, op: OperationInfo) {
        match self.operations.entry(op.path) {
            MapEntry::Occupied(mut e) => {
                let map = e.get_mut();
                if map.insert(op.method.clone(), op.operation).is_some() {
                    // This will print a warning if 2 routes have the same path and method
                    eprintln!("Warning: Operation replaced for {}:{}", op.method, e.key());
                }
            }
            MapEntry::Vacant(e) => {
                let mut map = HashMap::default();
                map.insert(op.method, op.operation);
                e.insert(map);
            }
        };
    }

    /// Returns a JSON Schema object for the type `T`.
    pub fn json_schema<T>(&mut self) -> SchemaObject
    where
        T: ?Sized + JsonSchema,
    {
        self.schema_generator.subschema_for::<T>().into()
    }

    /// Obtain the internal `SchemaGenerator` object.
    #[must_use]
    pub const fn schema_generator(&self) -> &SchemaGenerator { &self.schema_generator }

    /// Return the component definition/schema of an object without any
    /// references.
    pub fn json_schema_no_ref<T>(&mut self) -> SchemaObject
    where
        T: ?Sized + JsonSchema,
    {
        <T>::json_schema(&mut self.schema_generator).into()
    }

    /// Generate an `OpenApi` specification for all added operations.
    #[must_use]
    pub fn into_openapi(
        self,
        info: Info,
        tags: Vec<Tag>,
        security_schemes: Map<String, RefOr<SecurityScheme>>,
    ) -> OpenApi {
        let mut schema_generator = self.schema_generator;
        let mut schemas = schema_generator.take_definitions();

        for visitor in schema_generator.visitors_mut() {
            for schema in schemas.values_mut() {
                visitor.visit_schema(schema);
            }
        }

        OpenApi {
            openapi: "3.0.2".to_string(),
            info,
            tags,
            paths: {
                let mut paths = Map::new();
                for (path, map) in self.operations {
                    for (method, op) in map {
                        let path_item = paths.entry(path.clone()).or_default();
                        set_operation(path_item, method, op);
                    }
                }
                paths
            },
            components: Some(Components {
                schemas: schemas
                    .into_iter()
                    .map(|(k, v)| match v {
                        Schema::Object(schema) => (k, schema),
                        Schema::Bool(_) => unreachable!(),
                    })
                    .collect(),
                security_schemes,
                ..Components::default()
            }),
            ..OpenApi::default()
        }
    }
}

fn set_operation(path_item: &mut PathItem, method: Method, op: Operation) {
    let option = match method {
        Method::GET => &mut path_item.get,
        Method::PUT => &mut path_item.put,
        Method::POST => &mut path_item.post,
        Method::DELETE => &mut path_item.delete,
        Method::OPTIONS => &mut path_item.options,
        Method::HEAD => &mut path_item.head,
        Method::PATCH => &mut path_item.patch,
        Method::TRACE => &mut path_item.trace,
        method => {
            eprintln!("Warning: Method {} not supported", method);
            return;
        }
    };
    assert!(option.is_none());
    option.replace(op);
}

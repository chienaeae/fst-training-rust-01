mod gen;

use std::collections::BTreeMap;

use okapi::openapi3::{
    MediaType, Operation, Parameter, ParameterValue, RefOr, RequestBody, Response, Responses,
};
use schemars::{gen::SchemaSettings, JsonSchema, Map};

pub use gen::OpenApiGenerator;

/// Settings which are used to customise the behaviour of the
/// `OpenApiGenerator`.
#[derive(Debug, Clone)]
pub struct OpenApiSettings {
    /// Settings to customise how JSON Schemas are generated.
    pub schema_settings: SchemaSettings,
}

pub struct OperationInfo {
    /// The path of the endpoint
    pub path: String,
    /// The HTTP Method of this endpoint.
    pub method: http::Method,
    /// Contains information to be showed in the documentation about this
    /// endpoint.
    pub operation: Operation,
}

pub trait OpenApiGeneratorHelper {
    fn request_body<T>(&mut self) -> RequestBody
    where
        T: JsonSchema;

    fn response<T, Metadata>(&mut self) -> Responses
    where
        T: JsonSchema,
        Metadata: JsonSchema;

    fn path_parameter<T>(&mut self, name: &str, description: &str) -> Parameter
    where
        T: JsonSchema;

    fn query_parameter<T>(&mut self, name: &str, description: &str, required: bool) -> Parameter
    where
        T: JsonSchema;

    fn header_parameter<T>(&mut self, name: &str, description: &str, required: bool) -> Parameter
    where
        T: JsonSchema;
}

impl OpenApiGeneratorHelper for OpenApiGenerator {
    fn request_body<T>(&mut self) -> RequestBody
    where
        T: JsonSchema,
    {
        let schema = self.json_schema::<T>();
        let media = MediaType { schema: Some(schema), ..MediaType::default() };

        let mut content = Map::default();
        content.insert("application/json".to_string(), media);

        RequestBody { content, required: true, ..RequestBody::default() }
    }

    fn response<T, Metadata>(&mut self) -> Responses
    where
        T: JsonSchema,
        Metadata: JsonSchema,
    {
        // SAFETY: allow: this struct provide type only; qed
        #[allow(dead_code)]
        #[derive(JsonSchema)]
        struct ApiResponse<T, Metadata> {
            /// A status code of API response
            #[schemars(rename = "_status")]
            status: u16,

            /// A metadata of API response
            #[serde(rename = "_metadata", skip_serializing_if = "Option::is_none")]
            metadata: Option<Metadata>,

            data: T,
        }

        let schema = self.json_schema::<ApiResponse<T, Metadata>>();
        let media = MediaType { schema: Some(schema), ..MediaType::default() };

        let mut content = Map::default();
        content.insert("application/json".to_string(), media);
        let response = Response { content, ..Response::default() };

        let mut responses = Map::default();
        responses.insert(200.to_string(), response.into());
        Responses { responses, ..Responses::default() }
    }

    fn path_parameter<T>(&mut self, name: &str, description: &str) -> Parameter
    where
        T: JsonSchema,
    {
        let schema = self.json_schema::<T>();
        Parameter {
            name: name.to_string(),
            location: "path".to_string(),
            description: Some(description.to_string()),
            required: true,
            deprecated: false,
            allow_empty_value: false,
            value: ParameterValue::Schema {
                style: None,
                explode: None,
                allow_reserved: false,
                schema,
                example: None,
                examples: None,
            },
            extensions: Map::default(),
        }
    }

    fn query_parameter<T>(&mut self, name: &str, description: &str, required: bool) -> Parameter
    where
        T: JsonSchema,
    {
        let schema = self.json_schema::<T>();
        Parameter {
            name: name.to_string(),
            location: "query".to_string(),
            description: Some(description.to_string()),
            required,
            deprecated: false,
            allow_empty_value: false,
            value: ParameterValue::Schema {
                style: None,
                explode: None,
                allow_reserved: false,
                schema,
                example: None,
                examples: None,
            },
            extensions: Map::default(),
        }
    }

    fn header_parameter<T>(&mut self, name: &str, description: &str, required: bool) -> Parameter
    where
        T: JsonSchema,
    {
        let schema = self.json_schema::<T>();
        Parameter {
            name: name.to_string(),
            location: "header".to_string(),
            description: Some(description.to_string()),
            required,
            deprecated: false,
            allow_empty_value: false,
            value: ParameterValue::Schema {
                style: None,
                explode: None,
                allow_reserved: false,
                schema,
                example: None,
                examples: None,
            },
            extensions: Map::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct OpenApiResponses {
    pub responses: BTreeMap<String, RefOr<Response>>,
}

impl OpenApiResponses {
    #[must_use]
    pub fn response<T, Metadata>(&mut self, gen: &mut OpenApiGenerator, status_code: u16) -> Self
    where
        T: JsonSchema,
        Metadata: JsonSchema,
    {
        // SAFETY: allow: this struct provide type only; qed
        #[allow(dead_code)]
        #[derive(JsonSchema)]
        struct ApiResponse<T, Metadata> {
            /// A status code of API response
            #[schemars(rename = "_status")]
            status: u16,

            /// A metadata of API response
            #[serde(rename = "_metadata", skip_serializing_if = "Option::is_none")]
            metadata: Option<Metadata>,

            data: T,
        }

        let schema = gen.json_schema::<ApiResponse<T, Metadata>>();
        let media = MediaType { schema: Some(schema), ..MediaType::default() };

        let mut content = Map::default();
        content.insert("application/json".to_string(), media);
        let response = Response { content, ..Response::default() };

        self.responses.insert(status_code.to_string(), response.into());
        self.clone()
    }

    #[must_use]
    pub fn err_response<E, Metadata>(
        &mut self,
        gen: &mut OpenApiGenerator,
        status_code: u16,
    ) -> Self
    where
        E: JsonSchema,
        Metadata: JsonSchema,
    {
        // SAFETY: allow: this struct provide type only; qed
        #[allow(dead_code)]
        #[derive(JsonSchema)]
        struct ApiResponse<E, Metadata> {
            /// A status code of API response
            #[schemars(rename = "_status")]
            status: u16,

            /// A metadata of API response
            #[serde(rename = "_metadata", skip_serializing_if = "Option::is_none")]
            metadata: Option<Metadata>,

            /// An error message of API response
            error: E,
        }

        let schema = gen.json_schema::<ApiResponse<E, Metadata>>();
        let media = MediaType { schema: Some(schema), ..MediaType::default() };

        let mut content = Map::default();
        content.insert("application/json".to_string(), media);
        let response = Response { content, ..Response::default() };

        self.responses.insert(status_code.to_string(), response.into());
        self.clone()
    }

    pub fn responses(&self) -> Responses {
        Responses { responses: self.responses.clone(), ..Responses::default() }
    }
}

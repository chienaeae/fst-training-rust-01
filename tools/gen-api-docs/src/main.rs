// the rules is in following order:
//  - RUSTC ALLOW
//  - RUSTC WARNING
//  - CLIPPY
// rules not enabled:
//  - box_pointers
//  - missing_copy_implementations
//  - missing_debug_implementations
//  - missing_docs
//  - unreachable_pub
//  - unsafe_code
//  - unused_crate_dependencies,
//  - unused_qualifications
//  - unused_results
//  - variant_size_differences,
#![cfg_attr(
    feature = "cargo-clippy",
    deny(
        absolute_paths_not_starting_with_crate,
        deprecated_in_future,
        elided_lifetimes_in_paths,
        explicit_outlives_requirements,
        keyword_idents,
        macro_use_extern_crate,
        meta_variable_misuse,
        missing_abi,
        non_ascii_idents,
        noop_method_call,
        pointer_structural_match,
        semicolon_in_expressions_from_macros,
        single_use_lifetimes,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_op_in_unsafe_fn,
        unstable_features,
        unused_extern_crates,
        unused_import_braces,
        unused_lifetimes,
        anonymous_parameters,
        array_into_iter,
        asm_sub_register,
        bad_asm_style,
        bare_trait_objects,
        bindings_with_variant_name,
        cenum_impl_drop_cast,
        clashing_extern_declarations,
        coherence_leak_check,
        confusable_idents,
        const_evaluatable_unchecked,
        const_item_mutation,
        dead_code,
        deref_nullptr,
        drop_bounds,
        dyn_drop,
        ellipsis_inclusive_range_patterns,
        exported_private_dependencies,
        forbidden_lint_groups,
        function_item_references,
        illegal_floating_point_literal_pattern,
        improper_ctypes,
        improper_ctypes_definitions,
        incomplete_features,
        indirect_structural_match,
        inline_no_sanitize,
        invalid_doc_attributes,
        invalid_value,
        irrefutable_let_patterns,
        large_assignments,
        late_bound_lifetime_arguments,
        legacy_derive_helpers,
        mixed_script_confusables,
        nontrivial_structural_match,
        non_camel_case_types,
        non_fmt_panics,
        non_shorthand_field_patterns,
        non_snake_case,
        non_upper_case_globals,
        no_mangle_generic_items,
        overlapping_range_endpoints,
        path_statements,
        private_in_public,
        proc_macro_back_compat,
        proc_macro_derive_resolution_fallback,
        redundant_semicolons,
        renamed_and_removed_lints,
        stable_features,
        temporary_cstring_as_ptr,
        trivial_bounds,
        type_alias_bounds,
        tyvar_behind_raw_pointer,
        unaligned_references,
        uncommon_codepoints,
        unconditional_recursion,
        uninhabited_static,
        unknown_lints,
        unnameable_test_items,
        unreachable_code,
        unreachable_patterns,
        unstable_name_collisions,
        unsupported_calling_conventions,
        unused_allocation,
        unused_assignments,
        unused_attributes,
        unused_braces,
        unused_comparisons,
        unused_doc_comments,
        unused_features,
        unused_imports,
        unused_labels,
        unused_macros,
        unused_must_use,
        unused_mut,
        unused_parens,
        unused_unsafe,
        unused_variables,
        warnings,
        where_clauses_object_safety,
        clippy::all,
        clippy::cargo,
        clippy::nursery,
        clippy::pedantic
    ),
    allow(
        deprecated,
        clippy::future_not_send,
        clippy::module_name_repetitions,
        clippy::multiple_crate_versions
    )
)]

use std::collections::BTreeMap;

use http::Method;
use mochi::domain::model::{Card, DeleteInfo, LinkedLogicInfo, Logic};
use mochi_gen_api_docs::{
    OpenApiGenerator, OpenApiGeneratorHelper, OpenApiResponses, OpenApiSettings, OperationInfo,
};
use okapi::openapi3::{
    Info, Object, Operation, RefOr, SecurityRequirement, SecurityScheme, SecuritySchemeData,
    Server, Tag,
};
use serde_json::Value as JsonValue;

use lazy_static::lazy_static;
use schemars::gen::SchemaSettings;
use uuid::Uuid;

lazy_static! {
  static ref MOCHI_API_SERVER: Server = Server {
    url: "http://localhost:3000".to_string(),
    description: Some("Local Mochi Server".to_string()),
      ..Server::default()
  };

  static ref CARD_TAG: Tag = Tag {
    name: "Card".to_string(),
    description: Some("Card APIs".to_string()),
    ..Tag::default()
  };

  static ref GENERIC_LOGIC_TAG: Tag = Tag {
    name: "Generic Logic".to_string(),
    description: Some("Generic Logic APIs".to_string()),
    ..Tag::default()
  };

  // security keys
  static ref MOCHI_AUTHENTICATION_JWT_KEY: String = "MochiAuthenticationJWT".to_string();

  // security requirements
  static ref MOCHO_AUTHENTICATION_JWT_SECURITY: SecurityRequirement = {
    let mut sec = BTreeMap::new();
    sec.insert(MOCHI_AUTHENTICATION_JWT_KEY.clone(), vec![]);
    sec
  };

  // security schemas
  static ref MOCHI_AUTHENTICATION_JWT_SCHEMA: SecurityScheme = SecurityScheme {
    description: Some("Requires an authorization JWT.".to_string()),
    data: SecuritySchemeData::Http {
        scheme: "bearer".to_string(),
        bearer_format: Some("JWT".to_string()),
    },
    extensions: Object::default(),
  };
}

fn main() {
    let info = Info {
        title: "Mochi RESTful APIs".to_string(),
        version: "1.2.0".to_string(),
        ..Info::default()
    };

    let tags = vec![CARD_TAG.clone(), GENERIC_LOGIC_TAG.clone()];

    let security_schemes = {
        let mut security_schemes = BTreeMap::new();

        security_schemes.insert(
            MOCHI_AUTHENTICATION_JWT_KEY.clone(),
            RefOr::Object(MOCHI_AUTHENTICATION_JWT_SCHEMA.clone()),
        );

        security_schemes
    };

    let setting = OpenApiSettings {
        schema_settings: SchemaSettings::openapi3()
            .with(|settings| settings.inline_subschemas = true),
    };

    let mut gen = OpenApiGenerator::new(setting);

    card_apis(&mut gen)
        .into_iter()
        .chain(generic_logic_apis(&mut gen))
        .for_each(|o| gen.add_operation(o));

    println!("{}", serde_yaml::to_string(&gen.into_openapi(info, tags, security_schemes)).unwrap());
}

fn card_apis(gen: &mut OpenApiGenerator) -> Vec<OperationInfo> {
    let tags = vec![CARD_TAG.name.clone()];
    let servers = vec![MOCHI_API_SERVER.clone()];
    let security = vec![MOCHO_AUTHENTICATION_JWT_SECURITY.clone()];

    vec![
        OperationInfo {
            path: "/api/v1/card".to_string(),
            method: Method::GET,
            operation: Operation {
                tags: tags.clone(),
                summary: Some("List Cards".to_string()),
                operation_id: Some("list_cards".to_string()),
                description: Some("Returns all Cards".to_string()),
                parameters: Vec::new(),
                responses: OpenApiResponses::default()
                    .response::<Vec<Card>, JsonValue>(gen, 200)
                    .err_response::<JsonValue, JsonValue>(gen, 403)
                    .err_response::<JsonValue, JsonValue>(gen, 500)
                    .responses(),
                servers: Some(servers.clone()),
                security: Some(security.clone()),
                ..Operation::default()
            },
        },
        OperationInfo {
            path: "/api/v1/card".to_string(),
            method: Method::POST,
            operation: Operation {
                tags: tags.clone(),
                summary: Some("Create Card".to_string()),
                operation_id: Some("create_card".to_string()),
                description: Some("Returns the Card which is created".to_string()),
                parameters: Vec::new(),
                responses: OpenApiResponses::default()
                    .response::<Card, JsonValue>(gen, 200)
                    .err_response::<JsonValue, JsonValue>(gen, 403)
                    .err_response::<JsonValue, JsonValue>(gen, 500)
                    .responses(),
                servers: Some(servers.clone()),
                security: Some(security.clone()),
                ..Operation::default()
            },
        },
        OperationInfo {
            path: "/api/v1/card/{id}".to_string(),
            method: Method::GET,
            operation: Operation {
                tags: tags.clone(),
                summary: Some("Get Card by ID".to_string()),
                operation_id: Some("get_card_by_id".to_string()),
                description: Some("Returns the Card of that ID".to_string()),
                parameters: vec![gen.path_parameter::<Uuid>("id", "ID of Card").into()],
                responses: OpenApiResponses::default()
                    .response::<Card, JsonValue>(gen, 200)
                    .err_response::<JsonValue, JsonValue>(gen, 403)
                    .err_response::<JsonValue, JsonValue>(gen, 500)
                    .responses(),
                servers: Some(servers.clone()),
                security: Some(security.clone()),
                ..Operation::default()
            },
        },
        OperationInfo {
            path: "/api/v1/card/{id}".to_string(),
            method: Method::PUT,
            operation: Operation {
                tags: tags.clone(),
                summary: Some("Update Card by ID".to_string()),
                operation_id: Some("update_card_by_id".to_string()),
                description: Some("Returns the updated Card of that ID".to_string()),
                parameters: vec![gen.path_parameter::<Uuid>("id", "ID of Card").into()],
                responses: OpenApiResponses::default()
                    .response::<Card, JsonValue>(gen, 200)
                    .err_response::<JsonValue, JsonValue>(gen, 403)
                    .err_response::<JsonValue, JsonValue>(gen, 500)
                    .responses(),
                servers: Some(servers.clone()),
                security: Some(security.clone()),
                ..Operation::default()
            },
        },
        OperationInfo {
            path: "/api/v1/card/{id}".to_string(),
            method: Method::DELETE,
            operation: Operation {
                tags,
                summary: Some("Delete Card by ID".to_string()),
                operation_id: Some("delete_card_by_id".to_string()),
                description: Some("Returns the deleted Card ID".to_string()),
                parameters: vec![gen.path_parameter::<Uuid>("id", "ID of Card").into()],
                responses: OpenApiResponses::default()
                    .response::<DeleteInfo, JsonValue>(gen, 200)
                    .err_response::<JsonValue, JsonValue>(gen, 403)
                    .err_response::<JsonValue, JsonValue>(gen, 500)
                    .responses(),
                servers: Some(servers),
                security: Some(security),
                ..Operation::default()
            },
        },
    ]
}

fn generic_logic_apis(gen: &mut OpenApiGenerator) -> Vec<OperationInfo> {
    let tags = vec![GENERIC_LOGIC_TAG.name.clone()];
    let servers = vec![MOCHI_API_SERVER.clone()];
    let security = vec![MOCHO_AUTHENTICATION_JWT_SECURITY.clone()];

    vec![
        OperationInfo {
            path: "/api/v1/generic-logic".to_string(),
            method: Method::GET,
            operation: Operation {
                tags: tags.clone(),
                summary: Some("List Generic Logics".to_string()),
                operation_id: Some("list_generic_logics".to_string()),
                description: Some("Returns all Generic Logics".to_string()),
                parameters: Vec::new(),
                responses: OpenApiResponses::default()
                    .response::<Vec<Logic>, JsonValue>(gen, 200)
                    .err_response::<JsonValue, JsonValue>(gen, 403)
                    .err_response::<JsonValue, JsonValue>(gen, 500)
                    .responses(),
                servers: Some(servers.clone()),
                security: Some(security.clone()),
                ..Operation::default()
            },
        },
        OperationInfo {
            path: "/api/v1/generic-logic/{id}/link-card-by-id/{card_id}".to_string(),
            method: Method::POST,
            operation: Operation {
                tags: tags.clone(),
                summary: Some("Link Generic Logic to Card".to_string()),
                operation_id: Some("link_generic_logic_to_card".to_string()),
                description: Some("Returns linked logic info".to_string()),
                parameters: vec![
                    gen.path_parameter::<Uuid>("id", "ID of Generic Logic").into(),
                    gen.path_parameter::<Uuid>("card_id", "ID of Card").into(),
                ],
                responses: OpenApiResponses::default()
                    .response::<LinkedLogicInfo, JsonValue>(gen, 200)
                    .err_response::<JsonValue, JsonValue>(gen, 403)
                    .err_response::<JsonValue, JsonValue>(gen, 500)
                    .responses(),
                servers: Some(servers.clone()),
                security: Some(security.clone()),
                ..Operation::default()
            },
        },
        OperationInfo {
            path: "/api/v1/generic-logic/{id}/link-card-by-id/{card_id}".to_string(),
            method: Method::DELETE,
            operation: Operation {
                tags,
                summary: Some("Unlink Generic Logic from Card".to_string()),
                operation_id: Some("unlink_generic_logic_from_card".to_string()),
                description: Some("Returns the deleted linked logic info ID".to_string()),
                parameters: vec![
                    gen.path_parameter::<Uuid>("id", "ID of Generic Logic").into(),
                    gen.path_parameter::<Uuid>("card_id", "ID of Card").into(),
                ],
                responses: OpenApiResponses::default()
                    .response::<DeleteInfo, JsonValue>(gen, 200)
                    .err_response::<JsonValue, JsonValue>(gen, 403)
                    .err_response::<JsonValue, JsonValue>(gen, 500)
                    .responses(),
                servers: Some(servers),
                security: Some(security),
                ..Operation::default()
            },
        },
    ]
}

// OpenAPI v3.1.0 Specification
//
// OpenAPI inside OpenAPI
//
// The version of the OpenAPI document: 3.1.0
//
// Generated by: https://openapi-generator.tech

use crate::v31;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Openapi {
  #[serde(rename = "openapi", deserialize_with = "Option::deserialize")]
  pub openapi: Option<serde_json::Value>,
  #[serde(rename = "info")]
  pub info: Box<v31::Info>,
  #[serde(rename = "jsonSchemaDialect", default, skip_serializing_if = "Option::is_none")]
  pub json_schema_dialect: Option<serde_json::Value>,
  #[serde(rename = "servers", default, skip_serializing_if = "Option::is_none")]
  pub servers: Option<Vec<v31::Server>>,
  #[serde(rename = "paths", default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
  pub paths: std::collections::BTreeMap<String, v31::ObjectOrRef<v31::PathItem>>,
  #[serde(rename = "webhooks", default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
  pub webhooks: std::collections::BTreeMap<String, v31::ObjectOrRef<v31::PathItem>>,
  #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
  pub components: Option<Box<v31::Components>>,
  #[serde(rename = "security", default, skip_serializing_if = "Option::is_none")]
  pub security: Option<serde_json::Value>,
  #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
  pub tags: Option<Vec<v31::Tag>>,
  #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
  pub external_docs: Option<Box<v31::ExternalDocumentation>>,
}

impl Openapi {
  pub fn new(openapi: Option<serde_json::Value>, info: v31::Info) -> Openapi {
    Openapi {
      openapi,
      info: Box::new(info),
      json_schema_dialect: None,
      servers: None,
      paths: std::collections::BTreeMap::default(),
      webhooks: std::collections::BTreeMap::default(),
      components: None,
      security: None,
      tags: None,
      external_docs: None,
    }
  }

  pub fn into_operations(&self) -> impl Iterator<Item = (String, String, v31::Operation)> {
    self
      .paths
      .iter()
      .filter_map(|(path, path_item_ref)| path_item_ref.resolve(self).ok().map(|path_item| (path, path_item)))
      .flat_map(|(path, path_item)| {
        path_item.into_operations().map(|(method, operation)| (path.clone(), method, operation))
      })
      .collect::<Vec<_>>()
      .into_iter()
      .chain(
        self
          .webhooks
          .iter()
          .filter_map(|(path, path_item_ref)| path_item_ref.resolve(self).ok().map(|path_item| (path, path_item)))
          .flat_map(|(path, path_item)| {
            path_item.into_operations().map(|(method, operation)| (path.clone(), method, operation))
          })
          .collect::<Vec<_>>(),
      )
  }
}

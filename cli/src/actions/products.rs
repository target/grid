// Copyright (c) 2019 Target Brands, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::http::submit_batches;
use crate::transaction::{
    product_batch_builder, GRID_PRODUCT_NAMESPACE, GRID_SCHEMA_NAMESPACE, PIKE_NAMESPACE,
};
use grid_sdk::protocol::product::payload::{
    Action, ProductCreateAction, ProductCreateActionBuilder, ProductUpdateAction, ProductUpdateActionBuilder, ProductPayload, ProductPayloadBuilder,
    ProductDeleteAction, ProductDeleteActionBuilder,
};
use grid_sdk::protocol::product::state::ProductType;
use grid_sdk::protocol::schema::state::{DataType, PropertyValue, PropertyValueBuilder};
use grid_sdk::protos::IntoProto;
use reqwest::Client;

use crate::error::CliError;
use serde::Deserialize;

use crate::yaml_parser::{
    parse_value_as_boolean, parse_value_as_bytes, parse_value_as_i64, parse_value_as_lat_long,
    parse_value_as_sequence, parse_value_as_string, parse_value_as_u32, parse_value_as_data_type, parse_value_as_product_type
};

use serde_yaml::{Mapping, Value};
use sawtooth_sdk::messages::batch::BatchList;

#[derive(Debug, Deserialize)]
pub struct GridProduct {
    pub product_id: String,
    pub product_type: String,
    pub owner: String,
    pub properties: Vec<GridPropertyValue>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GridPropertyValue {
    pub name: String,
    pub data_type: String,
    pub bytes_value: Option<Vec<u8>>,
    pub boolean_value: Option<bool>,
    pub number_value: Option<i64>,
    pub string_value: Option<String>,
    pub enum_value: Option<u32>,
    pub struct_values: Option<Vec<String>>,
    pub lat_long_value: Option<(String, String)>,
}

/**
 * Print the fields for a given product
 * 
 * product - Product to be printed
 */
pub fn display_product(product: &GridProduct) {
    println!(
        "Product Id: {:?}\n Product Type: {:?}\n Owner: {:?}\n Properties:",
        product.product_id, product.product_type, product.owner,
    );
    display_product_property_definitions(&product.properties);
}

/**
 * Iterate through all fields of a Property Value and print the given value
 * 
 * properties - Property values to be printed
 */
pub fn display_product_property_definitions(properties: &[GridPropertyValue]) {
    properties.iter().for_each(|def| {
        println!(
            "\tProperty Name: {:?}\n\t Data Type: {:?}\n\t Bytes Value: {:?}\n\t Boolean Value: {:?}
        Number Value: {:?}\n\t String Value: {:?}\n\t Enum Value: {:?}\n\t Struct Values: {:?}\n\t Lat/Lon Values: {:?}\n\t",
            def.name,
            def.data_type,
            def.bytes_value,
            def.boolean_value,
            def.number_value,
            def.string_value,
            def.enum_value,
            def.struct_values,
            def.lat_long_value,
        );
    });
}

/**
 * Print all products in state
 * 
 * url - Url for the REST API
 */
pub fn do_list_products(url: &str) -> Result<(), CliError> {
    let client = Client::new();
    let products = client
        .get(&format!("{}/product", url))
        .send()?
        .json::<Vec<GridProduct>>()?;
    products.iter().for_each(|product| display_product(product));
    Ok(())
}

/**
 * Print all products in state
 * 
 * url - Url for the REST API
 * product_id - e.g. GTIN
 */
pub fn do_show_product(url: &str, product_id: &str) -> Result<(), CliError> {
    let client = Client::new();
    let product = client
        .get(&format!("{}/product/{}", url, product_id))
        .send()?
        .json::<GridProduct>()?;
    display_product(&product);
    Ok(())
}

/**
 * Create a new product
 * 
 * url - Url for the REST API
 * key - Signing key of the agent
 * wait -
 * path - Path to the yaml file that contains the product descriptions
 */
pub fn do_create_products(
    url: &str,
    key: Option<String>,
    wait: u64,
    path: &str,
) -> Result<(), CliError> {
    let payloads = parse_product_yaml(path, Action::ProductCreate(ProductCreateAction::default()))?;
    let batch_list = build_batches_from_payloads(payloads, key)?;
    submit_batches(url, wait, &batch_list)
}

/**
 * Update an existing product
 * 
 * url - Url for the REST API
 * key - Signing key of the agent
 * wait -
 * path - Path to the yaml file that contains the product descriptions
 */
pub fn do_update_products(
    url: &str,
    key: Option<String>,
    wait: u64,
    path: &str,
) -> Result<(), CliError> {
    let payloads = parse_product_yaml(path, Action::ProductUpdate(ProductUpdateAction::default()))?;
    let batch_list = build_batches_from_payloads(payloads, key)?;
    submit_batches(url, wait, &batch_list)
}

/**
 * Delete an existing product
 * 
 * url - Url for the REST API
 * key - Signing key of the agent
 * wait -
 * path - Path to the yaml file that contains the product descriptions
 */
pub fn do_delete_products(
    url: &str,
    key: Option<String>,
    wait: u64,
    path: &str,
) -> Result<(), CliError> {
    let payloads = parse_product_yaml(path, Action::ProductDelete(ProductDeleteAction::default()))?;
    let batch_list = build_batches_from_payloads(payloads, key)?;
    submit_batches(url, wait, &batch_list)
}

/**
 * Build a batch from our Product Payloads. The CLI is responsible for batch creation.
 * 
 * payloads - Product payloads
 * key - Signing key of the agent
 */
pub fn build_batches_from_payloads(payloads: Vec<ProductPayload>, key: Option<String>) -> Result<BatchList, CliError> {
    let mut batch_list_builder = product_batch_builder(key);
    for payload in payloads {
        batch_list_builder = batch_list_builder.add_transaction(
            &payload.into_proto()?,
            &[
                PIKE_NAMESPACE.to_string(),
                GRID_SCHEMA_NAMESPACE.to_string(),
            ],
            &[GRID_PRODUCT_NAMESPACE.to_string()],
        )?;
    }

    Ok(batch_list_builder.create_batch_list())
}

/**
 * Iterate through a list of products in a yaml file to build our payloads.
 * 
 * path: Path to the yaml file
 * action: Determines the type of product payload to generate
 */
fn parse_product_yaml(path: &str, action: Action) -> Result<Vec<ProductPayload>, CliError> {
    let file = std::fs::File::open(path)?;
    let products_yaml: Vec<Mapping> = serde_yaml::from_reader(file)?;

    match action {
        Action::ProductCreate(_) => products_yaml
            .iter()
            .map(|product_yaml| {
                let product_id =
                    parse_value_as_string(product_yaml, "product_id")?.ok_or_else(|| {
                        CliError::InvalidYamlError(
                            "Missing `product_id` field for Product.".to_string(),
                        )
                    })?;

                let product_type = parse_value_as_product_type(
                    &parse_value_as_string(product_yaml, "product_type")?.ok_or_else(|| {
                        CliError::InvalidYamlError(
                            "Missing `product_type` field for property definition.".to_string(),
                        )
                    })?,
                )?;

                let owner = parse_value_as_string(product_yaml, "owner")?.ok_or_else(|| {
                    CliError::InvalidYamlError("Missing `owner` field for Product.".to_string())
                })?;

                let properties =
                    parse_value_as_sequence(product_yaml, "properties")?.ok_or_else(|| {
                        CliError::InvalidYamlError(
                            "Product is missing `properties` field.".to_string(),
                        )
                    })?;

                let property_values = parse_value_as_properties(&properties)?;

                generate_create_product_payload(product_type, &product_id, &owner, &property_values)
            })
            .collect::<Result<Vec<ProductPayload>, _>>(),
        Action::ProductUpdate(_) => products_yaml
            .iter()
            .map(|product_yaml| {
                let product_id =
                    parse_value_as_string(product_yaml, "product_id")?.ok_or_else(|| {
                        CliError::InvalidYamlError(
                            "Missing `product_id` field for Product.".to_string(),
                        )
                    })?;

                let product_type = parse_value_as_product_type(
                    &parse_value_as_string(product_yaml, "product_type")?.ok_or_else(|| {
                        CliError::InvalidYamlError(
                            "Missing `product_type` field for property definition.".to_string(),
                        )
                    })?,
                )?;

                let properties =
                    parse_value_as_sequence(product_yaml, "properties")?.ok_or_else(|| {
                        CliError::InvalidYamlError(
                            "Product is missing `properties` field.".to_string(),
                        )
                    })?;

                let property_values = parse_value_as_properties(&properties)?;

                generate_update_product_payload(product_type, &product_id, &property_values)
            })
            .collect::<Result<Vec<ProductPayload>, _>>(),
        Action::ProductDelete(_) => products_yaml
            .iter()
            .map(|product_yaml| {
                let product_id =
                    parse_value_as_string(product_yaml, "product_id")?.ok_or_else(|| {
                        CliError::InvalidYamlError(
                            "Missing `product_id` field for Product.".to_string(),
                        )
                    })?;

                let product_type = parse_value_as_product_type(
                    &parse_value_as_string(product_yaml, "product_type")?.ok_or_else(|| {
                        CliError::InvalidYamlError(
                            "Missing `product_type` field for property definition.".to_string(),
                        )
                    })?,
                )?;
                generate_delete_product_payload(product_type, &product_id)
            })
            .collect::<Result<Vec<ProductPayload>, _>>(),
    }
}

/**
 * Given a yaml key/val, parse the val as a list of Property Value objects
 * 
 * properties - One or more yaml objects to be parsed as a Property Value
 */
fn parse_value_as_properties(properties: &[Value]) -> Result<Vec<PropertyValue>, CliError> {
    properties
        .iter()
        .map(|value| {
            let property = value.as_mapping().ok_or_else(|| {
                CliError::InvalidYamlError(
                    "Failed to parse schema property definition.".to_string(),
                )
            })?;
            parse_value_as_property_values(property)
        })
        .collect()
}

/**
 * Given a yaml object, parse it as a Property Value
 * 
 * property - Yaml object we have determined to be a Property Value
 */
fn parse_value_as_property_values(property: &Mapping) -> Result<PropertyValue, CliError> {
    let data_type = parse_value_as_data_type(&parse_value_as_string(property, "data_type")?.ok_or_else(
        || {
            CliError::InvalidYamlError(
                "Missing `data_type` field for property definition.".to_string(),
            )
        },
    )?)?;

    let mut property_value = PropertyValueBuilder::new()
        .with_name(parse_value_as_string(property, "name")?.ok_or_else(|| {
            CliError::InvalidYamlError(
                "Missing `name` field for product property value.".to_string(),
            )
        })?)
        .with_data_type(data_type.clone());

    property_value = match data_type {
        DataType::Bytes => property_value.with_bytes_value(
            parse_value_as_bytes(property, "bytes_value")?.ok_or_else(|| {
                CliError::InvalidYamlError(
                    "Missing `bytes_value` field for property value with type BYTES.".to_string(),
                )
            })?,
        ),
        DataType::Boolean => property_value.with_boolean_value(
            parse_value_as_boolean(property, "boolean_value")?.ok_or_else(|| {
                CliError::InvalidYamlError(
                    "Missing `boolean_value` field for property value with type BOOLEAN."
                        .to_string(),
                )
            })?,
        ),
        DataType::Number => property_value.with_number_value(
            parse_value_as_i64(property, "number_value")?.ok_or_else(|| {
                CliError::InvalidYamlError(
                    "Missing `number_value` field for property value with type NUMBER.".to_string(),
                )
            })?,
        ),
        DataType::String => property_value.with_string_value(
            parse_value_as_string(property, "string_value")?.ok_or_else(|| {
                CliError::InvalidYamlError(
                    "Missing `string_value` field for property value with type STRING.".to_string(),
                )
            })?,
        ),
        DataType::Enum => property_value.with_enum_value(
            parse_value_as_u32(property, "enum_value")?.ok_or_else(|| {
                CliError::InvalidYamlError(
                    "Missing `enum_value` field for property value with type ENUM.".to_string(),
                )
            })?,
        ),
        DataType::Struct => {
            // Properties is a repeated field, so we recursively call `parse_value_as_properties`
            let properties = parse_value_as_properties(
                property
                    .get(&Value::String("struct_values".to_string()))
                    .unwrap()
                    .as_sequence()
                    .unwrap(),
            )?;
            property_value.with_struct_values(properties)
        }
        DataType::LatLong => property_value.with_lat_long_value(
            parse_value_as_lat_long(property, "lat_long_value")?.ok_or_else(|| {
                CliError::InvalidYamlError(
                    "Missing `lat_long_value` field for property value with type LATLONG."
                        .to_string(),
                )
            })?,
        ),
    };

    property_value.build().map_err(|err| {
        CliError::PayloadError(format!("Failed to build property definition: {}", err))
    })
}

/**
 * Generate the payload needed to create a new product
 * 
 * product_type - e.g. GS1
 * product_id - e.g. GTIN
 * owner - Identifier of the organization responsible for maintaining the product
 * properties - One or more property values
 */
fn generate_create_product_payload(
    product_type: ProductType,
    product_id: &str,
    owner: &str,
    properties: &[PropertyValue],
) -> Result<ProductPayload, CliError> {
    let mut product_payload = ProductPayloadBuilder::new();

    let product_create_action_builder = ProductCreateActionBuilder::new()
        .with_product_id(product_id.to_string())
        .with_product_type(product_type)
        .with_owner(owner.to_string())
        .with_properties(properties.to_vec());

    let product_create_action = product_create_action_builder.build().map_err(|err| {
        CliError::PayloadError(format!("Failed to build product create payload: {}", err))
    })?;

    product_payload = product_payload.with_action(Action::ProductCreate(product_create_action));

    product_payload.build().map_err(|err| {
        CliError::PayloadError(format!("Failed to build product payload: {}", err))
    })
}

/**
 * Generate the payload needed to update an existing product
 * 
 * product_type - e.g. GS1
 * product_id - e.g. GTIN
 * properties - One or more property values
 */
fn generate_update_product_payload(
    product_type: ProductType,
    product_id: &str,
    properties: &[PropertyValue],
) -> Result<ProductPayload, CliError> {
    let mut product_payload = ProductPayloadBuilder::new();

    let product_update_action_builder = ProductUpdateActionBuilder::new()
        .with_product_id(product_id.to_string())
        .with_product_type(product_type)
        .with_properties(properties.to_vec());

    let product_update_action = product_update_action_builder.build().map_err(|err| {
        CliError::PayloadError(format!("Failed to build product update payload: {}", err))
    })?;

    product_payload = product_payload.with_action(Action::ProductUpdate(product_update_action));

    product_payload.build().map_err(|err| {
        CliError::PayloadError(format!("Failed to build product payload: {}", err))
    })
}

/**
 * Generate the payload needed to delete an existing product
 * 
 * product_type - e.g. GS1
 * product_id - e.g. GTIN
 */
fn generate_delete_product_payload(product_type: ProductType, product_id: &str) -> Result<ProductPayload, CliError> {
    let mut product_payload = ProductPayloadBuilder::new();

    let product_delete_action_builder =
        ProductDeleteActionBuilder::new()
            .with_product_id(product_id.to_string())
            .with_product_type(product_type);

    let product_delete_action = product_delete_action_builder.build().map_err(|err| {
        CliError::PayloadError(format!("Failed to build product delete payload: {}", err))
    })?;

    product_payload = product_payload.with_action(Action::ProductDelete(product_delete_action));

    product_payload.build().map_err(|err| {
        CliError::PayloadError(format!("Failed to build product delete payload: {}", err))
    })
}

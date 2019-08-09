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

use crate::error::CliError;
use grid_sdk::protocol::schema::state::{LatLong, LatLongBuilder};
use serde_yaml::{Mapping, Sequence, Value};
use grid_sdk::protocol::schema::state::DataType;
use grid_sdk::protocol::product::state::ProductType;

/**
 * Given a yaml object, parse it as a sequence
 * 
 * property - Yaml object we wish to parse in as a sequence
 */
pub fn parse_value_as_sequence(
    property: &Mapping,
    key: &str,
) -> Result<Option<Sequence>, CliError> {
    match property.get(&Value::String(key.to_string())) {
        Some(value) => match value.as_sequence() {
            Some(value) => Ok(Some(value.to_vec())),
            None => Err(CliError::InvalidYamlError(format!(
                "Value of {} has an invalid format. Expected is a yaml list.",
                key
            ))),
        },
        None => Ok(None),
    }
}

/**
 * Given a yaml object, parse it as a string
 * 
 * property - Yaml object we wish to parse in as a string
 */
pub fn parse_value_as_string(property: &Mapping, key: &str) -> Result<Option<String>, CliError> {
    match property.get(&Value::String(key.to_string())) {
        Some(value) => match value.as_str() {
            Some(value) => Ok(Some(value.to_string())),
            None => Err(CliError::InvalidYamlError(format!(
                "Value of {} has an invalid format. Expected is a yaml string.",
                key
            ))),
        },
        None => Ok(None),
    }
}

/**
 * Given a yaml object, parse it as a bool
 * 
 * property - Yaml object we wish to parse in as a bool
 */
pub fn parse_value_as_boolean(property: &Mapping, key: &str) -> Result<Option<bool>, CliError> {
    match property.get(&Value::String(key.to_string())) {
        Some(value) => match value.as_bool() {
            Some(value) => Ok(Some(value)),
            None => Err(CliError::InvalidYamlError(format!(
                "Value of {} has an invalid format. Expected is a yaml boolean (true/false).",
                key
            ))),
        },
        None => Ok(None),
    }
}

/**
 * Given a yaml object, parse it as a vector of bytes
 * 
 * property - Yaml object we wish to parse in as a vector of bytes
 */
pub fn parse_value_as_bytes(property: &Mapping, key: &str) -> Result<Option<Vec<u8>>, CliError> {
    match property.get(&Value::String(key.to_string())) {
        Some(value) => match value.as_i64() {
            Some(value) => Ok(Some(value.to_string().into_bytes())),
            None => Err(CliError::InvalidYamlError(format!(
                "Value of {} has an invalid format. Expected is a yaml boolean (true/false).",
                key
            ))),
        },
        None => Ok(None),
    }
}

/**
 * Given a yaml object, parse it as an i64
 * 
 * property - Yaml object we wish to parse in as an i64
 */
pub fn parse_value_as_i64(property: &Mapping, key: &str) -> Result<Option<i64>, CliError> {
    match property.get(&Value::String(key.to_string())) {
        Some(value) => match value.as_i64() {
            Some(value) => Ok(Some(value.to_string().parse::<i64>().map_err(|_| {
                CliError::InvalidYamlError(format!(
                    "Failed to parse value of {} to 64 bit integer",
                    key
                ))
            })?)),
            None => Err(CliError::InvalidYamlError(format!(
                "Value of {} has an invalid format. Expected is a yaml integer.",
                key
            ))),
        },
        None => Ok(None),
    }
}

/**
 * Given a yaml object, parse it as an u64
 * 
 * property - Yaml object we wish to parse in as an u64
 */
pub fn parse_value_as_u32(property: &Mapping, key: &str) -> Result<Option<u32>, CliError> {
    match property.get(&Value::String(key.to_string())) {
        // Serde only has methods to match 64 bit nums
        Some(value) => match value.as_u64() {
            Some(value) => Ok(Some(value.to_string().parse::<u32>().map_err(|_| {
                CliError::InvalidYamlError(format!(
                    "Failed to parse value of {} to 32 bit integer",
                    key
                ))
            })?)),
            None => Err(CliError::InvalidYamlError(format!(
                "Value of {} has an invalid format. Expected is a yaml integer.",
                key
            ))),
        },
        None => Ok(None),
    }
}

/**
 * Given a yaml object, parse it as a LatLong object
 * 
 * property - Yaml object we wish to parse in as a LatLong object
 */
pub fn parse_value_as_lat_long(property: &Mapping, key: &str) -> Result<Option<LatLong>, CliError> {
    match property.get(&Value::String(key.to_string())) {
        Some(value) => match value.as_str() {
            Some(value) => {
                let lat_long: Vec<&str> = value.split(',').collect();

                let lat: i64 = lat_long[0].parse().map_err(|_| {
                    CliError::InvalidYamlError(format!(
                        "Failed to parse the Latitude value for LatLong: {}",
                        key
                    ))
                })?;

                let long: i64 = lat_long[1].parse().map_err(|_| {
                    CliError::InvalidYamlError(format!(
                        "Failed to parse the Longitude value for LatLong: {}",
                        key
                    ))
                })?;

                Ok(Some(
                    LatLongBuilder::new()
                        .with_lat_long(lat, long)
                        .build()
                        .map_err(|err| {
                            CliError::InvalidYamlError(format!("Failed to build LatLong: {}", err))
                        })?,
                ))
            }
            None => Err(CliError::InvalidYamlError(format!(
                "Value of {} has an invalid format. Expected is a yaml integer.",
                key
            ))),
        },
        None => Ok(None),
    }
}

/**
 * Given a yaml object, parse it as a PropertyDefinition DataType
 * 
 * property - Yaml object we wish to parse in as a PropertyDefinition DataType
 */
pub fn parse_value_as_data_type(data_type: &str) -> Result<DataType, CliError> {
    match data_type.to_lowercase().as_ref() {
        "string" => Ok(DataType::String),
        "boolean" => Ok(DataType::Boolean),
        "bytes" => Ok(DataType::Bytes),
        "number" => Ok(DataType::Number),
        "enum" => Ok(DataType::Enum),
        "struct" => Ok(DataType::Struct),
        "lat_long" => Ok(DataType::LatLong),
        _ => Err(CliError::InvalidYamlError(format!(
            "Invalid data type for PropertyDefinition: {}",
            data_type
        ))),
    }
}

/**
 * Given a yaml object, parse it as a Product ProductType
 * 
 * property - Yaml object we wish to parse in as a Product ProductType
 */
pub fn parse_value_as_product_type(product_type: &str) -> Result<ProductType, CliError> {
    match product_type.to_uppercase().as_ref() {
        "GS1" => Ok(ProductType::GS1),
        _ => Err(CliError::InvalidYamlError(format!(
            "Invalid product_type for value: {}",
            product_type
        ))),
    }
}

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

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use sabre_sdk::ApplyError;
    } else {
        use sawtooth_sdk::processor::handler::ApplyError;
    }
}

use grid_sdk::protocol::product::payload::{Action, ProductCreateAction, ProductPayload};

pub fn validate_payload(payload: &ProductPayload) -> Result<(), ApplyError> {
    validate_timestamp(*payload.timestamp())?;
    match payload.action() {
        Action::ProductCreate(action_payload) => validate_product_create_action(action_payload),
        _ => Ok(()),
    }
}

fn validate_product_create_action(
    product_create_action: &ProductCreateAction,
) -> Result<(), ApplyError> {
    if product_create_action.product_id() == "" {
        return Err(ApplyError::InvalidTransaction(String::from(
            "product_id cannot be empty string",
        )));
    }
    if product_create_action.owner() == "" {
        return Err(ApplyError::InvalidTransaction(String::from(
            "Owner cannot be empty string",
        )));
    }
    Ok(())
}

fn validate_timestamp(timestamp: u64) -> Result<(), ApplyError> {
    match timestamp {
        0 => Err(ApplyError::InvalidTransaction(String::from(
            "Timestamp is not set",
        ))),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use grid_sdk::protos::product_payload::{
        ProductCreateAction as ProductCreateActionProto, ProductPayload as ProductPayloadProto,
        ProductPayload_Action as ActionProto,
    };
    use grid_sdk::protos::product_state::Product_ProductType;
    use grid_sdk::protos::IntoNative;

    #[test]
    /// Test that an ok is returned if the payload with ProductCreateAction is valid. This test
    /// needs to use the proto directly originally to be able to mimic the scenarios possbile
    /// from creating a ProductCreateAction from bytes. This is because the
    /// ProductCreateActionBuilder protects from building certain invalid payloads.
    fn test_validate_payload_valid() {
        let mut payload_proto = ProductPayloadProto::new();
        payload_proto.set_action(ActionProto::PRODUCT_CREATE);
        payload_proto.set_timestamp(2);
        let mut action = ProductCreateActionProto::new();
        action.set_product_id("my_product_id".to_string());
        action.set_owner("my_owner".to_string());
        action.set_product_type(Product_ProductType::GS1);
        payload_proto.set_product_create(action);
        let payload = payload_proto.clone().into_native().unwrap();
        assert!(
            validate_payload(&payload).is_ok(),
            "Payload should be valid"
        );
    }
}

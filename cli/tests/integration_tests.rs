extern crate assert_cmd;

use assert_cmd::prelude::*;
use std::path::Path;
use std::process::Command;
//use reqwest::Client;
//use crate::error::CliError;

mod integration {
    use super::*;

    //const URL: &str = "127.0.0.1:8080";
    /*
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OrganizationSlice {
        pub org_id: String,
        pub name: String,
        pub address: String,
        pub metadata: Vec<JsonValue>,
    }

    impl OrganizationSlice {
        pub fn from_organization(organization: &Organization) -> Self {
            Self {
                org_id: organization.org_id.clone(),
                name: organization.name.clone(),
                address: organization.address.clone(),
                metadata: organization.metadata.clone(),
            }
        }
    }

    fn make_request_show_organization(url: &str, id: &str) -> Result<(), CliError> {
        let client = Client::new();
        let organization = client
            .get(&format!("{}/organization/{}", url, id))
            .send()?
            .json::<OrganizationSlice>()?;
    }*/

    #[test]
    fn test_keygen() {
        //run `grid keygen {KEY_NAME} --force --key_dir {KEY_DIR}`
        let key_name: String = "test.key".to_owned();
        let key_dir: String = "./".to_owned();
        let mut cmd = Command::new("grid");
        cmd.arg("keygen")
            .args(&["key_name", &key_name])
            .arg("--force")
            .args(&["--key_dir", &key_dir]);
        cmd.assert().success(); //assert the command did not return an error code
        let public_key_path_str = key_dir.clone() + &key_name + ".pub";
        let private_key_path_str = key_dir.clone() + &key_name + ".priv";
        //assert the command successfully created the keys as files
        assert!(Path::new(&public_key_path_str).exists());
        assert!(Path::new(&private_key_path_str).exists());
    }

    #[test]
    fn test_organization_create() {
        let org_id: &str = "me_inc";
        let org_name: &str = "Me Inc.";
        let org_address: &str = "123 Main St";
        let expected_print_message: &str = "whatever this returns, formatted"; //todo! add `grid organization show`
                                                                               //make command to create an organization
        let mut cmd_create = Command::new("grid");
        cmd_create
            .arg("organization")
            .arg("create")
            .args(&["org_id", &org_id])
            .args(&["name", &org_name])
            .args(&["address", &org_address]);
        cmd_create.assert().success();
        //make a command to show the created organization
        let mut cmd_show = Command::new("grid");
        cmd_show
            .arg("organization")
            .arg("show")
            .args(&["org_id", org_id]);
        //get bytes from stdout, convert to string and check the message printed
        let print_message =
            String::from_utf8(cmd_show.assert().success().get_output().stdout.clone())
                .expect("Found invalid UTF-8");
        assert_eq!(print_message, expected_print_message)
    }

    #[test]
    fn test_product_create() {
        let yaml_file: &str = "test_product.yaml";
        let product_id: &str = "723382885088";
        let product_type: &str = "GS1";
        let expected_print_message: &str = "whatever this returns, formatted"; //todo! add `grid product show`
        let mut cmd_create = Command::new("grid");
        cmd_create
            .arg("product")
            .arg("create")
            .args(&["path", yaml_file]);
        cmd_create.assert().success();
        let mut cmd_show = Command::new("grid");
        cmd_show
            .arg("product")
            .arg("show")
            .args(&["product_id", product_id])
            .args(&["product_type", product_type]);
        let print_message =
            String::from_utf8(cmd_show.assert().success().get_output().stdout.clone())
                .expect("Found invalid UTF-8");
        assert_eq!(print_message, expected_print_message)
    }
}

extern crate assert_cmd;

use std::process::Command;
use assert_cmd::prelude::*;
use std::path::PathBuf;
use reqwest::Client;
use crate::error::CliError;

#[cfg(all(feature = "test-integration", test))]
mod integration {
    use super::*;

    const URL: &str = "127.0.0.1:8080";

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
    }

    #[test]
    fn test_function() {
        let mut cmd = Command::new("grid");
        cmd
            .args(&["--url", URL])
            .args(&["--wait", "10"])
            .args(&["-k", "file.key"]);
        let assert = cmd.assert();
        assert
            .failure();
        assert
            .success();
    }

    #[test]
    fn test_keygen() {
        //run `grid keygen {KEY_NAME} --force --key_dir {KEY_DIR}`
        let KEY_NAME: &str = "test.key";
        let KEY_DIR: &str = "./";
        let mut cmd = Command::new("grid");
        cmd
            .arg("keygen")
            .args(&["key_name", &KEY_NAME])
            .arg("--force")
            .args(&["--key_dir", &KEY_DIR]);
        cmd.assert().success(); //assert the command did not return an error code
        let key_dir = PathBuf::from(KEY_DIR);
        let mut public_key_path = key_dir.clone()
            .push(format!("{}.pub", &KEY_NAME));
        let mut private_key_path = key_dir.clone()
            .push(format!("{}.priv", &KEY_NAME));
        //assert the command successfully created the keys as files
        assert!(public_key_path.exists());
        assert!(private_key_path.exists());
    }

    #[test]
    fn test_organization_create() {
        let ORG_ID: &str = "me_inc";
        let ORG_NAME: &str = "Me Inc.";
        let PRINT_MESSAGE: &str = "whatever this returns, formatted"; //todo! add `grid organization show`
        //make command to create an organization
        let mut cmd_create = Command::new("grid");
        cmd_create
            .arg("organization")
            .arg("create")
            .args(&["org_id", &ORG_ID])
            .args(&["name", &ORG_NAME])
            .args(&["address", &ORG_ADDRESS]);
        cmd_create.assert().success();
        //make a command to show the created organization
        let mut cmd_show = Command::new("grid");
        cmd_show
            .arg("organization")
            .arg("show")
            .args(&["org_id", ORG_ID]);
        //get bytes from stdout, convert to string and check the message printed
        let print_message = String::from_utf8(
            cmd_show
                .assert()
                .success()
                .get_output()
                .stdout
            )
            .expect("Found invalid UTF-8");
        assert_eq!(print_message, &PRINT_MESSAGE)

    }
}

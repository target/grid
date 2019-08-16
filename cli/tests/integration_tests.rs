extern crate assert_cmd;
extern crate dirs;

use assert_cmd::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use users::get_current_username;

mod integration {
    use super::*;

    static KEY_DIR: &str = "/root";
    static PUB_KEY_FILE: &str = "/root/.grid/keys/root.pub";

    static ORG_ID: &str = "314156";
    static ORG_NAME: &str = "target";
    static ORG_ADDRESS: &str = "target hq";

    static PRODUCT_FILE: &str = "/build/cli/tests/test_product.yaml";

    mod product {
        use super::*;
        #[test]
        #[ignore]
        fn test_product_create() {
            //run `grid keygen`
            let key_name: String = get_current_username().unwrap().into_string().unwrap();
            let mut key_dir: PathBuf = dirs::home_dir().unwrap();
            assert_eq!(PathBuf::from(KEY_DIR), key_dir);
            key_dir.push(".grid");
            key_dir.push("keys");
            key_dir.push(&key_name);
            let mut cmd = Command::cargo_bin("grid").unwrap();
            cmd.arg("keygen").arg("--force");
            cmd.assert().success();
            let mut public_key_path = key_dir.clone();
            public_key_path.set_extension("pub");
            let mut private_key_path = key_dir.clone();
            private_key_path.set_extension("priv");
            assert!(public_key_path.exists());
            assert!(private_key_path.exists());

            //run `grid organization create`
            let mut cmd_create = Command::cargo_bin("grid").unwrap();
            cmd_create
                .arg("organization")
                .arg("create")
                .arg(&ORG_ID)
                .arg(&ORG_NAME)
                .arg(&ORG_ADDRESS)
                .args(&["--metadata", "gs1_company_prefixes=314"]);
            cmd_create.assert().success();

            //run `grid agent create`
            let pub_key = fs::read_to_string(PUB_KEY_FILE).unwrap();
            let mut cmd_create = Command::cargo_bin("grid").unwrap();
            cmd_create
                .arg("agent")
                .arg("create")
                .arg(&ORG_ID)
                .arg(&pub_key)
                .arg("true")
                .args(&[
                    "--roles",
                    "admin",
                    "can_create_product",
                    "can_update_product",
                    "can_delete_product",
                ]);
            cmd_create.assert().success();
            //run `grid product create`
            let mut cmd_create = Command::cargo_bin("grid").unwrap();
            cmd_create.arg("product").arg("create").arg(&PRODUCT_FILE);
            cmd_create.assert().success();
        }
    }
}

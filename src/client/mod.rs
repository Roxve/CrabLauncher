use rule::is_allowed;

use crate::{json::client::Client, LIB_DIR};

pub mod download;
pub mod rule;

pub fn get_req_libs(client: &Client) -> Vec<String> {
    let mut libs = Vec::new();

    for lib in &client.libraries {
        if lib.rules.is_some() {
            let rules = lib.rules.as_ref().unwrap();
            if !is_allowed(rules) {
                continue;
            }
        }

        if lib.natives.is_some() {
            let natives = lib.natives.as_ref().unwrap();

            for (os, native) in natives {
                if os == &crate::OS {
                    let classifiers = lib.downloads.classifiers.as_ref().unwrap();

                    let path = classifiers.get(native).unwrap().path.as_ref().unwrap();
                    libs.push(LIB_DIR.to_owned() + path);
                    break;
                }
            }
        }

        if lib.downloads.artifact.is_some() {
            libs.push(
                LIB_DIR.to_owned()
                    + &lib
                        .downloads
                        .artifact
                        .as_ref()
                        .unwrap()
                        .path
                        .as_ref()
                        .unwrap(),
            );
        }
    }

    return libs;
}

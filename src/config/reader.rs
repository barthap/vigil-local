// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::{collections::HashMap, env, fs};

use envsubst::substitute;
use toml;

use super::config::*;
use crate::APP_ARGS;

pub struct ConfigReader;

/// Check whether `value` contains invalid characters.
fn validate<S>(value: S, kind: &str) -> Result<(), String>
where
    S: AsRef<str>,
{
    static FORBIDDEN: &[&str] = &["$", "{", "}"];
    for c in FORBIDDEN {
        if value.as_ref().contains(c) {
            let err_msg = format!(
                "variable {} '{}' contains forbidden character '{}'",
                kind,
                value.as_ref(),
                c
            );
            return Err(err_msg);
        };
    }
    Ok(())
}

impl ConfigReader {
    pub fn make() -> Config {
        debug!("reading config file: {}", &APP_ARGS.config);

        // Read configuration
        let raw_conf = fs::read_to_string(&APP_ARGS.config).expect("cannot find config file");

        debug!("read config file: {}", &APP_ARGS.config);

        // Replace environment variables
        let environment = env::vars()
            .filter(|(k, v)| {
                if let Err(err) = validate(k, "key") {
                    warn!("Skipped invalid env var '{k}': {err}");
                    return false;
                }
                if let Err(err) = validate(v, "value") {
                    warn!("Skipped invalid env var '{k}': {err}");
                    return false;
                }
                true
            })
            .collect::<HashMap<String, String>>();

        let conf =
            substitute(&raw_conf, &environment).expect("cannot substitute environment variables");

        // Parse configuration
        toml::from_str(&conf).expect("syntax error in config file")
    }
}

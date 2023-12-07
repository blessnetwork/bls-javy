//use std::collections::HashMap;

use anyhow::{bail, Result};
use javy::{quickjs::JSValue, Runtime};
use blockless_sdk::*;

use crate::{APIConfig, JSApiSet};
pub(super) use config::FetchIOConfig;

mod config;

pub(super) struct FetchIO;

impl JSApiSet for FetchIO {
    fn register(&self, runtime: &Runtime, config: &APIConfig) -> Result<()> {
        let context = runtime.context();

        let global = context.global_object()?;

        let mut javy_object = global.get_property("Javy")?;

        // If you're defining something on the `Javy` object, ensure it exists.
        if javy_object.is_undefined() {
            javy_object = context.object_value()?;
            global.set_property("Javy", javy_object)?;
        }

        // `wrap_callback`` has a static lifetime so you can't use references to the config in its body.
        global.set_property(
            "__javy_fetchio_get",
            context.wrap_callback(move |_ctx, _this, args| {
                let [url] = args else {
                    bail!("Incorrect number of arguments");
                };
                // Convert JSValueRefs to Rust types.
                let url: String = url.try_into()?;

                let opts = HttpOptions::new("GET", 30, 10);
                let http = BlocklessHttp::open(&url, &opts);
                let http = http.unwrap();
                let body = http.get_all_body().unwrap();
                let body = String::from_utf8(body).unwrap();
                let data = match json::parse(&body).unwrap() {
                    json::JsonValue::Object(o) => o,
                    _ => panic!("must be object"),
                };
                
                println!("Http Data: {:?}", data);
                
                Ok(JSValue::Undefined)
            })?,
        )?;

        context.eval_global("fetch.js", include_str!("fetch.js"))?;

        Ok(())
    }
}

// Automated tests are highly recommended
#[cfg(test)]
mod tests {
    use std::env;

    use crate::{APIConfig, JSApiSet};
    use anyhow::Result;
    use javy::Runtime;

    use super::FetchIO;

    #[test]
    fn test_print_env_var() -> Result<()> {
        let runtime = Runtime::default();
        let context = runtime.context();
        FetchIO.register(&runtime, &APIConfig::default())?;
        env::set_var("HELLO", "there");
        let _ = context.eval_global("main", "Javy.Env.print('HELLO');")?;
        env::remove_var("HELLO");
        Ok(())
    }
}

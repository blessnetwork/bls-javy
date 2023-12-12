use std::collections::HashMap;
use anyhow::{anyhow, Result};
use javy::{quickjs::JSValue, Runtime};
use serde_json::{Value, from_slice};

use crate::{APIConfig, JSApiSet};
use javy::quickjs::{JSContextRef, JSValueRef};

pub(super) struct FetchIO;

impl JSApiSet for FetchIO {
    fn register(&self, runtime: &Runtime, _config: &APIConfig) -> Result<()> {
        let context = runtime.context();
        let global = context.global_object()?;

        // `wrap_callback`` has a static lifetime so you can't use references to the config in its body.
        global.set_property(
            "__javy_fetchio_request",
            context.wrap_callback(fetchio_request())?,
        )?;

        context.eval_global("fetch.js", include_str!("fetch.js"))?;

        Ok(())
    }
}

fn fetchio_request() -> impl FnMut(&JSContextRef, JSValueRef, &[JSValueRef]) -> anyhow::Result<JSValue> {
    move |_ctx: &JSContextRef, _this: JSValueRef, args: &[JSValueRef]| {
        if args.len() != 4 {
            return Err(anyhow!("Expecting 4 arguments, received {}", args.len()));
        }

        let url: String = args[0].try_into()?;
        let buffer: Vec<u8> = args[1].try_into()?;
        let byte_offset: usize = args[2].try_into()?;
        let byte_length: usize = args[3].try_into()?;

        let sliced_buffer: &[u8] = &buffer[byte_offset..(byte_offset + byte_length)];
        let request_obj: Value = from_slice(sliced_buffer)?;

        // @TODO: Call host API methods here
        println!("{:?}", url);
        println!("{:?}", request_obj);

        // Prepare Response
        let mut response: HashMap<String, JSValue> = HashMap::new();
        response.insert("ok".to_string(), JSValue::Bool(true));

        Ok(JSValue::Object(response))
    }
}
mod blockless;

use std::collections::HashMap;
use anyhow::{anyhow, Result};
use javy::{quickjs::JSValue, Runtime};
use serde_json::{from_slice};
use serde::{Deserialize, Serialize};

use crate::{APIConfig, JSApiSet};
use javy::quickjs::{JSContextRef, JSValueRef};
use crate::fetch_io::blockless::BlocklessHttp;

pub(super) struct FetchIO;

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchOptions {
    method: String,
}

impl FetchOptions {
    pub fn new(method: &str) -> Self {
        FetchOptions {
            method: method.into()
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl JSApiSet for FetchIO {
    fn register(&self, runtime: &Runtime, _config: &APIConfig) -> Result<()> {
        let context = runtime.context();
        let global = context.global_object()?;

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
        let request_obj: FetchOptions = from_slice(sliced_buffer)?;

        // Prepare Response
        let mut response: HashMap<String, JSValue> = HashMap::new();

        // Conditionally invoke the runtime host call
        if cfg!(feature = "runtime_bls") {
            let http = BlocklessHttp::open(&url, &request_obj).unwrap();
            let body = String::from_utf8(http.get_all_body().unwrap()).unwrap();
            http.close();

            response.insert("ok".to_string(), JSValue::Bool(true));
            response.insert("body".to_string(), JSValue::String(body));
        } else {
            response.insert("ok".to_string(), JSValue::Bool(false));
            response.insert("body".to_string(), JSValue::String(String::from("{}")));
        }

        Ok(JSValue::Object(response))
    }
}
use anyhow::Result;
use javy::{quickjs::JSValue, Runtime};
use javy::quickjs::{JSContextRef, JSValueRef};

use rand::RngCore;
use crate::{APIConfig, JSApiSet};

pub struct Crypto;

impl JSApiSet for Crypto {
    fn register(&self, runtime: &Runtime, _config: &APIConfig) -> Result<()> {
        let context = runtime.context();
        let global = context.global_object()?;

        global.set_property(
            "__javy_crypto_get_random_values",
            context.wrap_callback(get_random_values())?,
        )?;

        context.eval_global("crypto.js", include_str!("crypto.js"))?;

        Ok(())
    }
}

fn get_random_values() -> impl FnMut(&JSContextRef, JSValueRef, &[JSValueRef]) -> Result<JSValue> {
    move |_ctx: &JSContextRef, _this: JSValueRef, args: &[JSValueRef]| {
        let mut rng = rand::rngs::OsRng::default();

        let buffer = args[0].as_bytes_mut()?;
        let byte_offset: usize = args[1].try_into()?;
        let byte_length: usize = args[2].try_into()?;

        let buffer = &mut buffer[byte_offset..(byte_offset + byte_length)];

        // Fill the buffer with random values.
        rng.fill_bytes(buffer);

        Ok(JSValue::ArrayBuffer(buffer.to_vec()))
    }
}

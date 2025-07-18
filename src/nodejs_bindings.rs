// src/nodejs_bindings.rs
use crate::dtln_processor::{DtlnDeferredProcessor, DtlnProcessEngine};
use neon::prelude::*;
use neon::types::buffer::TypedArray;
use std::sync::{Arc, Mutex};

fn dtln_create_napi(mut cx: FunctionContext) -> JsResult<JsBox<Arc<Mutex<DtlnDeferredProcessor>>>> {
    let dtln_processor = DtlnDeferredProcessor::new();
    let Ok(dtln_processor) = dtln_processor else {
        return cx.throw_error("Failed to create DtlnDeferredProcessor");
    };

    Ok(cx.boxed(Arc::new(Mutex::new(dtln_processor))))
}

fn dtln_stop_napi(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let dtln_processor = cx.argument::<JsBox<Arc<Mutex<DtlnDeferredProcessor>>>>(0)?;
    dtln_processor.lock().unwrap().stop();
    Ok(cx.undefined())
}

fn dtln_denoise_napi(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    if cx.len() != 3 {
        return cx.throw_error("Invalid number of arguments, expected <engine: JsBox, samples: Float32Array, output: Float32Array>");
    }

    let processor_starved;

    let result: std::io::Result<()> = {
        let dtln_processor = cx.argument::<JsBox<Arc<Mutex<DtlnDeferredProcessor>>>>(0)?;

        let samples = cx.argument::<JsTypedArray<f32>>(1).unwrap();
        let mut output = cx.argument::<JsTypedArray<f32>>(2).unwrap();

        let lock = cx.lock();
        let samples_slice = samples.try_borrow(&lock).unwrap();
        let mut output_slice = output.try_borrow_mut(&lock).unwrap();

        let denoise_result = dtln_processor
            .lock()
            .unwrap()
            .denoise(&samples_slice)
            .map_err(|e| panic!("Error in dtln_denoise: {}", e))
            .unwrap();

        processor_starved = denoise_result.processor_starved;

        output_slice[..denoise_result.samples.len()].copy_from_slice(&denoise_result.samples);
        Ok(())
    };

    if result.is_ok() {
        Ok(cx.boolean(processor_starved))
    } else {
        cx.throw_error("Error in dtln_denoise")
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("dtln_denoise", dtln_denoise_napi)?;
    cx.export_function("dtln_create", dtln_create_napi)?;
    cx.export_function("dtln_stop", dtln_stop_napi)?;

    Ok(())
}

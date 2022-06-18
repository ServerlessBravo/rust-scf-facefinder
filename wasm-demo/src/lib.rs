use wasm_bindgen::prelude::*;

use facefinder::{Opt, PostData};
use log::info;

mod facefinder;
use serde_json::{json, Value};

#[wasm_bindgen]
pub fn process_event(body_str: String) -> String {
    info!("获取到 body_str:{body_str}");

    let mut event: Value = Value::default();
    if body_str.len() > 0 {
        if let Ok(e) = serde_json::from_str(&body_str) {
            event = e;
        }
    }

    /*
    提交数据:PostData
    */
    let resp = match serde_json::from_value::<PostData>(event) {
        Ok(data) => {
            let mut opt = Opt::default();

            if let Some(min_size) = data.min_size {
                opt.min_size = min_size;
            }
            if let Some(scale_factor) = data.scale_factor {
                opt.scale_factor = scale_factor;
            }
            if let Some(shift_factor) = data.shift_factor {
                opt.shift_factor = shift_factor;
            }
            if let Some(threshold) = data.threshold {
                opt.threshold = threshold;
            }

            match facefinder::detect_faces(&opt, &data.img) {
                Ok(faces) => serde_json::to_value(faces).unwrap_or(Value::Array(vec![])),
                Err(err) => json!({ "error": format!("{:?}", err) }),
            }
        }
        Err(err) => {
            json!({ "error": format!("{:?}", err) })
        }
    };

    resp.to_string()
}

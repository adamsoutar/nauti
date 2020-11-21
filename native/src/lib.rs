use rayon::prelude::*;
use neon::prelude::*;
use censor::*;

fn clean_strings(mut cx: FunctionContext) -> JsResult<JsArray> {
    let censor = Censor::Standard;

    let arg = cx.argument::<JsString>(0)?;
    let rs_str = arg.value();

    let v: Vec<String> = serde_json::from_str(&rs_str[..]).unwrap();
    let censored: Vec<String> = v.par_iter().map(|naughty| censor.censor(&naughty[..])).collect();

    // Alloc a JS array and fill it
    let ret_arr = JsArray::new(&mut cx, censored.len() as u32);

    for i in 0..censored.len() {
        let heap_str = cx.string(&censored[i][..]);
        ret_arr.set(&mut cx, i as u32, heap_str)?;
    }

    Ok(ret_arr)
}

fn clean_objects(mut cx: FunctionContext) -> JsResult<JsArray> {
    let censor = Censor::Standard;

    let arg = cx.argument::<JsString>(0)?;
    let arg2 = cx.argument::<JsString>(1)?;
    let rs_str = arg.value();
    let key_str = arg2.value();

    let v: Vec<serde_json::Value> = serde_json::from_str(&rs_str[..]).unwrap();
    let censored: Vec<serde_json::Value> = v.par_iter().map(|naughty| {
        let mut cloned = naughty.clone();
        match &naughty[&key_str[..]] {
            serde_json::Value::String(prop_str) => {
                cloned[&key_str[..]] = serde_json::Value::String(censor.censor(&prop_str[..]));
            },
            _ => panic!("Prop is not a string")
        }
        cloned
    }).collect();

    // Alloc a JS array and fill it with objects made from the serde_json Values in censored
    let ret_arr = JsArray::new(&mut cx, censored.len() as u32);

    for i in 0..censored.len() {
        let js_value = neon_serde::to_value(&mut cx, &censored[i])?;
        ret_arr.set(&mut cx, i as u32, js_value)?;
    }

    Ok(ret_arr)
}

register_module!(mut cx, {
    cx.export_function("rawCleanStrings", clean_strings)?;
    cx.export_function("rawCleanObjects", clean_objects)?;
    Ok(())
});

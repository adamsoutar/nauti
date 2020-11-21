use serde_json::{Value};
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

// fn clean_objects(mut cx: FunctionContext) -> JsResult<JsArray> {
//     let censor = Censor::Standard;
//
//     let arr = cx.argument::<JsArray>(0)?;
//     let key = cx.argument::<JsString>(1)?;
//
//     for i in 0..arr.len() {
//         let obj = arr.get(&mut cx, i)?;
//         let dict = obj.downcast_or_throw::<JsObject, FunctionContext>(&mut cx)?;
//         let prop = dict.get(&mut cx, key)?;
//         let prop_str = prop.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
//         let censored = censor.censor(&prop_str.value()[..]);
//         let js_string = cx.string(censored);
//         dict.set(&mut cx, key, js_string)?;
//     }
//
//     Ok(arr)
// }

register_module!(mut cx, {
    cx.export_function("rawCleanStrings", clean_strings)?;
    // cx.export_function("cleanObjects", clean_objects)?;
    Ok(())
});

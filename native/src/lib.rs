use neon::prelude::*;
use censor::*;

fn filter_strings(mut cx: FunctionContext) -> JsResult<JsArray> {
    let censor = Censor::Standard;

    let arr = cx.argument::<JsArray>(0)?;

    for i in 0..arr.len() {
        let obj = arr.get(&mut cx, i)?;
        let str = obj.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
        let censored = censor.censor(&str.value()[..]);
        let js_string = cx.string(censored);
        arr.set(&mut cx, i as u32, js_string).unwrap();
    }

    Ok(arr)
}

fn filter_objects(mut cx: FunctionContext) -> JsResult<JsArray> {
    let censor = Censor::Standard;

    let arr = cx.argument::<JsArray>(0)?;
    let key = cx.argument::<JsString>(1)?;

    for i in 0..arr.len() {
        let obj = arr.get(&mut cx, i)?;
        let dict = obj.downcast_or_throw::<JsObject, FunctionContext>(&mut cx)?;
        let prop = dict.get(&mut cx, key)?;
        let prop_str = prop.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
        let censored = censor.censor(&prop_str.value()[..]);
        let js_string = cx.string(censored);
        dict.set(&mut cx, key, js_string)?;
    }

    Ok(arr)
}

register_module!(mut cx, {
    cx.export_function("filterStrings", filter_strings)?;
    cx.export_function("filterObjects", filter_objects)?;
    Ok(())
});

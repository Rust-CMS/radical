use actix_web::web::Data;
use handlebars::{
    to_json, Context, Handlebars, Helper, HelperDef, JsonRender, Output, RenderContext,
    RenderError, ScopedJson,
};
use std::sync::Mutex;

fn get(
    h: &Helper,
    _: &Handlebars,
    ctx: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let module_title = h
        .param(0)
        .ok_or(RenderError::new(
            "No module title provided to helper function.",
        ))?
        .render();

    // helper that allows a custom error message to show if the value does not exist in the database yet.
    // errors are passed up through `ok_or` returning a RenderError, then passed to the `try` block.
    let field  = (|| -> Result<String, RenderError> {
        let values = ctx
            .data()
            .get("fields")
            .ok_or(RenderError::new("No fields exist on this page."))?
            .get(module_title.clone())
            .ok_or(RenderError::new(&format!(
                "Field `{}` does not exist on the page.",
                module_title
            )))?
            .get("content")
            .unwrap()
            .render();

        Ok(values)
    })();

    out.write(&field.unwrap_or_else(|e| e.desc))?;
    Ok(())
}

/// For this helper, we need to return ScopedJson.
/// The #each operator does not accept a string as an argument, and normal helpers are meant to write strings.
/// With such, we use the handlebars HelperDef object that allows us to return ScopedJson.
#[derive(Clone, Copy)]
pub struct ArrayHelper;

impl HelperDef for ArrayHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        let module_title = h
            .param(0)
            .ok_or(RenderError::new(
                "No module title provided to helper function.",
            ))?
            .render();

        let fields = (|| -> Result<ScopedJson, RenderError>  {
            let values = ctx
                .data()
                .get("array_fields")
                .ok_or(RenderError::new("No fields exist on this page."))?
                .get(module_title.clone())
                .ok_or(RenderError::new(&format!(
                    "Field `{}` does not exist on the page.",
                    module_title
                )))?
                .clone()
                .into();

            Ok(values)
        })();
        let empty_array: Vec<String> = Vec::new();
        Ok(Some(fields.unwrap_or(to_json(empty_array).into())))
    }
}

pub static ARRAY_HELPER: ArrayHelper = ArrayHelper;

pub fn register_helpers(handlebars: Data<Mutex<Handlebars<'_>>>) {
    handlebars
        .lock()
        .unwrap()
        .register_helper("get", Box::new(get));
    handlebars
        .lock()
        .unwrap()
        .register_helper("getarray", Box::new(ARRAY_HELPER));
}

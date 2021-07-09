use actix_web::web::Data;
use handlebars::{
    Context, Handlebars, Helper, HelperDef, JsonRender, Output, RenderContext, RenderError,
    ScopedJson,
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
    let field_result: Result<String, RenderError> = try {
        ctx.data()
            .get("fields")
            .ok_or(RenderError::new("No fields exist on this page."))?
            .get(module_title.clone())
            .ok_or(RenderError::new(&format!(
                "Field `{}` does not exist on the page.",
                module_title
            )))?
            .get("content")
            .unwrap()
            .render()
    };

    out.write(&field_result.unwrap_or_else(|e| e.desc))?;
    Ok(())
}

#[derive(Clone, Copy)]
pub struct ArrayHelper;

impl HelperDef for ArrayHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
    ) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        let module_title = h
            .param(0)
            .ok_or(RenderError::new(
                "No module title provided to helper function.",
            ))?
            .render();

        let res: ScopedJson = ctx.data()
            .get("array_fields")
            .ok_or(RenderError::new("No fields exist on this page."))?
            .get(module_title.clone())
            .ok_or(RenderError::new(&format!(
                "Field `{}` does not exist on the page.",
                module_title
            )))?
            .clone()
            .into();

        Ok(Some(res))
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

use std::sync::Mutex;

use actix_web::web::Data;
use handlebars::{Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};

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
    out.write(
        &ctx.data()
            .get("fields")
            .unwrap()
            .get(module_title.clone())
            .unwrap()
            .get("content")
            .unwrap()
            .render(),
    )?;
    Ok(())
}

pub fn register_helpers(handlebars: Data<Mutex<Handlebars<'_>>>) {
    handlebars
        .lock()
        .unwrap()
        .register_helper("get", Box::new(get));
}

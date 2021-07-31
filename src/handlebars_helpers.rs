use rocket_dyn_templates::handlebars::{Handlebars, HelperDef, RenderContext, Helper, Context, JsonRender, HelperResult, Output, RenderError};
use rocket_dyn_templates::handlebars::Renderable;

pub fn contains<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    r: &'reg Handlebars,
    ctx: &'rc Context,
    rc: &mut RenderContext<'reg, 'rc>,
    out: &mut dyn Output,
) -> HelperResult {
    // param 0 should be an array
    // param 1 should be a string
    // param 0 should contain param 1
    let array = h.param(0);
    if array.is_none() {
        return Err(RenderError::new("No array given"));
    }
    let array = array.unwrap().value().as_array();
    if array.is_none() {
        return Err(RenderError::new("First param should be an array"));
    }
    let array = array.unwrap();
    let needle = h.param(1);
    if needle.is_none() {
        return Err(RenderError::new("No needle given"));
    }
    let needle = needle.unwrap().value();
    if array.contains(needle) {
        return h.template()
        .map(|t| t.render(r, ctx, rc, out))
        .unwrap_or(Ok(()));
    } else {
        return h.inverse()
        .map(|t| t.render(r, ctx, rc, out))
        .unwrap_or(Ok(()));
    }
}
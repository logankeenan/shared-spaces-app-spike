use handlebars::Handlebars;
use serde_json::Value;


// TODO this should probably leverage include_dir and just get everything at compile time
//  rather than trying to remember to include_str for each new template that is made
// https://crates.io/crates/include_dir
pub fn render(template_name: String, model: Value) -> String {
    let layout = include_str!("../pages/layout.hbs");
    let file_list = include_str!("../pages/file/list.hbs");
    let file_details =  include_str!("../pages/file/details.hbs");
    let device_create =  include_str!("../pages/device/create.hbs");
    let mut page_template = "";

    if template_name == "file/list" {
        page_template = file_list;
    } else if template_name == "file/details" {
        page_template = file_details;
    } else if template_name == "device/create" {
        page_template = device_create;
    }

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("layout_page", layout);

    let result = handlebars.render_template(page_template, &model);

    match result {
        Ok(markup) => {
            markup
        },
        Err(_) => "nothing".to_string(),
    }
}
use handlebars::Handlebars;
use serde_json::Value;

pub fn render(template_name: String, model: Value) -> String {
    let layout = include_str!("../pages/layout.hbs");
    let file_list = include_str!("../pages/file/list.hbs");
    let mut page_template = "";

    if template_name == "file/list" {
        page_template = file_list;
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
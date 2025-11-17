use console::style;

fn replace_optional(template: &str, value: Option<&str>) -> String {
    match value {
        Some(v) => template.replace("{?}", v),
        None => template.to_string()
    }
}

pub fn success(msg: &str, value: Option<&str>) {
    println!("{} : {}", style("✔").green(), replace_optional(msg, value));
}

pub fn info(msg: &str, value: Option<&str>) {
    println!("{} : {}", style("i").blue(), replace_optional(msg, value));
}

pub fn warning(msg: &str, value: Option<&str>) {
    println!("{} : {}", style("⚠").yellow(), replace_optional(msg, value));
}

pub fn error(msg: &str, value: Option<&str>) {
    println!("{} : {}", style("✘").red(), replace_optional(msg, value));
}
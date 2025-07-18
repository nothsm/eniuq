fn eniuq(template: &str) { println!(r#"{}("{}"); }}"#, template, template.replace(r#"\"#, r#"\\"#).replace(r#"""#, r#"\""#)); }

fn main() {
    eniuq("fn eniuq(template: &str) { println!(r#\"{}(\"{}\"); }}\"#, template, template.replace(r#\"\\\"#, r#\"\\\\\"#).replace(r#\"\"\"#, r#\"\\\"\"#)); }

fn main() {
    eniuq"); }

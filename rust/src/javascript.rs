use std::{fs, process::Command};

use color_eyre::Result;

pub fn execute_js(javascript: &mut Option<String>) -> Result<()> {
    let mut start_js = fs::read_to_string("javascript/start.js")?;
    let end_js = fs::read_to_string("javascript/end.js")?;

    start_js.push_str(&javascript.as_mut().expect("Javascript needs to be parsed"));
    start_js.push_str(&end_js);

    start_js = html_escape::decode_html_entities(&start_js).to_string();

    let path = "tmp.js";
    fs::write(path, &start_js.as_bytes())?;
    let exit_status = Command::new("node").arg(path).status()?;
    assert!(exit_status.success());

    Ok(())
}

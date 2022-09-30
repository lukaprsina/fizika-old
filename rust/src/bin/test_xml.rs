use std::{
    fs::File,
    io::{self, Write},
};

use xml::writer::{EmitterConfig, EventWriter, Result, XmlEvent};

fn handle_event<W: Write>(w: &mut EventWriter<W>, line: String) -> Result<()> {
    let line = line.trim();
    let event: XmlEvent = if line.starts_with("+") && line.len() > 1 {
        XmlEvent::start_element(&line[1..]).into()
    } else if line.starts_with("-") {
        XmlEvent::end_element().into()
    } else {
        XmlEvent::characters(&line).into()
    };
    w.write(event)
}

fn main() {
    let mut file = File::create("output.xml").unwrap();

    let input = io::stdin();
    let mut output = io::stdout();
    let mut config = EmitterConfig::new().perform_indent(true);
    config.perform_escaping = false;

    let mut writer = config.create_writer(&mut file);
    loop {
        print!("> ");
        output.flush().unwrap();
        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => match handle_event(&mut writer, line) {
                Ok(_) => {}
                Err(e) => panic!("Write error: {}", e),
            },
            Err(e) => panic!("Input error: {}", e),
        }
    }
}

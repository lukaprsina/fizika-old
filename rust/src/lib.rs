use std::{error::Error, sync::Arc, thread::sleep, time::Duration};

use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};

pub fn get_links(tab: Arc<Tab>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut links = Vec::new();
    let elements = tab.find_elements("body > div a")?;

    for element in elements {
        let attributes = element.get_attributes()?.expect("No attributes");
        let on_click = attributes.get("onclick").unwrap();
        let start = "window.open(\'".len();
        let end = on_click
            .chars()
            .skip(start)
            .position(|c| c == '\'')
            .unwrap();
        let tab_str = on_click.chars().skip(start).take(end).collect::<String>();
        links.push(tab_str);
    }

    Ok(links)
}

pub fn create_fizika_tab() -> Result<(Arc<Tab>, Browser), Box<dyn Error>> {
    let options = LaunchOptionsBuilder::default()
        .headless(false)
        .idle_browser_timeout(Duration::from_secs(10 * 60))
        .build()?;

    let browser = Browser::new(options)?;
    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to("http://fizika.sc-nm.si/")?;
    tab.wait_until_navigated()?;
    sleep(Duration::from_secs(1));
    Ok((tab, browser))
}

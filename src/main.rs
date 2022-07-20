use colored::*;
use curl::easy::Easy;
use regex::Regex;

fn main() {
    println!("{}", "🦀 loading menu...".red().bold());
    let html = get_data();
    let days = ["ma", "ti", "ke", "to", "pe"];

    let regex = Regex::new(r#"(?m)<p class="ruoka-header-(ruoka|kasvisruoka)">([^<]*)<"#).unwrap();

    let result = regex.captures_iter(html.as_str());

    // create menu array and push meals to it
    let mut menu: Vec<&str> = Vec::new();
    for mat in result {
        menu.push(mat.get(2).map_or("food", |m| m.as_str()));
    }

    // print the menu to the output
    for (i, meal) in menu.iter().enumerate() {
        let f = meal.replace("\\xc3\\xb6", "ö").replace("\\xc3\\xa4", "ä");
        if i % 2 == 0 {
            print!("\n{} ", days[i / 2].blue().bold());
            let norm = "meal:".yellow();
            println!("{norm} {f}");
        } else {
            let vege = "vege:".green();
            println!("   {vege} {f}");
        }
    }
}

fn get_data() -> String {
    // get data from the url
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle
        .url("https://www.mayk.fi/tietoa-meista/ruokailu/")
        .unwrap();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    // convert to String from &[u8]
    return String::from_utf8(data).expect("The body is not valid UTF8!");
}

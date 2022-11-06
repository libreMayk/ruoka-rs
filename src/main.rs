use chrono::prelude::*;
use colored::*;
use curl::easy::Easy;
use regex::Regex;

fn main() {
    println!("{} {}", "~>".red().bold(), "loading menu...".red().bold());
    let html = get_data("https://www.mayk.fi/tietoa-meista/ruokailu/");

    let days = ["ma", "ti", "ke", "to", "pe"].to_vec();

    let today = days[day_today()];

    let regex = Regex::new(r#"(?m)<p class="ruoka-header-(ruoka|kasvisruoka)">([^<]*)<"#).unwrap();

    let result = regex.captures_iter(html.as_str());

    // create menu array and push meals to it
    let mut menu: Vec<&str> = Vec::new();
    for mat in result {
        menu.push(mat.get(2).map_or("food", |m| m.as_str()));
    }

    // if no items in menu, return
    if menu.len() == 0 {
        println!("{}", "no food this week :(".blue().bold());
        return;
    }

    // print the menu to the output
    for (i, meal) in menu.iter().enumerate() {
        let food = meal.replace("\\xc3\\xb6", "ö").replace("\\xc3\\xa4", "ä");
        let len = days.len() - (menu.len() / 2) + i / 2;
        let day = days[len];

        if i % 2 == 0 {
            // show current day
            if day == today {
                print!("{} ", day.bold().red());
            } else {
                print!("{} ", day.bold().blue());
            }

            let norm = "meal:".yellow();
            let mut full = format!("{norm} {food}");

            if day == today {
                full = full.bold().to_string();
            }

            println!("{full}");
        } else {
            let vege = "vege:".green();
            let mut full = format!("   {vege} {}", food.green());

            if day == today {
                full = full.bold().to_string();
            }

            println!("{full}");
        }
    }
}

fn get_data(url: &str) -> String {
    // get data from the url
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url).unwrap();
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

// check if the day is not out of limit
fn day_today() -> usize {
    let dt = Local::now().weekday().num_days_from_monday();

    if dt >= 5 {
        return 0;
    } else {
        return dt as usize;
    }
}

use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Utc};
use std::env;
use wkhtmltopdf::*;

fn generate_html_style() -> String {
    r##"
    <style>
        html,body,table,tr,td,ul,a,div,td,th {
            color: #000;
            font-size: 9px;
            margin: 0px;
            padding: 0px;
            text-decoration: none;
        }
        div.page {
            height: 297mm;
            margin-left: 15mm;
            padding: 3mm;
            page-break-after: always;
            width: 225mm;
        }
        div.year_month {
            display: inline-block;
            height: 70mm;
            margin-left: 7mm;
            margin-top: 8mm;
            text-align: center;
            width: 68mm;
        }
        div.year_month div.title {
            border-bottom: 1px #000 solid;
            display: inline-block;
            font-size: 6mm;
            height: 7mm;
            text-align: center;
            width: 100%;
        }
        div.year_month a div {
            display: inline-block;
            font-size: 4mm;
            font-weight: bold;
            height: 5mm;
            padding: 2mm;
            text-align: center;
            vertical-align: middle;
            width: 6mm;
        }
        div.year_month a div.weekend {
            color: #aaa;
        }
        div.year_month a div {
            position: relative;
        }
        div.year_month a div div.circled_single,
        div.year_month a div div.circled_double {
            color: #000;
            font-size: 3mm;
            height: 3mm;
            margin: 0;
            position: absolute;
            text-align: left;
            top: 0;
            vertical-align: top;
            z-index: 9;
        }
        div.day_notes {
            height: 200mm;
            padding: 3mm;
            width: 225mm;
        }
        div.day_tasks {
            height: 70mm;
            padding: 3mm;
            width: 140mm;
        }
        div.header div.year {
            padding: 5mm 0mm 2mm 20mm;
            width: 115mm;
        }
        div.header div.year a {
            font-size: 20mm;
        }
        div.header div.year div {
            border-left: 3px solid #000;
            display: inline-block;
            font-size: 5mm;
            margin-left: 2mm;
            padding-left: 2mm;
            width: 40mm;
        }
        div.tabs_top div.tab a {
            font-size: 5mm;
        }
        div.tabs_top div.tab {
            border: solid #ccc 1px;
            border-radius: 2mm 2mm 0mm 0mm;
            display: inline-block;
            margin-top: 12mm;
            padding: 3mm 2mm 1mm 2mm;
            text-align: center; width: 30mm;
        }
        div.header, div.header div.year, div.tabs_top, div.page, div.day_tasks, div.day_notes {
            float: left;
        }
        div.author, div.author a {
            font-size: 8mm;
        }
        ul {
            list-style-type: none;
        }
        ul li {
            border-bottom: 1px solid #eee;
            font-size: 12mm;
            width: 100%;
        }
        ul.circle li::before {
            content: "☐ ";
            font-size: 6mm;
        }
        div.year_month a div div.circled_single {
            border: 2px solid #000;
            border-radius: 40px 60px 40px 60px;
            left: 1mm;
            -webkit-transform: rotate(-15deg);
            width: 3mm;
        }
        div.year_month a div div.circled_double {
            border: 2px solid #000;
            border-radius: 40px 60px 40px 60px;
            left: 0;
            -webkit-transform: rotate(-15deg);
            width: 5mm;
        }

        div.day div.year_month {
            margin-top: 0mm;
        }
        
    </style>"##
        .to_owned()
}

fn generate_html_page_header(year: &str, month: u32, day: u32) -> String {
    "<div class=\"header\">".to_owned()
        + "<div class=\"year\">"
        + "<a href=\"#page_year\">"
        + &year
        + "</a>"
        + &if month == 0 {
            "".to_owned()
        } else {
            let date = get_date(year, month, day);
            "<div>".to_owned() + &date.format("Week %W<br>%A<br>%e %B").to_string() + "</div>"
        }
        + "</div>"
        + &generate_html_tabs_top()
        + "</div>"
}

fn generate_html_tabs_top() -> String {
    r##"
    <div class="tabs_top">
        <div class="tab calendar"><a href="#page_year">Calendar</a></div>
        <div class="tab tasks"><a href="#page_tasks">Tasks</a></div>
        <div class="tab notes"><a href="#page_notes">Notes</a></div>
    </div>
    "##
    .to_owned()
}

fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 && year % 100 == 0 && year % 400 == 0 {
        true
    } else if year % 4 == 0 && year % 100 != 0 && year % 400 == 0 {
        true
    } else {
        false
    }
}

fn number_of_days_in_month(year: &str, month: u32) -> u32 {
    if month == 2 && is_leap_year(year.parse::<i32>().unwrap()) {
        29
    } else {
        match month {
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }
}

fn get_date(year: &str, month: u32, day: u32) -> DateTime<Utc> {
    let naive_date: NaiveDateTime =
        NaiveDate::from_ymd(year.parse::<i32>().unwrap(), 1, 1).and_hms(1, 1, 1);
    if day > 0 {
        DateTime::<Utc>::from_utc(
            naive_date.with_month(month).unwrap().with_day(day).unwrap(),
            Utc,
        )
    } else {
        DateTime::<Utc>::from_utc(naive_date.with_month(month).unwrap(), Utc)
    }
}

fn generate_tiny_month_calendar(year: &str, month: u32, current_day: u32) -> String {
    let mut date = get_date(year, month, 0);
    let days_in_month = number_of_days_in_month(year, month);
    let mut spacer_days_at_front: usize = 0;
    let mut html: String = r##"<div class="year_month">"##.to_owned();
    html += "<div class=\"title\">";
    html += &date.format("%B").to_string();
    html += "</div>";
    for day in 1..=days_in_month {
        //set date
        date = get_date(year, month, day);
        let formatted_date = date.weekday();

        //add spacing at front of month if required
        if day == 1 {
            spacer_days_at_front = match formatted_date {
                chrono::Weekday::Mon => 0,
                chrono::Weekday::Tue => 1,
                chrono::Weekday::Wed => 2,
                chrono::Weekday::Thu => 3,
                chrono::Weekday::Fri => 4,
                chrono::Weekday::Sat => 5,
                chrono::Weekday::Sun => 6,
            };
            html += &"<a><div></div></a>".repeat(spacer_days_at_front);
        }

        //figure out weekends for colour change
        let weekend =
            if formatted_date == chrono::Weekday::Sat || formatted_date == chrono::Weekday::Sun {
                "weekend"
            } else {
                ""
            };

        let today =
            if current_day == day {
                if day < 10 {
                    r##"<div class="circled_single"></div>"##
                } else {
                    r##"<div class="circled_double"></div>"##
                }
            } else {
                ""
            };

        //create the html for the day link within the calendar month
        let link = "<a href=\"#day_".to_owned()
            + &month.to_string()
            + "_"
            + &day.to_string()
            + "\"><div class=\""
            + weekend
            + "\">"
            + today
            + &day.to_string()
            + "</div></a>";
        html += &link;
    }
    //add spacing at end of month if required
    html += &"<a><div></div></a>"
        .repeat(42 as usize - spacer_days_at_front as usize - days_in_month as usize);
    html += "</div>";
    html
}

fn generate_html_year(year: &str) -> String {
    let mut html: String = r##"<div id="page_year" class="year page" name="year">"##.to_owned();
    for month in 1..=12 {
        html += &generate_tiny_month_calendar(year, month, 0);
    }
    html += "</div>";

    generate_html_page_header(year, 0, 0) + &html
}

fn generate_html_days(year: &str) -> String {
    let mut html: String = "".to_owned();
    for month in 1..=12 {
        let days_in_month = number_of_days_in_month(year, month);
        for day in 1..=days_in_month {
            let day_html = generate_html_page_header(year, month, day)
                + r##"<div class="day page" name="day_"##
                + &month.to_string()
                + "_"
                + &day.to_string()
                + "\">"
                + &generate_tiny_month_calendar(year, month, day)
                + r##"<div class="day_tasks"><ul class="circle">"##
                + &"<li></li>".repeat(4)
                + "</ul></div>"
                + r##"<div class="day_notes"><ul>"##
                + &"<li>&nbsp;</li>".repeat(16)
                + "</ul></div>"
                + "</div>";
            html += &day_html;
        }
    }
    html
}

fn generate_html_tasks(year: &str) -> String {
    generate_html_page_header(year, 0, 0)
        + r##"<div class="tasks page" name="page_tasks"><ul class="circle">"##
        + &"<li></li>".repeat(22)
        + "</ul></div>"
}

fn generate_html_notes(year: &str) -> String {
    generate_html_page_header(year, 0, 0)
        + r##"<div class="notes page" name="page_notes"><ul>"##
        + &"<li>&nbsp;</li>".repeat(21)
        + "</ul></div>"
}

fn generate_html_author(year: &str) -> String {
    generate_html_page_header(year, 0, 0)
        + r##"<div class="author page">
    <p>Hello! If you are in need of next years planner, I have good news! This PDF and any year you need can be generated from this project:</p>
    <p><a href="https://github.com/craigmayhew/pdf-year-planner">https://github.com/craigmayhew/pdf-year-planner</a></p>
    <p>If you experience any problems, please raise a ticket describing what happened, what you expected to happen and what device you are using!</p>
    <p>This document was generated with Rust. Ideas and contributions welcome!</p>
    </div>"##
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // accept a single argument for year else default to the current year if it's absent
    let chosen_year = if args.len() > 1 && &args[1] != "" {
        args[1].clone()
    } else {
        let naive_date_time = Utc::now().naive_utc().format("%Y").to_string();
        naive_date_time
    };

    let title = chosen_year.to_string() + " Year Planner";
    let filename = chosen_year.to_string() + ".pdf";
    let error_pdf_save = "failed to save ".to_owned() + &filename;

    // generate html
    let html =
        r##"<!DOCTYPE html><html><head><meta charset="utf-8">"##
            .to_owned()
            + &generate_html_style()
            + "</head><body>"
            + &generate_html_year(&chosen_year)
            + &generate_html_days(&chosen_year)
            + &generate_html_tasks(&chosen_year)
            + &generate_html_notes(&chosen_year)
            + &generate_html_author(&chosen_year)
            + "</body></html>";

    // turn html into a pdf
    let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app
        .builder()
        .orientation(Orientation::Portrait)
        .page_size(PageSize::A4)
        .margin(Size::Millimeters(5))
        .title(&title)
        .build_from_html(&html)
        .expect("failed to build pdf");

    //save the pdf
    pdfout.save(&filename).expect(&error_pdf_save);
    Ok(())
}

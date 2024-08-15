mod models;
use models::{DateFormat, HistoricalTimeline, Opt};
//use std::path::PathBuf;
use structopt::StructOpt;
fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);

    let file_path = String::from("bible_timeline.yaml");
    
    let timeline = format_data(get_data(file_path));

    let start = opt.start_date_end_date.0;
    let end = opt.start_date_end_date.1;

    create_svg(timeline.clone(), start.get_data(), end.get_data());

    //println!("Extracted strings: {:?}", timeline.HistoricalTimeline.events[0]);
    for event in timeline.HistoricalTimeline.events.iter() {
        //print!("{:?}", event.date_precision);
    }
}

//Parses YAML file into structure defined in models.rs, and sorts events based on date
fn format_data(data: String) -> HistoricalTimeline {
    let mut data: HistoricalTimeline = serde_yaml::from_str(&data).expect("Failed to parse data.");

    for (_, event) in data.HistoricalTimeline.events.iter_mut().enumerate() {
        if event.date.contains("BC") {
            event.date = event.date.replace(" BC", "");
            event.date = format!("{}{}", '-', event.date);
        }
           
        else {
            event.date = event.date.replace(" AD", "");
        }

        if let Ok(date_int) = event.date.parse::<i32>(){
                event.date_int = Some(date_int);
        }
        //let format = DateFormat::new(event.date_precision.clone());
        //println!("{:?}",format);
    }

    data.HistoricalTimeline.events.sort_by(|a, b| {
        let date_cmp = a.date.cmp(&b.date);
        if date_cmp == std::cmp::Ordering::Equal {
            a.event_id.cmp(&b.event_id)
        } else {
            date_cmp
        }
    });

    data
}

use std::fs::File;
use std::io::Read;
//Reads YAML file into string and returns
fn get_data(file_path:String) -> String {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error attempting to read file.").to_string();
    //println!("{:?}", contents);
    contents
}

use svg::node::element::{/*Circle, Rectangle, */Text, Line};
use svg::Document;
//Draws SVG timeline
fn create_svg(timeline: HistoricalTimeline, min: i32, max: i32) {
    let mut document = Document::new()
        .set("width", 1000)
        .set("height", 1000)
    .add(Line::new()
        .set("x1", 500)         
        .set("y1", 0)         
        .set("x2", 500)        
        .set("y2", 1000)        
        .set("stroke", "black") 
        .set("stroke-width", 2)
        );

            let mut iter = 50;
            for event in timeline.HistoricalTimeline.events.iter() {
                if event.date_int.unwrap() > min && event.date_int.unwrap() < max {
                    document = document
                    .add(Text::new(event.event_id.to_string() + ": " + &event.title + ": " + &event.date)
                    .set("x", 505)
                    .set("y", iter)
                    .set("font-family", "Verdana")
                    .set("font-size", 20)
                    .set("fill", "black"));
                    iter += 50;
                }
            }
    // Save SVG to file
    svg::save("output.svg", &document).expect("Failed to save SVG file");
}


//How to create SVG Shapes 
/*
let mut document = Document::new()
        .set("width", 1000)
        .set("height", 1000)
        .add(Rectangle::new()
            .set("x", 10)
            .set("y", 10)
            .set("width", 100)
            .set("height", 100)
            .set("fill", "blue"))
        .add(Circle::new()
            .set("cx", 150)
            .set("cy", 150)
            .set("r", 50)
            .set("fill", "red"));
*/
use scraper::{Html, Selector};
use std::fs;

fn main() {
    let filename = "recipes/recipe1.html";
    let html = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let fragment = Html::parse_document(&html);
    // println!("{:?}", document);

    let steps_selector = Selector::parse(r#"div[id="steps"]"#).unwrap();
    let step_selector = Selector::parse(r#"p[class="step_text"]"#).unwrap();
    let steps_to_iter = fragment.select(&steps_selector).next().unwrap();
    // let a_selector = Selector::parse("a").unwrap();

    // let text = steps_to_iter.text().collect::<Vec<_>>();
    // for t in text {
    //     println!("{}", t);
    // }

    let steps = steps_to_iter
        .select(&step_selector)
        .map(|step| step.text().collect::<String>())
        .collect::<Vec<_>>();

    // println!("{:?}", steps);

    for step in steps {
        println!("{}", step.trim());
    }

    // // println!("{:?}", steps);
    // for step in steps {
    //     println!("{}", step);
    // }
}

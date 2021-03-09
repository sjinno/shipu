// use reqwest::{self, StatusCode};
use scraper::{Html, Selector};

// const URL: &str = "https://cookpad.com/recipe/";

// use std::io::Write;
// // fn main() {
// //     let mut file = std::fs::File::create("data.txt").expect("create failed");
// //     file.write_all("Hello World".as_bytes()).expect("write failed");
// //     file.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");
// //     println!("data written to file" );
// //  }

use std::io::Read;
// // fn main(){
// //    let mut file = std::fs::File::open("data.txt").unwrap();
// //    let mut contents = String::new();
// //    file.read_to_string(&mut contents).unwrap();
// //    print!("{}", contents);
// // }

#[derive(Debug, Default)]
struct Context {
    title: String,
    photo_url: String,
    ingredients: Vec<(String, String)>,
    steps: Vec<String>,
}

// fn get_recipe_context(url: &str) -> Result<Context, Box<dyn std::error::Error>> {
//     let mut ctx = Context::default();
//     let res = reqwest::blocking::get(url)?;
//     match res.status() {
//         StatusCode::OK => {
//             let body = res.text()?;
//             let fragment = Html::parse_fragment(&body);
//             ctx.title = get_title(&fragment);
//             ctx.photo_url = get_photo_url(&fragment);
//             println!("{:?}", ctx);
//         }
//         _ => println!("Uh oh, the link may be broken :("),
//     }
//     Ok(ctx)
// }

fn get_recipe_context(contents: &str) {
    let mut ctx = Context::default();
    let fragment = Html::parse_fragment(contents);
    ctx.title = get_title(&fragment);
    ctx.photo_url = get_photo_url(&fragment);
    // println!("{:?}", ctx);
    ctx.ingredients = get_ingredients(&fragment);
    ctx.steps = get_steps(&fragment);
    // println!("{:?} {}", ctx.steps, ctx.steps.len());
}

fn get_title(fragment: &Html) -> String {
    let title_selector = Selector::parse(r#"div[id="recipe-title"]"#).unwrap();
    let h1_selector = Selector::parse("h1").unwrap();
    fragment
        .select(&title_selector)
        .next()
        .unwrap()
        .select(&h1_selector)
        .next()
        .unwrap()
        .inner_html()
        .trim()
        .to_string()
}

fn get_photo_url(fragment: &Html) -> String {
    let photo_selector = Selector::parse(r#"div[id="main-photo"]"#).unwrap();
    let img_selector = Selector::parse("img").unwrap();
    fragment
        .select(&photo_selector)
        .next()
        .unwrap()
        .select(&img_selector)
        .next()
        .unwrap()
        .value()
        .attr("src")
        .unwrap()
        .to_string()
}

fn get_ingredients(fragment: &Html) -> Vec<(String, String)> {
    let ingredients_selector = Selector::parse(r#"div[id="ingredients_list"]"#).unwrap();
    let name_selector = Selector::parse(r#"span[class="name"]"#).unwrap();
    let amount_selector = Selector::parse(r#"div[class="ingredient_quantity amount"]"#).unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let ingredients_to_iter = fragment.select(&ingredients_selector).next().unwrap();
    let names = ingredients_to_iter
        .select(&name_selector)
        .map(|name| {
            if let Some(n) = name.select(&a_selector).next() {
                n.inner_html()
            } else {
                name.inner_html()
            }
        })
        .collect::<Vec<_>>();
    let amounts = ingredients_to_iter
        .select(&amount_selector)
        .map(|amount| amount.inner_html())
        .collect::<Vec<_>>();
    let zipped = names.iter().zip(amounts.iter());
    zipped.map(|(i, j)| (i.clone(), j.clone())).collect()
}

fn get_steps(fragment: &Html) -> Vec<String> {
    let steps_selector = Selector::parse(r#"div[id="steps"]"#).unwrap();
    let step_selector = Selector::parse(r#"p[class="step_text"]"#).unwrap();
    let steps_to_iter = fragment.select(&steps_selector).next().unwrap();
    let a_selector = Selector::parse("a").unwrap();
    steps_to_iter
        .select(&step_selector)
        .map(|step| {
            if let Some(s) = step.select(&a_selector).next() {
                s.inner_html()
            } else {
                step.inner_html()
            }
        })
        .collect()
}

fn main() {
    // let url = "https://cookpad.com/recipe/1847041";
    // let res = reqwest::blocking::get(url)?;
    // let body = res.text()?;
    // let mut file = std::fs::File::create("data.txt").expect("create failed");
    // file.write_all(body.as_bytes()).expect("write failed");
    // Ok(())
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // print!("{}", contents);
    // get_recipe_context(url);
    get_recipe_context(&contents);
}

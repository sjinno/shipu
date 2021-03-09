use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use reqwest::{self, StatusCode};
use scraper::{Html, Selector};

#[pyclass]
#[derive(Debug, Default)]
struct Context {
    #[pyo3(get)]
    title: String,
    #[pyo3(get)]
    photo_url: String,
    #[pyo3(get)]
    ingredients: Vec<(String, String)>,
    #[pyo3(get)]
    steps: Vec<String>,
}

#[pyfunction]
fn get_recipe_context(url: &str) -> Context {
    let mut ctx = Context::default();
    let res = reqwest::blocking::get(url).unwrap();
    match res.status() {
        StatusCode::OK => {
            let body = res.text().unwrap();
            let fragment = Html::parse_fragment(&body);
            ctx.title = get_title(&fragment);
            ctx.photo_url = get_photo_url(&fragment);
            ctx.ingredients = get_ingredients(&fragment);
            ctx.steps = get_steps(&fragment);
            println!("Success!");
        }
        _ => println!("Uh oh, the link may be broken :("),
    }
    ctx
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

/// A Python module implemented in Rust.
#[pymodule]
fn page_minimizer(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Context>()?;
    m.add_function(wrap_pyfunction!(get_recipe_context, m)?)?;
    Ok(())
}

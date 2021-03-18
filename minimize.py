# std:
import os
import sys
# Third party libraries:
from bs4 import BeautifulSoup
from jinja2 import Environment, FileSystemLoader, select_autoescape
import requests
import page_minimizer


# Get templates ready:
env = Environment(
    loader=FileSystemLoader('./templates'),
    autoescape=select_autoescape(['html', 'xml']),
)
template = env.get_template('recipe_template.html')
index_temp = env.get_template('index.html')


def generate_recipe_page(ctx):
    output_file = ctx['title'] + '.html'
    with open(output_file, 'w') as f:
        f.write(template.render(ctx))


def update_recipe_list(title):
    with open('RECIPES', 'a') as f:
        f.write(title + '\n')


def get_recipe_list():
    with open('RECIPES', 'r') as f:
        recipes = f.read().split()
    return {'recipes': recipes}


def update_index_page(recipe_ls):
    with open('index.html', 'w') as f:
        f.write(index_temp.render(recipe_ls))


def main():
    ctx = page_minimizer.get_recipe_context(url)
    recipe_ctx = {
        'title': ctx.title,
        'photo_url': ctx.photo_url,
        'ingredients': ctx.ingredients,
        'steps': ctx.steps,
    }
    generate_recipe_page(recipe_ctx)
    update_recipe_list(ctx.title)
    recipe_ls = get_recipe_list()
    update_index_page(recipe_ls)


if __name__ == '__main__':
    # Get environment varibale:
    url = sys.argv[1]  # 1 https://cookpad.com/recipe/1847041 - メンチカツ
    print(f'{url}')
    main()

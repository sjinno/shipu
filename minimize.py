# std:
import os
import sys
# Third party libraries:
from bs4 import BeautifulSoup
from jinja2 import Environment, FileSystemLoader, select_autoescape
import requests


# Get templates ready:
env = Environment(
    loader=FileSystemLoader('./templates'),
    autoescape=select_autoescape(['html', 'xml']),
)
template = env.get_template('recipe_template.html')
index_temp = env.get_template('index.html')


# Scraping happens here.
def get_recipe_context(url):
    r = requests.get(url, auth=('user', 'pass'))
    if r.status_code == 200:
        print('Success!')
        soup = BeautifulSoup(r.content, 'html.parser')
        # CLEANING CLEANING CLEANING:
        title = soup.find(class_='recipe-title').text.strip()
        photo_url = soup.find(id='main-photo').img.attrs['src']
        # == CLEANING INGREDIENTS STARTS HERE ==
        names = [name.text for name in soup.find(
            id='ingredients').find_all(class_='name')]
        amounts = [
            amount.text for amount in soup.find(id='ingredients').find_all(class_='amount')
        ]
        ingredients = list(zip(names, amounts))
        # == CLEANING INGREDIENTS ENDS HERE ==
        steps = [step.text.strip()
                 for step in soup.find_all(class_='step_text')]
        # DONE CLEANING :)
        ctx = {
            'title': title,
            'photo_url': photo_url,
            'ingredients': ingredients,
            'steps': steps,
        }
        return ctx
    else:
        print('Something went wrong :(')


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
    recipe_ctx = get_recipe_context(url)
    generate_recipe_page(recipe_ctx)
    update_recipe_list(recipe_ctx['title'])
    recipe_ls = get_recipe_list()
    update_index_page(recipe_ls)


if __name__ == '__main__':
    # Get environment varibale:
    url = sys.argv[1]  # 1 https://cookpad.com/recipe/1847041 - メンチカツ
    print(f'{url}')
    main()

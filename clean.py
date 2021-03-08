# Third party libraries:
from bs4 import BeautifulSoup
from jinja2 import Environment, FileSystemLoader, select_autoescape
import requests

# std
import os
import sys


# Get environment varibale:
url = sys.argv[1]  # 1 https://cookpad.com/recipe/1847041 - メンチカツ
print(f'{url}')


# SCRAPE DATA AND PARSE IT:
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
steps = [step.text.strip() for step in soup.find_all(class_='step_text')]
# DONE CLEANING :)


# Get template ready:
env = Environment(
    loader=FileSystemLoader('./templates'),
    autoescape=select_autoescape(['html', 'xml']),
)
template = env.get_template('recipe_template.html')
context = {
    'title': title,
    'photo_url': photo_url,
    'ingredients': ingredients,
    'steps': steps,
}


output_file = title + '.html'
with open(output_file, 'w') as f:
    f.write(template.render(context))


with open('RECIPES', 'a') as f:
    f.write(title + '\n')

with open('RECIPES', 'r') as f:
    recipes = f.read().split()

# print(recipes)
recipe_ctx = {
    'recipes': recipes,
}

index_temp = env.get_template('index.html')
with open('index.html', 'w') as f:
    f.write(index_temp.render(recipe_ctx))

""" import os
import requests
from bs4 import BeautifulSoup


url = 'https://cookpad.com/recipe/1043219'  # スンドゥブ

r = requests.get(url, auth=('user', 'pass'))
print(r.status_code)

soup = BeautifulSoup(r.content, 'html.parser')
recipe = soup.find(id='recipe')


txt = recipe.txt

txt.replace('\n', ' ').split()


print(txt)


# with open('output.html', 'w', encoding='utf-8') as f:
#     f.write(str(recipe.prettify()))


# os.system('xdg-open output.html')
 """

# from jinja2 import Template

# template = Template('Hello, {{ name }}!')
# print(template.render(name='John Doe'))


from jinja2 import Environment, PackageLoader, select_autoescape, FileSystemLoader
env = Environment(
    loader=FileSystemLoader('./templates'),
    autoescape=select_autoescape(['html', 'xml'])
)

template = env.get_template('mytemplate.html')

# print(template.render(the='variables', go='here'))

with open('out.html', 'w') as f:
    f.write(template.render(the='variables', go='here'))

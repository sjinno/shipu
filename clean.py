import os
import requests
from bs4 import BeautifulSoup


url = 'https://cookpad.com/recipe/1043219'  # スンドゥブ

r = requests.get(url, auth=('user', 'pass'))
print(r.status_code)

soup = BeautifulSoup(r.content, 'html.parser')
recipe = soup.find(id='recipe')

with open('output.html', 'w', encoding='utf-8') as f:
    f.write(str(recipe.prettify()))


os.system('xdg-open output.html')

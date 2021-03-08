## 食譜(shípǔ) is a program that scrapes data of your favorite recipe(s) from クックパッド and outputs the data you only need to see(no ads & no trackers) into a file.

I am still in the process of its design, but this may be useful because:

1. You will not have to wait for the recipe to load potentially forever if you are experiencing bad internet connection.

2. You will not have to worry about any ads or trackers.
3. You will not have to see all other unnecessary garbage.
4. You can customize it (`recipe_template.html`) the way you like.
5. You will be able to access to those recipes offline! :)

### Todo:

- [ ] Oraganize code into functions and classes.
- [ ] Add features: `recipe-with-photos` if available.
- [ ] Create `index.html` to list all the saved recipes for easy access.

### Note

> This should work on Linux/Unix systems, though I'm not sure if it's true because I haven't tested it. FYI, this program is developed on Fedora 33.

### Usage

1. If you have `pipenv` installed on your system, you should be able to `pipenv install` or alternatively `pipenv install -r ./requirements.txt` to install the required packages **after** cloning this repo with:

   ```
   git clone https://github.com/sjinno/shipu.git
   ```

   After that, simply go copy the url of your favorite recipe from クックパッド and run

   ```
   pipenv run python clean.py your-fav-reciepe-url
   ```

2. If you do not have `pipenv` installed, then you can use `virtualenv`.

   So, you would `virtualenv env`, `source ./env/bin/activate`, `pip install -r requirements,txt`, and run

   ```
   python clean.py your-fav-reciepe-url
   ```
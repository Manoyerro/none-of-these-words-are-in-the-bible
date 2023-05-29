import json
from urllib.request import urlopen


f = urlopen('https://raw.githubusercontent.com/thiagobodruk/bible/master/json/en_kjv.json')

data = json.load(f)

for i in data:
    print(i["name"])
    print(len(i["chapters"]))
    for chapter_num, chapter in enumerate(i["chapters"]):
        for verse_num, verse in enumerate(chapter):
            print(f"{i['name']} {chapter_num + 1}:{verse_num + 1} - {verse}")
            


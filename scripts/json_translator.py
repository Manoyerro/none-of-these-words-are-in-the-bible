import json
import re
from urllib.request import urlopen

sanitise_regex = r"[^\w*]"
f = urlopen('https://raw.githubusercontent.com/thiagobodruk/bible/master/json/en_kjv.json')
data = json.load(f)
words = {}

for i in data:
    for chapter_num, chapter in enumerate(i["chapters"]):
        for verse_num, verse in enumerate(chapter):
            for word in verse.split(" "):
                word = re.sub(sanitise_regex, "", word.title())
                if word not in words.keys():
                    words[word] = []
                words[word].append({
                    "book": i["name"],
                    "chapter": chapter_num + 1,
                    "verse": verse_num + 1
                })

json_string = json.dumps(words)
print("Produced JSON:\n", json_string)
print("Number of words:", len(words.keys()))
with open("words.json", "w") as out:
    out.write(json_string)
print("Written to words.json.")

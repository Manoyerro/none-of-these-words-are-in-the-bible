import json
from urllib.request import urlopen

f = urlopen('https://raw.githubusercontent.com/thiagobodruk/bible/master/json/en_kjv.json')
data = json.load(f)
words = {}

for i in data:
    for chapter_num, chapter in enumerate(i["chapters"]):
        for verse_num, verse in enumerate(chapter):
            for word in verse.split(" "):
                word = word.title()
                if word not in words.keys():
                    words[word] = []
                words[word].append({
                    "book": i["name"],
                    "chapter": chapter_num,
                    "verse": verse_num
                })

print("Produced JSON:\n", words)
print("Number of words:", len(words.keys()))

from genericpath import isfile
import json
# 0 2 4 5 37

for i in range(40):
    if not isfile(f"./courses/{i}/math.txt"):
        print(i)

with open("./chapter_infos.txt") as fp:
    file = json.load(fp)
    for dir in [0, 2, 4, 5, 37]:
        print(file[dir])

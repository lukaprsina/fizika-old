from genericpath import isfile
import json
# 0 2 4 5 37

dirs = []
for i in range(40):
    if not isfile(f"./courses/{i}/math.txt"):
        print(i)

with open("./chapter_infos.txt") as fp:
    file = json.load(fp)
    for dir in dirs:
        print(f"{dir}: {file[dir]}")

    print("\n")
    for count, info in enumerate(file):
        if info["heading"] == "2. Tlak":
            print(count)

from genericpath import isfile
import json
from sys import argv

with open("./chapter_infos.txt") as fp:
    file = json.load(fp)

    print("\n")
    """ for count, info in enumerate(file):
        if info["heading"] == "2. Tlak":
            print(count) """
    print(file[int(argv[1])])

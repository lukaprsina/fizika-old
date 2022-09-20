with open("./courses/9/page_19.html", "r+") as fp:
    html = fp.read()
    result = html.replace('href="#resevanjeAVTOosi1"', 'href="#94c451e0d35ffc9530e5b98660250ae0"')
    fp.seek(0)
    fp.write(result)
    fp.truncate()

with open("./courses/27/page_103.html", "r+") as fp:
    html = fp.read()
    result = html.replace('href="#resevanjevezaveCvec1"', 'href="#897a79036e984377a709c192890cc547"', 1)
    result = result.replace('href="#resevanjevezaveCvec1"', 'href="#6fb1b7ccd9db4229a0982b54c02f2898"', 1)
    fp.seek(0)
    fp.write(result)
    fp.truncate()

with open("./courses/27/page_104.html", "r+") as fp:
    html = fp.read()
    result = html.replace('id="16c24c5bcf164994d97e797d0e801727"', 'id="897a79036e984377a709c192890cc547"')
    fp.seek(0)
    fp.write(result)
    fp.truncate()

with open("./courses/27/page_105.html", "r+") as fp:
    html = fp.read()
    result = html.replace('id="16c24c5bcf164994d97e797d0e801727"', 'id="6fb1b7ccd9db4229a0982b54c02f2898"')
    fp.seek(0)
    fp.write(result)
    fp.truncate()

with open("./courses/29/page_9.html", "r+") as fp:
    html = fp.read()
    result = html.replace('href="#resevanjeMOCvrovalke1"', 'href="#4b5c16ef569c72e06c764001bbe69ed4"')
    fp.seek(0)
    fp.write(result)
    fp.truncate()
    
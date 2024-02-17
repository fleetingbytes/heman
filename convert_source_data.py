#!/usr/bin/env python

import csv
import re

regex = re.compile(r"RFC(?P<rfc>\d+)(, Section (?P<section>\d+\.\d+\.\d+))?")

with open("http-status-codes-1.csv", encoding="utf-8") as data:
    reader = csv.reader(data)
    for row in reader:
        if row[2]:
            link = ""
            if match := regex.search(row[2]):
                d = match.groupdict()
                rfc = d.get("rfc")
                link = f"https://www.rfc-editor.org/rfc/rfc{rfc}.html"
                if section := d.get("section", ""):
                    link += f"#section-{section}"
                link = f', "{link}"'
            print(f'({row[0]}, "{row[1]}", "{row[2]}"{link}),')

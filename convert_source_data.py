#!/usr/bin/env python

import csv

with open("http-status-codes-1.csv", encoding="utf-8") as data:
    reader = csv.reader(data)
    for row in reader:
        if row[2]:
            print(f'({row[0]}, "{row[1]}", "{row[2]}"),')

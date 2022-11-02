#!/usr/bin/env python3

# A Converter of weights.json to use ICU4X's data since the keys have to be sorted.

import json

def main():
    with open('weights.json', 'r') as read_file:
        json_data = json.load(read_file)
        with open('converted_weights.json', 'w') as write_file:
            json.dump(json_data, write_file, sort_keys=True)

if __name__ == "__main__":
    main()

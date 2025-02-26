#!/usr/bin/env python3

import json
import sys

def main():
    filename = sys.argv[1]

    with open(filename, "r") as f:
        proof = json.load(f)
    print(proof)

if __name__ == "__main__":
    main()
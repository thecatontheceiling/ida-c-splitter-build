import json

function_definitions = json.load(open("intermediate.json"))["function_definitions"]
proportion = 1000

with open("signatures_sample.txt", "w") as f:
    for i, function_definition in enumerate(function_definitions):
        if i % proportion == 0:
            f.write(function_definition["signature"] + "\n")

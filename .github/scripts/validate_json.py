import json

json_file ='target/json_all_options.json'

with open(json_file, 'r', encoding='cp850') as file:
    line_number = 0
    for line in file:
        line_number += 1
        try:
            data = json.loads(line)

            # We only check that there is no added column because output is full of expected missing values
            expected_columns = {'id', 'first_name', 'last_name', 'company_email', 'created', 'connection', 'code', 'code_between_5_and_15', 'is_subscribed', 'score', 'percentage', 'constant_string', 'constant_string_list', 'constant_string_weighted_list', 'external_data'}
            if not all(field in expected_columns for field in data):
                print(f"Issues with the colums in the json file line {line_number}")
                print("Expected:", expected_columns)
                print("Real:", data)
                exit(1)

        except json.JSONDecodeError as e:
            print(f"Failed to load JSON file")
            exit(1)

import pandas as pd

csv_file = 'target/csv_all_options.csv'

try:
    data = pd.read_csv(csv_file)
except Exception as e:
    print(f"Failed to load CSV file")
    exit(1)


expected_columns = {'id', 'first_name', 'last_name', 'company_email', 'created', 'connection', 'code', 'code_between_5_and_15', 'is_subscribed', 'score', 'percentage', 'constant_string', 'constant_string_list', 'constant_string_weighted_list', 'external_data'}
if not expected_columns.issubset(data.columns):
    print("Issues with the colums in the csv file")
    print("Expected:", expected_columns)
    print("Real:", data.columns)
    exit(1)
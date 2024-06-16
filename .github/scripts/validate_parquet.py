import pandas as pd

parquet_file ='target/parquet_all_options.parquet'

try:
    data = pd.read_parquet(parquet_file)
except Exception as e:
    print("Failed to load PARQUET file")
    exit(1)

expected_columns = {'id', 'first_name', 'last_name', 'company_email', 'created', 'connection', 'code', 'code_between_5_and_15', 'is_subscribed', 'score', 'percentage', 'constant_string', 'constant_string_list', 'constant_string_weighted_list', 'external_data'}
if not expected_columns.issubset(data.columns):
    print("Issues with the colums in the parquet file")
    print("Expected:", expected_columns)
    print("Real:", data.columns)
    exit(1)

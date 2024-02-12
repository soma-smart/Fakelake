Fakelake
---------
FakeLake is a command line tool that generates fake data from a YAML schema.

##### Example
Here is a YAML file that will generate 1 millions rows with 4 columns.
```yaml
columns:
  - name: id
    provider: Increment.integer
    start: 42
    presence: 0.8

  - name: first_name
    provider: Person.fname

  - name: company_email
    provider: Person.email
    domain: soma-smart.com

  - name: created
    provider: Random.Date.date
    format: "%Y-%m-%d"
    after: 2000-02-15
    before: 2020-07-17

info:
  output_name: all_options
  output_format: parquet
  rows: 1_000_000
```

[Click here](usage/create_your_yaml_file.md) to create your YAML file.
[Click here](usage/generate.md) to generate from a YAML file.
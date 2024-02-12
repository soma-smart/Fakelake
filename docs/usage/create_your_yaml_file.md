Create your YAML file
--------------------

A YAML file for Fakelake is composed of two parts:

#### Columns
A list of columns with a name, a provider, provider's parameters and [options](../columns/options.md).<br/>
[Click here](../columns/providers/index.md) for the list of available providers.

Example of a file with one column:
```yaml
columns:
  - name: unique_id
    provider: Increment.integer
    start: 100
```

#### Info
To setup the generated file, see [here](../output/parameters.md).

Example of a parquet file of 10 million rows:
```yaml
info:
  output_name: generated_file
  output_format: parquet
  rows: 10_000_000
```

#### Example
```yaml
columns:
  - name: unique_id
    provider: Increment.integer
    start: 100

info:
  output_name: generated_file
  output_format: parquet
  rows: 10_000_000
```

That's it ! This is enough to generate a parquet file.

Next step, generate it.
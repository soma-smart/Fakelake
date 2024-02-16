Output parameters
--------------

### Generate file name
To change the name of the generated file, use output_name
```yaml
info:
  output_name: generate_file_name
```

### Format
To choose the format of the generated file, use output_format.
##### Parquet
```yaml
info:
 output_format: parquet
```

##### CSV
```yaml
info:
 output_format: csv
```

### Rows
To choose the number of rows in the generated file, use rows.
```yaml
info:
 rows: 1000000
```
It can also be written with delimiters for readibilty.
```yaml
info:
 rows: 1_000_000
```
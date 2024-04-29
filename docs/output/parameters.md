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
 delimiter: ','
```
Default delimiter is ',' but you can specify any character.

##### JSON
```yaml
info:
 output_format: json
 wrap_up: false
```
By default, wrap_up is set to false.  
When wrap_up is set to false, each line into the result file is a json object but the whole file is not a valid json.  
When wrap_up is set to true, the whole file is a valid json, rows are wrapped up into an array.

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
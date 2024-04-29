Constant provider
-------

### external
```yaml
- name: external_data
  provider: Constant.external
  path: path/to/data.txt
```

This provider enables to load a list of strings from external data file. Each line of this external file is considered as a value. A value is randomly assigned for each line.  
The parameter **path** contains the path to the file. If file doesn't exist, a error is raised.  

[Options](../options.md) are also possible.

In this case, corrupted means random string value which is not into the file.

### string
#### unique value
```yaml
 - name: constant_as_string
   provider: Constant.string
   data: trout
```

#### list of values
```yaml
 - name: list_of_constants_as_string
   provider: Constant.string
   data: [trout, salmon, carp]
```

#### list of weighted values
```yaml
 - name: list_of_weighted_constants_as_string
   provider: Constant.string
   data: 
     - value: trout
     - value: salmon
       weight: 8
     - value: carp
```

Data value can be unique value, a list of values or a dictionnary.  
Integer, float or string can be specify into the configuration but the result will be stored as a string.
If a unique value is specified, all lines will have this value.  
If a list of values is specified, value will randomly assigned for each line.  
If a weighted list of values is specified, value will weighted randomly assigned for each line: for example is useful to generate data skewing.  
 
[Options](../options.md) are also possible.
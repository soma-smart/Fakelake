Increment provider
-------

### integer
```yaml
 - name: adding_one_to_integer
   provider: Increment.integer
   start: 100
   step: 2
```
Increment an integer by one each row.
It starts from the optional parameter **start**. Default is 0.
It increments by the optional parameter **step**. Default is 1.
 
[Options](../options.md) are also possible.
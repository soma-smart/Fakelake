Random provider
------

### Date
##### date
```yaml
 - name: created
   provider: Random.Date.date
   format: "%m-%d-%Y"
   after: 02-15-2000
   before: 07-17-2020
```
Create a random date with:

- an optional parameter **format**. Default is "%Y-%m-%d"
- an optional parameter **after** as a lower boundary. It should follow the **format** parameter. Default is 1980-01-01
- an optional parameter **before** as a upper boundary. It should follow the **format** parameter. Default is 2000-01-01

[Options](../options.md) are also possible.

### Number
##### i32
```yaml
 - name: score
   provider: Random.Number.i32
   min: -100
   max: 100
```
Create a random 32 bits integer with:

- an optional parameter **min**. Default is the minimum 32bits integer.
- an optional parameter **max**. Default is the maximum 32bits integer.

[Options](../options.md) are also possible.

### String
##### alphanumeric
```yaml
 - name: string_code
   provider: Random.String.alphanumeric
```
Create a random string of length 10, with only Alphanumerics characters.

[Options](../options.md) are also possible.
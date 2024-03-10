Random provider
------

### Boolean
```yaml
 - name: is_subscribed
   provider: Random.bool
```
Create a random boolean.

[Options](../options.md) are also possible.

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

##### datetime
```yaml
 - name: connection
   provider: Random.Date.datetime
   format: "%m-%d-%Y %H-%M-%S"
   after: 02-15-2000 12:01:01
   before: 07-17-2020 15:06:06
```
Create a random datetime with:

- an optional parameter **format**. Default is "%Y-%m-%d %H:%M:%S"
- an optional parameter **after** as a lower boundary. It should follow the **format** parameter. Default is 1980-01-01 12:00:00
- an optional parameter **before** as a upper boundary. It should follow the **format** parameter. Default is 2000-01-01 12:00:00

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
   length: 5..15
```
Create a random string, with only Alphanumerics characters.

- an optional parameter **length** to specify the length of the string. This parameter can be a range `5..15` or a constant `8`. Default is 10.

[Options](../options.md) are also possible.
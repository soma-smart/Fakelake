Person provider
-------

### email
```yaml
 - name: email
   provider: Person.email
   domain: soma-smart.com
```
Create a random email with:

- random string of length 10 for the local-part
- optional **domain** parameter. Default is "example.com"

[Options](../options.md) are also possible.

In this case, corrupted means random string not in UTF8 format.

### fname
```yaml
 - name: first_name_in_top_1000_fr
   provider: Person.fname
```
Returns a random first name from top 1000 french list.

[Options](../options.md) are also possible.

In this case, corrupted means random string not in UTF8 format.

### lname
```yaml
 - name: last_name_in_top_1000_fr
   provider: Person.lname
```
Returns a random last name from top 1000 french list.

[Options](../options.md) are also possible.

In this case, corrupted means random string not in UTF8 format.
Optional Parameters
-------
On top of the provider's logic, you can apply options

### Presence
```yaml
 - name: column_name
   provider: Any.provider
   presence: 0.8
```
Adds a percentage of presence to the column: with missing values in the result.
Default value is **1**, or always present.
The parameter should be set between 0 and 1, otherwise it will be set to the closest.

In this example, 80% of the column will be generated, 20% will be missing.
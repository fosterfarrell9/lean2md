This is a test file for markers
The next line should be excluded from the output because it ends with the --+ marker:
But this line should be included despite having --#: --#
The next line is a normal line with
This should appear in output
And this is a normal line

```lean
def example2 := 43  --#
def example3 := 44
```
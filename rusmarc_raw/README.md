# RUSMARC raw

Crate to parse export data format of russian bibliographic systems.

## Format description.

Here is a format example:

```
#1: Some identificator
#100: Some data
#300: ^ASubfield 1^BSubfield 2^CSubfield 3
*****
#1: Some identificator
#100: Some data
#300: ^ASubfield 1^BSubfield 2^CSubfield 3
*****
...
```

Data consists of entries. Entries contain numbered fields. One field per line. Field with same number can appear multiple times in a single entry.

Field numbers and their content adhere to RUSMARC specification (http://www.rusmarc.info/2017/rusmarc/fields.htm)...

## And why is the crate created

...however, the data format **does not** adhere to RUSMARC data format (same as MARC). I failed to find any documentation of what even is this format.

Services like Znanium (https://znanium.ru/) and Lan (https://e.lanbook.com/) do export data in this format.

So, this package was created, since its easier to write my own parser, than to spend few days, trying to find a robust and working parser to a non-documented format.

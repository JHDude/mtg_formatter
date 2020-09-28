# mtg_formatter
A simple CLI program to convert DelverLens CSVs into Archidekt importable txt files

## Input Format
CSVs need to be formatted in the following way:
1) Header needs to be `QuantityX,Name,Edition code,Foil`
2) Body needs to have those parameters on each line. Example line: `"1x","Knight of the Ebon Legion","M20",""`

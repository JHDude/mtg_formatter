# mtg_formatter
A simple CLI program to convert DelverLens CSVs into Archidekt importable txt files

## Input Format
CSVs need to be formatted in the following way:
1) Header needs to be `QuantityX,Name,Edition code,Foil`
2) Body needs to have those parameters on each line. Example line: `"1x","Knight of the Ebon Legion","M20",""`

## To Build
1) You need to install Cargo for your system -> https://doc.rust-lang.org/cargo/getting-started/installation.html
2) You need to have git installed -> https://git-scm.com/book/en/v2/Getting-Started-Installing-Git
3) In a command prompt, you'll need to navigate to whatever folder you want this repository to live in.
4) Clone this repository via Git: 
  `git clone https://github.com/JHDude/mtg_formatter.git`
5) Navigate into the mtg_formatter folder via command prompt
6) Run: `cargo build --release`

## To Run
1) Make sure the code is built using the steps above
2) The CLI command can be run from the `target/release/mtg_formatter` folder like so: `target/release/mtg_formatter "path_to_csv.csv"
3) If you're importing a commander deck, you can designate your commander using the `-c` flag: `target/release/mtg_formatter -c "Commander McAwesomesauce" "path_to_csv.csv"

If the commander name you type in doesn't match the name of a card in the deck, the cards will still import. You just will have to mark the commander in the program manually.

## To Do:
1) Support more import formats
2) Support more export formats
3) Figure out some way to package the program so people don't have to build the code themselves - ideally it can be a clickable icon that opens up the prompt. But that may not be possible...

**CONTRIBUTIONS ARE WELCOME**
*This program is made during my spare time, so releases may be a little sporadic. But I'll do my best to respond to any issues raised, or code contributions.*

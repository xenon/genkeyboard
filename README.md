# Genkeyboard

Generates keyboard layouts for various languages targetting a variety of input backends.

## Solves what problem?
If you have multiple programs that can handle custom keyboard layouts, genkeyboard will be able
to generate the same keyboard layout in many different formats for these programs.

Genkeyboard can also be used to help make writing your keyboard layout easier by using tools
in the rust programming language rather than writing everything by hand.

## What does that mean?
Genkeyboard fundamentally does two things:

1. Generate a mapping from a sequence of input letters to an output string
    - ie: generate an input method for a language
2. Take this mapping and output it in the format of a common keyboard layout config
    - ie: save the input method as a config for your favourite keyboard layout program

## Example
The following example generates a greek polytonic keyboard layout that works with emacs.
```
genkeyboard generate --layout greek --format emacs-quail
```
## Usage
Each of the following subheaders is a subcommand in genkeyboard, alternatively try ```genkeyboard --help``` for more information.
### generate
Generates a keyboard layout in the specified format and prints it to the console (or saves it to a file)
#### Arguments
Go in the format: ```generate <OPTIONAL> <REQUIRED>```
##### REQUIRED
```
--layout <LAYOUT>
--format <FORMAT>
```
##### OPTIONAL
```
-o / --output-file <PATH>
```
### list-layouts
Lists the available keyboard layouts that can be generated by the program.
#### Currently supported layouts
- **greek**: Polytonic greek, phonetic layout.
- **latin**: Adds macrons and breve marks.
### list-formats
List the available output formats that can be generate by the program
#### Currently supported formats
- **list**: A human-readable mapping
- **emacs-quail**: An emacs-compatible keyboard layout
- **vim-fn**: A vim function which toggles the keyboard layout

# Extending Genkeyboard layouts and formats
This is by no means comprehensive and is only intended to give some intuition to those who are interested.
## Adding a new layouts
- see ```src/kbdlayout/latin.rs``` for an example.
### in general...
- Start in ```src/kbdlayout/<MYLAYOUT>.rs```
- Make a function which takes in a ```keyboard: &mut KbdWriter```
- Set the metadata of your new ```keyboard``` layout
- Create various mapping sections called ```KbdMap```s and write these sections to ```keboard```
### making it accessible by command line args
- Add a new enum entry ```<MYLAYOUT>``` to the ```Layout``` enum in  ```src/kbdlayout/mod.rs```
- In ```src/main.rs``` make the match statement on ```args.layout``` call your new layout function.
## Adding a new format
- see ```src/kbdwriter/list.rs``` for an example.
### in general...
- Start in ```src/kbdwriter/<MYFORMAT>.rs```
- Create a new struct with a RefCell to a KbdWriter
- Implement Display for the struct and use the KbdWriter
    - Print the metadata in a suitable way for your format
    - Print each mapping section in a suitable way for your format
### making it accessible by command line args
- Add a new enum entry ```<MYFORMAT>``` to the ```Format``` enum in  ```src/kbdwriter/mod.rs```
- In ```src/main.rs``` make the match statement on ```args.format``` format with your new struct.

# License
- GPL 3.0 or later
- See LICENSE
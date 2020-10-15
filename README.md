# Dictionary Fixing Utilty
dict-fixer is a CLI tool for removing ```'``` from an existing dictionary.  It will remove the full
word that contains the character ```'```.
## Expected input
The input file is expected to be a newline (```\n```) delimited file containing english words.  It will
produce a reduced version of the input file that omits all words that contain a ```'``` character.
# CLI usage
## Normal usage
Call ```dict-fixer <input dictionary> <output dictionary>``` which will:
1. Read in ```input dictionary```
2. Modify the data from ```input dictionary```, removing words with the ```'``` character
3. Write the modified data to ```output dictionary```
## Help function
Call ```dit-fixer -h``` which will print the help message.
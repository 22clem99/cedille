# Reserved keywords

'égal' 'plus' 'moins' 'fois' 'divisé' 'modulo' 'et' 'ou' 'non' 'plus' 'grand' 'petit' 'tant' 'que' 'faire' 'alors' 'vrai' 'faux' 'flotant' 'entier' 'chaîne' 'différent' 

*Kept for future features*
'entrée' 'sortie' 'fonction' 'structure'

# Identifier

LETTER = 'a' + ... + 'z' + 'A' + ... + 'Z' + 'é' + 'É' + 'è' + 'È' + 'ê' + 'Ê' + 'ë' + 'Ë' + 'à' + 'À' + 'â' + 'Â' + 'ä' + 'Ä' + 'ù' + 'Ù' + 'ü' + 'Ü' + 'û' + 'Û' + 'î' + 'Î' + 'ï' + 'Ï' + 'ç' + 'Ç'

DIGIT = '0' + ... + '9'

IDENT = (LETTER + '-')(LETTER + DIGIT + '-')*

# Symbols

'('  ')'  ':'  '.'  ','  '!'  '?'  ';'  '"'  '/'  '{'  '}'  '+'  '-'  '|'

# Integers
POSITIVE_DIGIT = '1' + ... + '9'

INT = '-' + '0' + POSITIVE_DIGIT DIGIT*

# Float
NUM = POSITIVE_DIGIT DIGIT+

SIGN = '+' + '-' + ''

DEC = NUM ',' NUM

FLOAT = SIGN DEC

# String
STRING_CAR = all characters

STRING            = '"' (STRING_CAR)* '"'

MULTI_LINE_STRING = '"' (STRING_CAR + EOL)* '"'

# Comments

Monoline : **My comment** | *code*
Multiline : Not yet define
Module
    = Import* Declaration*

Import
    = 'import' ('pkg' | 'ext')? STRING 'as' ImportAlias 'exposing' { IDENTIFIER (',' IDENTIFIER)* }
    | 'import' ('pkg' | 'ext')? STRING 'as' ImportAlias
    | 'import' ('pkg' | 'ext')? STRING 'exposing' { IDENTIFIER (',' IDENTIFIER)* }
    | 'import' ('pkg' | 'ext')?

ImportAlias
    = IDENTIFIER (. IDENTIFIER)*

Declaration
    = 'pub'? 'ext' IDENTIFIER (':' Type)?
    | 'pub'? 'let' IDENTIFIER (':' Type)? '=' Expression
    | 'run' Expression
    | 'pub'? 'ext' 'type' IDENTIFIER
    | 'pub'? 'type' IDENTIFIER '=' Type

Expression
    = Expression '.' IDENTIFIER ('.' IDENTIFIER)*
    | Expression Expression+
    | Expression 'as' Type
    | { BlockDeclaration* 'ret' Expression }
    | 'if' Expression 'then' Expression 'else' Expression
    | IDENTIFIER ('.' IDENTIFIER)*
    | '_'
    | Expression '+'  Expression
    | Expression '-'  Expression
    | Expression '*'  Expression
    | Expression '/'  Expression
    | Expression '%'  Expression
    | Expression '^'  Expression
    | Expression '==' Expression
    | Expression '!=' Expression
    | Expression '<'  Expression
    | Expression '<=' Expression
    | Expression '>'  Expression
    | Expression '>=' Expression
    | Expression '&&' Expression
    | Expression '||' Expression
    | Expression '::' Expression
    | Expression '++' Expression
    | Expression '|>' Expression
    | Expression '>>' Expression
    | Pattern+ '=>' Expression
    | STRING
    | NUMBER
    | '[' Expression (',' Expression)* ']'
    | '{' RecordField (',' RecordField)* '}'
    | '()'
    | '#' IDENTIFIER Expression*
    | 'where' Expression ('is' Pattern ('if' Expression)? '=>' Expression)+
    | '(' Expression ')'

BlockDeclaration
    = 'let' IDENTIFIER (':' Type)? '=' Expression
    | 'run' Expression

Pattern
    = '[' Pattern (',' Pattern)* (',' '...' IDENTIFIER)? ']'
    | '{' RecordPattern (',' RecordPattern)* (',' '...' IDENTIFIER)? '}'
    | NUMBER
    | STRING
    | '@' IDENTIFIER Pattern*
    | '#' IDENTIFIER Pattern*
    | '_'
    | IDENTIFIER

RecordPattern
    = IDENTIFIER ':' Pattern
    | IDENTIFIER

RecordField
    = IDENTIFIER ':' Expression
    | IDENTIFIER

Type
    = Type '->' Type
    | Type '|' Type
    | Type Type+
    | '#' IDENTIFIER Type*
    | '{' IDENTIFIER ':' Type (',' IDENTIFIER ':' Type)* '}'
    | '(' Type ')'
    | '*'

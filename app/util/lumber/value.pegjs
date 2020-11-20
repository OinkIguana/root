{
    const { Struct, Variable } = window;
}

start = pattern

_ = " "*
__ = " "+

pattern
    = literal
    / struct
    / name:$variable { return new Variable(name) }
    / list
    / record

literal
    = pattern:$decimal { Number.parseFloat(pattern) }
    / pattern:$integer { Number.parseInt(pattern, 10) }
    / string

decimal = integer "." integer
integer = [0-9]+
string  = '"' contents:$('\\"' / [^'"'])* '"' { return contents }

struct
    = name:$atom _ '(' _ contents:pattern _ ')' { return new Struct(name, contents) }
    / name:$atom _ contents:record { return new Struct(name, contents) }
    / name:$atom _ contents:list { return new Struct(name, contents) }
    / name:$atom { return new Struct(name) }

list = '[' _ contents:list_entries? _ ']' { return contents || [] }
list_entries = head:pattern (_ "," _ tail:pattern)* { return [head, ...tail] }

record = '{' _ contents:fields? _ '}' { return contents || {} }
fields = head:field (_ "," _ tail:field)* { return Object.fromEntries([head, ...tail]) }
field = key:$atom _ ":" _ val:pattern { return [key, val] }

atom = [a-z] [A-Za-z0-9_]*
variable = [A-Z] [A-Za-z0-9_]*

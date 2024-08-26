Regras gramaticais

[] - NÃO TERMINAL
() - TERMINAL

| terminal   | class |
| ---------- | ----- |
| {          | 0     |
| }          | 1     |
| class      | 2     |
| interface  | 3     |
| abstract   | 4     |
| concrete   | 4     |
| extends    | 5     |
| implements | 5     |
| public     | 6     |
| protected  | 6     |
| private    | 6     |
| static     | 7     |
| local      | 7     |
| final      | 8     |
| base       | 8     |
| int        | 9     |
| float      | 9     |
| double     | 9     |
| char       | 9     |
| void       | 9     |
| id         | 10    |
| ;          | 11    |
| (          | 12    |
| )          | 13    |
| [          | 14    |
| ]          | 15    |
| ,          | 16    |
| =          | 17    |
| +          | 18    |
| -          | 18    |
| \*         | 18    |
| /          | 18    |
| %          | 18    |
| <          | 19    |
| >          | 19    |
| <=         | 19    |
| >=         | 19    |
| ==         | 19    |
| !=         | 19    |
| &&         | 20    |
| new        | 24    |
| return     | 25    |
| break      | 25    |
| continue   | 25    |
| if         | 26    |
| else       | 27    |
| while      | 28    |
| for        | 29    |
| switch     | 30    |
| case       | 31    |
| default    | 32    |
| true       | 33    |
| false      | 33    |
| null       | 33    |
| this       | 34    |
| super      | 34    |
| $          | 35    |
| :          | 36    |
| @          | 37    |
| '          | 38    |
| "          | 39    |
| ++         | 40    |
| --         | 40    |
| \n         | 41    |
| \t         | 41    |
| ' '        | 41    |

[] - NAO TERMINAL
() - TERMINAL

1. Program -> Class Program | ε
2. Class -> { ClassBody }
3. ClassBody -> ClassMember ClassBody | ε
4. ClassMember -> (Class | Interface | Abstract | Concrete)
5. Class -> class id

### Rodando o projeto
``` bash
cargo run
```

PROGRAM -> [DECLARATION] [DECLARATIONS]

[DECLARATION] [DECLARATIONS] -> [DECLARATION] [DECLARATIONS] || E

[DECLARATION] -> [STRUCT] [ID] [INHERITANCE] ({) [ITEM_DECLS] (})

command, value e atrib, var, exp (item_decl -> atrib_decl)
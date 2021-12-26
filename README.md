# Swordfish

Swordfish is a preprocessor language designed for minecraft datapack functions (.mcfunction files).

# Features

Currently, the language supports macro definitions, formatted macro definitions, importing definition files and repeating lines. 

## Macro definitions
A macro in swordfish is considered a string of text which is later replaced with something useful by the preprocessor. 

To define a macro, use a `=` followed by a name and its definition
```
=greet say Hello!
```
To use the macro, simply type it's name.
Once you run the preprocessor, this,
```
greet
```
becomes this
```
say Hello
```
This is called macro expansion. 

## Formatted macro definitions
Sometimes, you may whish to have macros inside of macros. For example you may have a defintion that summons a certain entity, and some postition for it to be summoned
```
=summon_sheep execute at @a run summon minecraft:sheep
=pos ~1 ~ ~1
```
We may want to also seperate the summoning to make it reusable. For this we can use the formatted macro
```
=all_summon execute at @a run summon
=entity miencraft:sheep
=pos ~1 ~ ~1
#summon_sheep all_summon entity

summon_sheep pos
```

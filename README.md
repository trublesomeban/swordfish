# Swordfish

Swordfish is a preprocessor language designed for minecraft datapack functions (.mcfunction files).

# Installation

- [Linux binary](https://trublesomeban.github.io/swordfish/bin/linux/swordfish)
- [Windows executable](https://trublesomeban.github.io/swordfish/bin/windows/swordfish.exe)

**Make sure to add the location of the program to your path variable**

# Usage

To use the preprocessor, open you command line and `cd` into your datapack folder and (Assuming you have added the programs location to your path variable) type `swordfish {filename}`. This will run the preprocessor and output `{filename}.mcfunction` in the same folder.

# Features

Currently, the language supports macro definitions, formatted macro definitions, repeating lines and importing definition files.

## Macro definitions

A macro in swordfish is considered a string of text which is later replaced with its definition by the preprocessor.

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

Normally macros inside of other macros will not expand. The purpose of formatted macros is to be able to do that. The above code produces the following output

```
execute at @a run summon minecraft:sheep ~1 ~ ~1
```

## Line repetion

You can repeat a line by preceeding it with a `!` and the number of repetions like so

```
!2 execute as @a run effect give @s minecraft:instant damage 1 0 true
```

This results in

```
execute as @a run effect give @s minecraft:instant damage 1 0 true
execute as @a run effect give @s minecraft:instant damage 1 0 true
```

## Importing definition files

To import a file containing macros use a `-` followed by the file name

```
-lib.sf
```

_in lib.sf_

```
=greet say Hello!
```

All macros defined in the file will be bound and everything else will be ingored.

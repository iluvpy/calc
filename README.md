## calc
a small calculator written in rust without an AST nor lexing.

## Why
I made this project mostly for my self to learn the rust basics and to see if its possible to make something like this
without lexing nor parsing and if it would make the process of writing such a program simpler.

## Problems (for now)
negative numbers arent properly interpreted but work some of the time (with bigger numbers and multiple negative ones).

## Usage (not working for now)
the main usage will be `./calc [EXPRESSION]` where EXPRESSION is a string of numbers, operators and brackets <br>
form form a mathematical expression. Example: `./calc (12-43)*54*37/2*(32-1)`, Note that the '*' can be removed in case of (y)\*x
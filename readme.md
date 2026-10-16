# The Martix Console

The console contains multiple columns of. In each a column a sequence of green characters will be moving downwards.

## Console Attributes
* Consolse contains only green characters
* Characters (Character sequence - CS) will always be in a sequence, never single characters move downwards
* White spaces (Break sequence - BC) will also be moving downwards in sequences
* The characters in the CS can change value, but the length of the sequence will stay the same until its of the screen
* Characters in the CS will slowly fade out over time

## Controls
* `q`, `Esc` or `Ctrl-C` quits and restores the terminal
* The window is maximized at start when the terminal supports it

## Rendering
* Glyphs stay in place; each column has a moving head with its own speed (7-14 rows/s)
* The head is white and the trail fades from bright to dark green
* Truecolor is used when `COLORTERM` or Windows Terminal announces it, otherwise 256 colours

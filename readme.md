# Text animator
This program is a simple thing i made to animate text.

## How to use
The program takes 2 required arguments and 1 optional:
| Argument  | What it does |
| --------- | ------------ |
| Text      | Specifies what text the program should animate. There can be multiple pieces of text animated after each other and the pieces are separated by ", ". To have a new line in the text simply put "\n". |
| Framerate | How long the program should wait between frames in milliseconds. |
| Delay     | How long the delay between pieces of text should be in milliseconds, if not provided it will wait for a key press instead. |

Additionaly the program takes 1 flag:
| Flag       | What it does |
| ---------- | ------------ |
| -f, -frame | Whenether the program should put a frame around the text or not. |

## Examples
* text-animation.exe "fun times" 2 250 -f
* text-animation.exe "NOOOOOOOO, yes" 10 -f
* text-animation.exe "Knorb" 0

## Notes
The program can currently only render uppercase letters from the english alphabet (and the swedish alphabet), it will ignore any characters it cannot render.
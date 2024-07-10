# ASCII-ART

Converts a image into an ascii art that can be displayed into a terminal

## Usage
```
ascii_art file1[:opt=val,opt] ... filen[:opt=val,opt]

```

### options
color=
    rgb          : does not write a space in white, full color
    rgba         : writes a space when alpha is 0 , full color
    luma         : does not write a space in white, b&w
    luma-alpha   : writes a space when alpha is 0 , b&w
type=
    ascii        : tries to make a pretty ascii art
    braille      : writes a colored braille character 
    block        : writes pure space and a color as backround
    singeC       : C is the character that it will write when there is color
                   if an option does not exist like 'text' it will take the last letter

size=
    WidthxHeight : the size in charactes of the resulting text

out=
    path: the path where the image will be written


# Examples

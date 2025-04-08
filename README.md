# Image to String Translator
The program will take any image within the img folder and convert its RGB values into ASCII characters, producing a (possibly) readable image.

Note that the image name should be entered EXACTLY as it appears within the folder. Any whitespaces will be trimmed at the end of the image name, but not at the beginning. Do not forget to include the file extension (.png).

### Compatibility
The following file extensions have the known compatibilities:
| File Extension | Status |
|---|---|
| .png | <span style="color:green">Supported</span> |
| .jpg | <span style="color:green">Supported</span> |
Any other file extensions have unknown compatibility.

### Generating images
Although you can't generate images within this program in the current version, an upcoming refactor is coming to allow a new function that permits images to appear. Once I am out of class, I will include this.
However, using a pixel art maker (the example image, img/hello_world.png, was generated using [pixelart.com][https://www.pixilart.com/draw]), you can add generate an image by manually changing each RGB value. Note that the height is dependant on how many lines you have, and width should be equal to your longest line. Remember that each pixel stores 3 values; the first character is the R value, the second character is the B value, and the third character is the G value.

Below is a table of all ASCII values (TODO: validate)
| Dec | Hex  | Char | Rust Byte Literal |
|-----|------|------|-------------------|
| 0   | 0x00 | NUL  | `b'\0'`           |
| 1   | 0x01 | SOH  | `b'\x01'`         |
| 2   | 0x02 | STX  | `b'\x02'`         |
| 3   | 0x03 | ETX  | `b'\x03'`         |
| 4   | 0x04 | EOT  | `b'\x04'`         |
| 5   | 0x05 | ENQ  | `b'\x05'`         |
| 6   | 0x06 | ACK  | `b'\x06'`         |
| 7   | 0x07 | BEL  | `b'\x07'`         |
| 8   | 0x08 | BS   | `b'\x08'`         |
| 9   | 0x09 | TAB  | `b'\t'`           |
| 10  | 0x0A | LF   | `b'\n'`           |
| 11  | 0x0B | VT   | `b'\x0B'`         |
| 12  | 0x0C | FF   | `b'\x0C'`         |
| 13  | 0x0D | CR   | `b'\r'`           |
| 14  | 0x0E | SO   | `b'\x0E'`         |
| 15  | 0x0F | SI   | `b'\x0F'`         |
| 16  | 0x10 | DLE  | `b'\x10'`         |
| 17  | 0x11 | DC1  | `b'\x11'`         |
| 18  | 0x12 | DC2  | `b'\x12'`         |
| 19  | 0x13 | DC3  | `b'\x13'`         |
| 20  | 0x14 | DC4  | `b'\x14'`         |
| 21  | 0x15 | NAK  | `b'\x15'`         |
| 22  | 0x16 | SYN  | `b'\x16'`         |
| 23  | 0x17 | ETB  | `b'\x17'`         |
| 24  | 0x18 | CAN  | `b'\x18'`         |
| 25  | 0x19 | EM   | `b'\x19'`         |
| 26  | 0x1A | SUB  | `b'\x1A'`         |
| 27  | 0x1B | ESC  | `b'\x1B'`         |
| 28  | 0x1C | FS   | `b'\x1C'`         |
| 29  | 0x1D | GS   | `b'\x1D'`         |
| 30  | 0x1E | RS   | `b'\x1E'`         |
| 31  | 0x1F | US   | `b'\x1F'`         |
| 32  | 0x20 | SP   | `b' '`            |
| 33  | 0x21 | !    | `b'!'`            |
| 34  | 0x22 | "    | `b'"'`            |
| 35  | 0x23 | #    | `b'#'`            |
| 36  | 0x24 | $    | `b'$'`            |
| 37  | 0x25 | %    | `b'%'`            |
| 38  | 0x26 | &    | `b'&'`            |
| 39  | 0x27 | '    | `b'\''`           |
| 40  | 0x28 | (    | `b'('`            |
| 41  | 0x29 | )    | `b')'`            |
| 42  | 0x2A | *    | `b'*'`            |
| 43  | 0x2B | +    | `b'+'`            |
| 44  | 0x2C | ,    | `b','`            |
| 45  | 0x2D | -    | `b'-'`            |
| 46  | 0x2E | .    | `b'.'`            |
| 47  | 0x2F | /    | `b'/'`            |
| 48  | 0x30 | 0    | `b'0'`            |
| 49  | 0x31 | 1    | `b'1'`            |
| 50  | 0x32 | 2    | `b'2'`            |
| 51  | 0x33 | 3    | `b'3'`            |
| 52  | 0x34 | 4    | `b'4'`            |
| 53  | 0x35 | 5    | `b'5'`            |
| 54  | 0x36 | 6    | `b'6'`            |
| 55  | 0x37 | 7    | `b'7'`            |
| 56  | 0x38 | 8    | `b'8'`            |
| 57  | 0x39 | 9    | `b'9'`            |
| 58  | 0x3A | :    | `b':'`            |
| 59  | 0x3B | ;    | `b';'`            |
| 60  | 0x3C | <    | `b'<'`            |
| 61  | 0x3D | =    | `b'='`            |
| 62  | 0x3E | >    | `b'>'`            |
| 63  | 0x3F | ?    | `b'?'`            |
| 64  | 0x40 | @    | `b'@'`            |
| 65  | 0x41 | A    | `b'A'`            |
| 66  | 0x42 | B    | `b'B'`            |
| 67  | 0x43 | C    | `b'C'`            |
| 68  | 0x44 | D    | `b'D'`            |
| 69  | 0x45 | E    | `b'E'`            |
| 70  | 0x46 | F    | `b'F'`            |
| 71  | 0x47 | G    | `b'G'`            |
| 72  | 0x48 | H    | `b'H'`            |
| 73  | 0x49 | I    | `b'I'`            |
| 74  | 0x4A | J    | `b'J'`            |
| 75  | 0x4B | K    | `b'K'`            |
| 76  | 0x4C | L    | `b'L'`            |
| 77  | 0x4D | M    | `b'M'`            |
| 78  | 0x4E | N    | `b'N'`            |
| 79  | 0x4F | O    | `b'O'`            |
| 80  | 0x50 | P    | `b'P'`            |
| 81  | 0x51 | Q    | `b'Q'`            |
| 82  | 0x52 | R    | `b'R'`            |
| 83  | 0x53 | S    | `b'S'`            |
| 84  | 0x54 | T    | `b'T'`            |
| 85  | 0x55 | U    | `b'U'`            |
| 86  | 0x56 | V    | `b'V'`            |
| 87  | 0x57 | W    | `b'W'`            |
| 88  | 0x58 | X    | `b'X'`            |
| 89  | 0x59 | Y    | `b'Y'`            |
| 90  | 0x5A | Z    | `b'Z'`            |
| 91  | 0x5B | [    | `b'['`            |
| 92  | 0x5C | \    | `b'\\'`           |
| 93  | 0x5D | ]    | `b']'`            |
| 94  | 0x5E | ^    | `b'^'`            |
| 95  | 0x5F | _    | `b'_'`            |
| 96  | 0x60 | `    | `b'\``'           |
| 97  | 0x61 | a    | `b'a'`            |
| 98  | 0x62 | b    | `b'b'`            |
| 99  | 0x63 | c    | `b'c'`            |
| 100 | 0x64 | d    | `b'd'`            |
| 101 | 0x65 | e    | `b'e'`            |
| 102 | 0x66 | f    | `b'f'`            |
| 103 | 0x67 | g    | `b'g'`            |
| 104 | 0x68 | h    | `b'h'`            |
| 105 | 0x69 | i    | `b'i'`            |
| 106 | 0x6A | j    | `b'j'`            |
| 107 | 0x6B | k    | `b'k'`            |
| 108 | 0x6C | l    | `b'l'`            |
| 109 | 0x6D | m    | `b'm'`            |
| 110 | 0x6E | n    | `b'n'`            |
| 111 | 0x6F | o    | `b'o'`            |
| 112 | 0x70 | p    | `b'p'`            |
| 113 | 0x71 | q    | `b'q'`            |
| 114 | 0x72 | r    | `b'r'`            |
| 115 | 0x73 | s    | `b's'`            |
| 116 | 0x74 | t    | `b't'`            |
| 117 | 0x75 | u    | `b'u'`            |
| 118 | 0x76 | v    | `b'v'`            |
| 119 | 0x77 | w    | `b'w'`            |
| 120 | 0x78 | x    | `b'x'`            |
| 121 | 0x79 | y    | `b'y'`            |
| 122 | 0x7A | z    | `b'z'`            |
| 123 | 0x7B | {    | `b'{'`            |
| 124 | 0x7C | \|   | `b'|'`            |
| 125 | 0x7D | }    | `b'}'`            |
| 126 | 0x7E | ~    | `b'~'`            |
| 127 | 0x7F | DEL  | `b'\x7F'`         |

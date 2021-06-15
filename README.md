# CPO (CoPy at Offset) 1.0

### Example

This tool extracts data from a file between two hexadecimal offsets. See this example, we have this text file:

```txt
Forknife is a bad game
This meme is dead
```

Looking in a hex editor we can see that the word `game` is at hex `00000012` to `00000015`.

![Image of the hex editor](docs/hex.png)

So we can execute this command:

```bash
cpo test/text.txt 00000012 00000015
```

This will result in:

```txt
game
```

To output the result to a text file add ` > output.txt` to the command like this:

```bash
cpo test/text.txt 00000012 00000015 > output.txt
```

While this doesn't look very useful with such a small example, extracting huge chunks from point to point can be frustrating, thus this tool.
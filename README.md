![Logo Sheet](https://github.com/Lazylllama/SheetSmith/blob/main/example-logo.png)
<p align="center"><i>this image was created using sheetsmith 👀 (see /example directory)</i></p>

# SheetSmith
> [!IMPORTANT]
> In heavy development, will change alot!

A simple sprite sheet packer written in Rust with different interfaces (CLI, TUI, Web, GUI). It takes a directory of images and packs them into a single sprite sheet, along with metadata about the position and size of each sprite.

## CLI Usage

> [!NOTE]
> If you are on linux, you need to do `chmod +x sheetsmithcli` before you can run the command and prefix it with `./`, if you are on windows, add ".exe" after sheetsmithcli!`

> [!IMPORTANT]
> For the shipwrights/reviewers, if you arent one then you can disregard.
> Last time I forgot to mention the "examples" directory, so sorry for that, see the CLI examples below for a test command you can start with so you don't have to go to itch again...

**Options:**
```bash
Usage: sheetsmithcli [OPTIONS]

Options:
  -i, --input-dir <INPUT_DIR>  The input directory containing the images to pack [default: input]
  -o, --output <OUTPUT>        The output file for the packed sprite sheet [default: output.png]
      --no-color               Disable color in prints
  -s, --size <SIZE>            The size of the output sprite sheet [default: 1080x1080]
  -p, --padding <PADDING>      Padding between sprites in pixels [default: 0]
      --alg <ALGORITHM>        Algorithm to use for packing. Options: guillotiere [default: guillotiere]
  -t, --trim-transparent       Trim transparent pixels from the edges of images before packing This can help GREATLY reduce the size of the output sprite sheet and improve packing efficiency
  -a, --auto-size              Automatically find a good sheet size
  -d, --debug                  Debug mode: Print more often to find problematic images
  -h, --help                   Print help
  -V, --version                Print version
```

**Examples:**
> [!NOTE]
> Add ".exe" after sheetsmithcli if you are on windows!
- `sheetsmithcli -i example -o logo.png -s 896x256`
  - This will create the logo at the top of this readme using the sprites in `/example`. This folder is only there if you cloned the repository that is.
- `sheetsmithcli -o finished.png -s 2048x2048`
  - Create a sheet with the size **2048px** wide and **2048px** in height then fill that sheet with the images in the *input* directory.
- `sheetsmithcli -i sprites -o sheet.png -a`
  - Will automatically find a good sheet size for the images in the sprites directory, currently only recommended if you have square images, will be made better soon.



## Example Output Sheet
![Example Sheet](https://github.com/Lazylllama/SheetSmith/blob/main/example.png)

## Notable Dependencies
- anyhow [error handling]
- colored [colorful terminal output]
- clap [command-line argument parsing]
- guillotiere [packing algorithm]

## Flavortown Sidequests

### Optimization
I have had two optimzationm techniques in mind when making this:

- Efficient algorithms
  - This entire project is based off an algorithm that packs the images efficiently, I've also made it possible for you to be able to select with algorithm you want to use.
- Optimize asset sizes
  - Theres args you can add to your command to optimize the size and layout fo your final image, you can remove transparency on the input images and you can compress the output images if you want that, theres alot to chose from.

### Rusty Frontend
- [Ratatui](https://ratatui.rs/)
  - Selfexplanatory, for the terminal user interface, might also add a GUI version later. In the works currently.

## Todo
- [ ] Implement other packing algorithms
- [ ] Implement metadata ouput for unity
- [x] ~~Fill sheet left -> right, top -> bottom instead of what the hell is currently happening~~
- [x] ~~Add size optimizer that tries to find the smallest possible max_size for the output sprite sheet~~
- [ ] Ratatui 👀
- [ ] File compression

## Credits
- [https://kenney.nl](https://kenney.nl) for the example sprites 👑

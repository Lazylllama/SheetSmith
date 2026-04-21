# SheetSmith
> [!IMPORTANT]
> In heavy development, will change alot.

A simple sprite sheet packer written in Rust. It takes a directory of images and packs them into a single sprite sheet, along with metadata about the position and size of each sprite.

## Usage
```bash
Usage: sheetsmith [OPTIONS]

Options:
  -i, --input-dir <INPUT_DIR>  The input directory containing the images to pack [default: input]
  -o, --output <OUTPUT>        The output file for the packed sprite sheet [default: output.png]
  -m, --max-size <MAX_SIZE>    The format of the output metadata (json or unity) The maximum size of the output sprite sheet [default: 2048]
  -p, --padding <PADDING>      Padding between sprites in pixels [default: 2]
      --alg <ALGORITHM>        Algorithm to use for packing. Options: guillotiere [default: guillotiere]
  -t, --trim-transparent       Trim transparent pixels from the edges of images before packing This can help GREATLY reduce the size of the output sprite sheet and improve packing efficiency
  -a, --auto-size              Automatically find a good sheet size
  -d, --debug                  Debug mode: Print more often to find problematic images
  -h, --help                   Print help
  -V, --version                Print version
```

## Example Sheet
![Example Sheet](https://github.com/Lazylllama/SheetSmith/blob/main/example.png)

## Notable Dependecies
- anyhow [error handling]
- clap [command-line argument parsing]
- guillotiere [sprite packing algorithm]

## Flavortown Sidequests

### Optimization
I have had two optimzationm techniques in mind when making this:

- Efficient algorithms
  - This entire project is based off an algorithm that packs the images efficiently, I've also made it possible for you to be able to select with algorithm you want to use.
- Optimize asset sizes
  - Theres args you can add to your command to optimize the size and layout fo your final image, you can remove transparency on the input images and you can compress the output images if you want that, theres alot to chose from.

### Rusty Frontend
- [Ratatui](https://ratatui.rs/)
  - Selfexplanatory, for the terminal user interface, might also add a GUI version later.

## Todo
- [ ] Implement other packing algorithms
- [ ] Implement metadata ouput for unity
- [ ] Fill sheet left -> right, top -> bottom instead of what the hell is currently happening
- [x] Add size optimizer that tries to find the smallest possible max_size for the output sprite sheet
- [ ] Ratatui 👀
- [ ] File compression

## Credits
- [https://kenney.nl](https://kenney.nl) for the example sprites 👑

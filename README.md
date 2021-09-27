# DataBrush

DataBrush is a simple library for displaying structured data in the command line.
In supports dividing data into chunks and highlighting certain elements of those chunks.

## Example

```rust
    let sample = fs::read("/home/carstein/sample").unwrap();

    let mut example1 = Dataset::new(String::from("Executable file"), sample).unwrap();

    let mut chunk1 = match Chunk::new(String::from("elf header"), 52) {
        Ok(v) => v,
        Err(error) => panic!("Problem creating chunk: {:?}", error),
    };

    chunk1.set_highlight(
        Highlight::new(String::from("signature"), 0, 4).unwrap()).unwrap();
    chunk1.set_highlight(
        Highlight::new(String::from("e_entry"), 24, 4).unwrap()).unwrap();
    chunk1.set_highlight(
        Highlight::new(String::from("e_phoff"), 28, 4).unwrap()).unwrap();


    let chunk2 = match Chunk::new(String::from("program header table"), 32) {
        Ok(v) => v,
        Err(error) => panic!("Problem creating chunk: {:?}", error),
    };

    example1.add_chunk(chunk1).expect("Problem adding chunk");
    example1.add_chunk(chunk2).expect("Problem adding chunk");
    
    // Start painting
    let painter =  Painter::new();
    let sketch = painter.prepare(&example1);
```

This code will, if provided with an Elf file will display two chunks with three separate highlights.
Like on an example below (of course Markdown does not preserve colors):
```shell
======== Executable file
00000000┃ 7F 45 4C 46 01 01 01 00   00 00 00 00 00 00 00 00  ┓
00000010┃ 02 00 03 00 01 00 00 00   F0 82 04 08 34 00 00 00  ┃ elf header
00000020┃ 48 11 00 00 00 00 00 00   34 00 20 00 09 00 28 00  ┃
00000030┃ 1E 00 1B 00                                        ┛
-- signature
-- e_entry
-- e_phoff

00000030┃             06 00 00 00   34 00 00 00 34 80 04 08  ┓
00000040┃ 34 80 04 08 20 01 00 00   20 01 00 00 05 00 00 00  ┃ program header table
00000050┃ 04 00 00 00                                        ┛
```

## Problems/Plans
 - [ ] Better documentation
 - [ ] Better error handling
 - [ ] Terminal color detection (right now it just assumes it works)
 - [ ] Complete Api for manual dataset creation (as for now chunks needs to be added sequentionaly)
 - [ ] Helper method to read data structure from JSON
 - [ ] Support for dissecting bitfields
 - [ ] Optionally, raw data display (in addition to hex display)
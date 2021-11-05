# DataBrush

DataBrush is a simple library for displaying structured data in the command line.
In supports dividing data into chunks and highlighting certain elements of those chunks.

## Example

Easiest way to use the library is to import the format definition from the `JSON' file.

```rust
// Load the file to display
let sample = fs::read("/home/carstein/cample").unwrap();

// Read format from the json
let format = fs::read("/home/carstein/elf_format.json").unwrap();

// Create a data set by supplying format and data
let example: Dataset = Dataset::from_json(format, sample).unwrap();

// // Start painting
let painter =  databrush::Painter::new();
print!("{}", painter.prepare(&example).unwrap());
```

There is also a possibility to create a dataset manually, by calling relevant 

```rust
// Load the file to display
let sample = fs::read("/home/carstein/sample").unwrap();

let mut example1 = Dataset::new(String::from("Executable file"), sample).unwrap();

// Define a new chunk of 52 bytes
let mut chunk1 = match Chunk::new(String::from("elf header"), 52) {
    Ok(v) => v,
    Err(error) => panic!("Problem creating chunk: {:?}", error),
};

// Add three highlights to the data chunk 
chunk1.set_highlight(
    Highlight::new(String::from("signature"), 0, 4).unwrap()).unwrap();
chunk1.set_highlight(
    Highlight::new(String::from("e_entry"), 24, 4).unwrap()).unwrap();
chunk1.set_highlight(
    Highlight::new(String::from("e_phoff"), 28, 4).unwrap()).unwrap();

// Define another chunk of 32 bytes
let chunk2 = match Chunk::new(String::from("program header table"), 32) {
    Ok(v) => v,
    Err(error) => panic!("Problem creating chunk: {:?}", error),
};

// Add chunks to the dataset sequentionally (offset is calculated automatically)
example1.add_chunk(chunk1).expect("Problem adding chunk");
example1.add_chunk(chunk2).expect("Problem adding chunk");
    
// Start painting
let painter =  Painter::new();
let sketch = painter.prepare(&example1);
```

This code will, if provided with an Elf file will display two chunks with three separate highlights.
Like on an example below:
![Databrush sample](https://i.imgur.com/eVg7RCu.png)

## Format
Databrush expect format to have fields like presented below.

```json
{
  "name": "Executable file",
  "chunks": [
    {
      "name": "elf header",
      "offset": 0,
      "length": 52,
      "highlights": [
        {
          "name": "signature",
          "offset": 0,
          "length": 4
        },
      ]
    }
  ]
}
```
Important information is that offsets values are absolute and in relation to the entire data sample.


## Problems/Plans
```text
 - [ ] Better documentation
 - [X] Better error handling
 - [ ] Terminal color detection (right now it just assumes it works)
 - [X] Complete Api for manual dataset creation (as for now chunks needs to be added sequentionaly)
 - [X] Helper method to read data structure from JSON
 - [ ] Support for dissecting bitfields
 - [ ] Optionally, raw data display (in addition to hex display)
 ```

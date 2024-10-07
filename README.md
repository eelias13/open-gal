# OpenGAL

**OpenGAL** is a compiler that converts OpenGAL source code into a JEDEC file, which you can use to program a [GAL](https://en.wikipedia.org/wiki/Generic_array_logic) (Generic Array Logic) chip.

[![License: MIT](https://img.shields.io/badge/License-MIT-red.svg)](https://opensource.org/licenses/MIT)

## Web Editor

OpenGAL includes a web editor that you can try out [here](https://eelias13.github.io/projects/open-gal/).

- The **Download Code** button saves the code you've written.
- The **Compile** button compiles your code into a JEDEC file, ready to be flashed onto a GAL chip.
- The **Transpile to WinCUPL** button converts your code into [WinCUPL](https://www.microchip.com/en-us/development-tool/wincupl) code, another programming language for GALs that inspired OpenGAL.
- The **Download Table Data** button saves a JSON file representing the internal state of the code before hardware synthesis.

## Documentation

OpenGAL has four main language elements:

### 1. `pin`

The `pin` element assigns a name to a pin, which is essential because you cannot directly address pins. For example:

```
pin 1 = a;
```

This assigns the name `a` to pin 1 on your GAL chip.

### 2. `table`

There are three types of tables in OpenGAL:

1. **Full table:** You must specify every possible input combination. For example:

    ```rust
    table(i0, i1 -> and) {
        00 0
        01 0
        10 0
        11 1
    }
    ```

    Here, all input combinations (`i0` and `i1`) are explicitly represented.

2. **Fill table:** Any unspecified input combination is automatically filled with a default value (e.g., `.fill(0)`). For example:

    ```rust
    table(i0, i1 -> and).fill(0) {
        11 1
    }
    ```

    This table behaves like the full table above, as unspecified combinations default to `0`.

3. **Count table:** You specify only the output, and inputs are implied in order. For example:

    ```rust
    table(i0, i1 -> and).count {
        0
        0
        0
        1
    }
    ```

    This produces the same result as the full and fill tables above.

### 3. Boolean Functions

Boolean functions are straightforward. For example:

```
and = i0 & i1;
```

Supported operators include: 
- AND: `&`
- OR: `|`
- XOR: `?`
- NOT: `!`

You can also use parentheses `()` for grouping.

### 4. `dff`

The `dff` element represents a D flip-flop. To set a pin as a D flip-flop, simply type `.dff` in front of the pin, like so:

```
a.dff;
```

## What's New in OpenGAL

OpenGAL introduces two types of lists to simplify your code:

1. **Comma-separated list:** Specify multiple values separated by commas. For example:
   ```
   a, b, c2
   ```

2. **Range list:** Automatically generate values with ranges. For example:
   ```
   [1..5] => 1, 2, 3, 4, 5
   ```

   This works with variables as well. For example:
   ```
   pin[0..2] => pin0, pin1, pin2
   ```

Here are some examples of how to use these features:

- **Pin declaration:** 
  ```
  pin 1, 2 = o[0..1]; 
  pin [1..2] = o0, o1;
  ```

- **D flip-flop declaration:**
  ```
  o[0..2].dff; 
  o0, o1.dff;
  ```

- **Function declaration:**
  ```
  o[0..2] = a & b; 
  o0, o1 = a & b;
  ```

- **Table declaration:**
  ```
  table(i[0..1] -> and, or, xor) { ... }
  ```

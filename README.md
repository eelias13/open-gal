# open gal

open gal is a compiler that compiles the open gal source code into a jedec file that you can then burn on a [gal](https://en.wikipedia.org/wiki/Generic_array_logic)

## how the language works

in open gal ther are 4 language elements

### pin

the pin element assigns a name to the pin ist is important to do that because you can't address the pins directly

e.g. `pin 1 = a;` this assigns this name _a_ to the pin _1_ on your gal

### table

ther are 3 types of tabel

1. **full** if you define your table this way you have to consider every possible input e.g.

```#
table(i0, i1 -> and) {
    00 0
    01 0
    10 0
    11 1
}
```

Note that every possibility of the input variable (`i0` and `i1`) is represented in the table

2. **fill** if you define your table this way you every possibility you don't specify gets automatically assigned to the value in the fill (`.fill(0)` => 0) e.g.

```#
table(i0, i1 -> and).fill(0) {
    11 1
}
```

Note this table has the same effect as the table in _1_

3. **count** if you define your table this way you have to specify the "right" side of the table but it is important that you don't mix up the order and define every possible output e.g.

```#
table(i0, i1 -> and).count {
    0
    0
    0
    1
}
```

Note this table has the same effect as the table in _1_ and _2_

### boolean function

this element is imple just specify a function e.g. `and = i0 & i1;`

the operators are: and = `&`, or = `|`, xor = `?`, not = `!`.

and you can also use parentheses `()`

### dff

this is also straight forward just type `.dff` in front of the pin you want to set the d flip flop to e.g. `a.dff`

## what is new in open gal

to make life easier the new easy gal compiler also supports the list. there are 2 types of lists

1. the first one is the comma-separated list just specify the values and then separated by a comma e.g. `a, b, c2`

2. the second list is a list that counts automatically e.g. `[1..5]` => `1, 2, 3, 4, 5`

Note this dose also work with variabes e.g. `pin[0..2]` => `pin0, pin1, pin2`

Here are examples on how to use them in your project:

- pin `pin 1, 2 = o[0..1];` or `pin [1..2] = o0, o1;`

- dff `o[0..2].dff;` or `o0, o1.dff;`

- functions `o[0..2] = a & b` or `o0, o1 = a & b`

- table `table(i[0..1] -> and, or, xor) { ... }`

## how to install and build

before you start you need some tools

the first is [git](https://git-scm.com/downloads)

thi second is [rust](https://www.rust-lang.org/tools/install)

and the last one is [cbindgen](https://crates.io/crates/cbindgen) this you cant download this via cargo by typing `cargo install cbindgen`

if you have everything set up you can go ahead and clone the repo `git clone https://github.com/eeli1/open-gal` then `cd open-gal` and `make`

to use it just type `./OpenGal`

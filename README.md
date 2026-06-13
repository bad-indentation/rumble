# Rumble - Rust-Based Jumble and Anagram Solver
**Rumble** is a fast command-line tool for word game enthusiasts. It yields
all of the words that can be created using a fixed set of letters. For example:

```
$ rumble-cli ptso
```

outputs

```bash
spot
post
pots
tops
stop
opts
Found 6 solutions.
```

Rumble can also find partial anagrams that don't use all the letters using the -p flag.

```bash
$ rumble-cli murleb -p
elm
blue
...
rumble
lemur
Found 58 solutions.
```

## Installation
```bash
$ cargo install rumble-cli
```

## General Use
<ins>Usage</ins>: rumble-cli \[OPTIONS\] \<SCRAMBLED\>

<ins>Arguments</ins>:
  
  \<SCRAMBLED\>  The scrambled word you want to unscramble

<ins>Options</ins>:
  
  **-p, --include-partial**  Whether to include words that don't use all available letters
  
  **-v, --verbose**          Print debugging details to stderr
  
  **-h, --help**             Print help
  
  **-V, --version**          Print version

## License
Rumble is licensed under the MIT license. This means that you are free to 
install, modify, and distribute this code as long as you include the license 
text in your derivative. For more information, see LICENSE.txt.

## Note
Rumble uses a very extensive dictionary. As such, it may output obscure, profane,
or offensive words for certain inputs.

## Acknowledgements
Rumble uses a wordlist found in the repo [wordnik/wordlist](https://github.com/wordnik/wordlist?tab=MIT-1-ov-file).
You can find its license information in wordlist-license.txt.

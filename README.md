# ungoedel
It happens all the time, you leave your Turing machine lying around unattended for only a couple of minutes
and immediately a free roamig theoretical computer scientist comes by and gödelises your Turing machine,
rendering it completely unuseable.

Luckily, now there is a solution to this extremely common problem:
_ungoedel_ (or in the correct german spelling _ungödel_) is a program,
written entirely in Rust, to ungödelise any* Turing machine and returns it in the form of a Turing table. 
Check out the [wiki](https://github.com/nils-matthaei/ungoedel/wiki) to see how it works!

\*as long as it's tape alphabet is {0, 1, □}.

## How does it work?

It's really quite simple:

Just run the code with a binary gödel-number as the argument. And just like that the associated 
turing table will be printed out.

```
$ ungödel 100111011011010000101101011101101000010100001010000101000010

   |     0      |     1      |     □      |
---|------------|------------|------------|
Z0 | (Z0, 1, R) | (Z1, 0, N) | (   --   ) |
Z1 | (Z1, 0, L) | (Z2, 0, N) | (   --   ) |
Z2 | (   --   ) | (   --   ) | (   --   ) |
```

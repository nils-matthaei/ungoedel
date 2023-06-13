# ungoedel
It happens all the time, you leave your turing machine lying around unattended for only a couple of minutes
and immediately a free roamig theoretical computer scientist comes by and gödelises your turing machine,
rendering it completely unuseable.

Luckily, now there is a solution to this extrennely common problem:
_ungoedel_ or in the correct german spelling _ungödel_ is a simple program,
written entirely in Rust, to ungödelise any* turing machine and returns it in the form of a turing table. 
Check out the Wiki to see how it works!

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

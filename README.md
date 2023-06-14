# BirdStrikeRS

## components

tabiplexer/encoder

## link types

point2d
bool enable/disable

## DSL?

```
# comment
* foo:  NUM      30
* bar:  PNT      (2, 2)
* qux:  STR      "hi there *screams*"
* nand: LFUNC0   nand

* gate0: LogicGate  + LFUNC0:NAND
  + n0:qux + port: name + PNT0:foo +PNT1:h

*gate0 +n0:23



```


# BirdStrikeRS

## components

0: highlight
1: center

2: text
3: button
4: radio selector



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


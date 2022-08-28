# Instructions

## EXT

0000 000000000000
---- ------------
OPC  Ignored

Exit with zero

## STR

0001 000 000000000
---- --- ---------
OPC  SR  OFF

Store SR to memory at RPC + OFF

## LDR

0001 000 000000000
---- --- ---------
OPC  DR  OFF

Load 16 bit from memory at RPC + OFF and store it into DR

## INC

0001 0 00000000000
---- - -----------
OPC  M OFF     ---
               REG

### M == 0

Increment value at memory at RPC + OFF

### M == 1

Increment value of register REG

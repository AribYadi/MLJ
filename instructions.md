# Instructions

## EXT

| 0000 | 000000000000 |
| ---- | ------------ |
| OPC  | Ignored      |

Exit with zero

## STR

| 0001 | 000 | 000000000 |
| ---- | --- | --------- |
| OPC  | SR  | OFF       |

Store SR to memory at RPC + OFF

## LDR

| 0010 | 000 | 000000000 |
| ---- | --- | --------- |
| OPC  | DR  | OFF       |

Load 16 bit from memory at RPC + OFF and store it into DR

## INC

| 0011 | 0   | 00000000000 |
| ---- | --- | ----------- |
| OPC  | M   | OFF \| REG  |

- M == 0:
  Increment the value at memory at RPC + OFF

- M == 1:
  Increment the value of register REG

## DEC

| 0011 | 0   | 00000000000 |
| ---- | --- | ----------- |
| OPC  | M   | OFF \| REG  |

- M == 0:
  Decrement the value at memory at RPC + OFF

- M == 1:
  Decrement the value of register REG

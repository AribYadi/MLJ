# Specification for MLJ

## Register

| Name   | Bits | Purpose         |
| ------ | ---- | --------------- |
| R0..R5 | 32   | General         |
| RC     | 3    | Condition       |
| RPC    | 16   | Program Counter |

## Memory

An array of 16 bits with 0xFFFF in length

## Instruction

Instructions are 16 bits and are read from memory at addr RPC \
4 leftmost bits of the instructions are the opcode

### Opcodes

| Hex | Binary | Name                       |
| --- | ------ | -------------------------- |
| 0x0 | 0b0000 | [EXT](instructions.md#EXT) |
| 0x1 | 0b0001 | [STR](instructions.md#STR) |
| 0x2 | 0b0010 | [LDR](instructions.md#LDR) |
| 0x3 | 0b0011 | [INC](instructions.md#INC) |
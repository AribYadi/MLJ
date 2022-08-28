# Specification for MLJ

## Register

| Name   | Bits | Purpose         |
| ------ | ---- | --------------- |
| R0..R5 | 32   | General         |
| RC     | 1    | Condition       |
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
| 0x4 | 0b0100 | [DEC](instructions.md#DEC) |
| 0x5 | 0b0101 | [CMP](instructions.md#CMP) |
| 0x6 | 0b0110 | [JMC](instructions.md#JMC) |
| 0x7 | 0b0111 | [JMP](instructions.md#JMP) |
| 0x8 | 0b1000 | [MOV](instructions.md#MOV) |

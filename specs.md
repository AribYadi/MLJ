# Specification for MLJ

## Register

| Name   | Bits | Purpose         |
| ------ | ---- | --------------- |
| R0..R5 | 32   | General         |
| RC     | 1    | Condition       |
| RPC    | 16   | Program Counter |
| RSP    | 16   | Stack Pointer   |

## Memory

An array of 16 bits with 0xFFFF in length

## Instruction

Instructions are 16 bits and are read from memory at addr RPC \
4 leftmost bits of the instructions are the opcode

### Opcodes

| Hex  | Binary  | Name                       |
| ---- | ------- | -------------------------- |
| 0x00 | 0b00000 | [EXT](instructions.md#EXT) |
| 0x01 | 0b00001 | [STR](instructions.md#STR) |
| 0x02 | 0b00010 | [LDR](instructions.md#LDR) |
| 0x03 | 0b00011 | [INC](instructions.md#INC) |
| 0x04 | 0b00100 | [DEC](instructions.md#DEC) |
| 0x05 | 0b00101 | [CMP](instructions.md#CMP) |
| 0x06 | 0b00110 | [JMC](instructions.md#JMC) |
| 0x07 | 0b00111 | [JMP](instructions.md#JMP) |
| 0x08 | 0b01000 | [MOV](instructions.md#MOV) |
| 0x09 | 0b01001 | [ADD](instructions.md#ADD) |
| 0x0A | 0b01010 | [SUB](instructions.md#SUB) |
| 0x0B | 0b01011 | [MUL](instructions.md#MUL) |
| 0x0C | 0b01100 | [DIV](instructions.md#DIV) |
| 0x0D | 0b01101 | [REM](instructions.md#REM) |
| 0x0E | 0b01110 | [CLL](instructions.md#CLL) |
| 0x0F | 0b01111 | [RET](instructions.md#RET) |

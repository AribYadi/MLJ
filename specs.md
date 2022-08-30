# Specification for MLJ

## Endianness

For [Endianness](https://en.wikipedia.org/wiki/Endianness) MLJ uses big-endian

## Register

| Name   | Bits | Purpose         |
| ------ | ---- | --------------- |
| R0..R5 | 32   | General         |
| RC     | 1    | Condition       |
| RPC    | 16   | Program Counter |
| RSP    | 16   | Stack Pointer   |

## Memory

An array of 16 bits with 0xFFFF in length

### Layout

| 0x0000..0x2000 | 0x0201..0xFFFF        |
| -------------- | --------------------- |
| Call Stack     | Program Counter Start |

## Instruction

Instructions are 16 bits and are read from memory at addr RPC \
4 leftmost bits of the instructions are the opcode

### Opcodes

| Hex  | Binary  | Name                       |
| ---- | ------- | -------------------------- |
| 0x00 | 0b00000 | [EXT](instructions.md#EXT) |
| 0x08 | 0b00001 | [STR](instructions.md#STR) |
| 0x10 | 0b00010 | [LDR](instructions.md#LDR) |
| 0x18 | 0b00011 | [INC](instructions.md#INC) |
| 0x20 | 0b00100 | [DEC](instructions.md#DEC) |
| 0x28 | 0b00101 | [CMP](instructions.md#CMP) |
| 0x30 | 0b00110 | [JMC](instructions.md#JMC) |
| 0x38 | 0b00111 | [JMP](instructions.md#JMP) |
| 0x40 | 0b01000 | [MOV](instructions.md#MOV) |
| 0x48 | 0b01001 | [ADD](instructions.md#ADD) |
| 0x50 | 0b01010 | [SUB](instructions.md#SUB) |
| 0x58 | 0b01011 | [MUL](instructions.md#MUL) |
| 0x60 | 0b01100 | [DIV](instructions.md#DIV) |
| 0x68 | 0b01101 | [REM](instructions.md#REM) |
| 0x70 | 0b01110 | [CLL](instructions.md#CLL) |
| 0x78 | 0b01111 | [RET](instructions.md#RET) |

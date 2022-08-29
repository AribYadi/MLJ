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

| 0100 | 0   | 00000000000 |
| ---- | --- | ----------- |
| OPC  | M   | OFF \| REG  |

- M == 0:
  Decrement the value at memory at RPC + OFF

- M == 1:
  Decrement the value of register REG

## CMP

| 0101 | 000 | 000 | 000 | 000     |
| ---- | --- | --- | --- | ------- |
| OPC  | M   | SR1 | SR2 | Ignored |

- M == 0:
  Compare SR1 and SR2 and put 1 into RC if equal

- M == 1:
  Compare SR1 and SR2 and put 1 into RC if not equal

- M == 2:
  Compare SR1 and SR2 and put 1 into RC if less than

- M == 3:
  Compare SR1 and SR2 and put 1 into RC if less than or equal

- M == 4:
  Compare SR1 and SR2 and put 1 into RC if greater than

- M == 5:
  Compare SR1 and SR2 and put 1 into RC if greater than or equal

## JMC

| 0110 | 000000000000 |
| ---- | ------------ |
| OPC  | ADDR         |

Set RPC to ADDR if RC == 1

## JMP

| 0111 | 000000000000 |
| ---- | ------------ |
| OPC  | ADDR         |

Set RPC to ADDR

## MOV

| 1000 | 000 | 0   | 00000000  |
| ---- | --- | --- | --------- |
| OPC  | DR  | M   | SR \| IMM |

- M == 0:
  Copy the value of SR into DR
- M == 1:
- Set DR's value to IMM

## ADD

| 1001 | 000 | 0   | 00000000   |
| ---- | --- | --- | ---------- |
| OPC  | SR1 | M   | SR2 \| IMM |

- M == 0:
  Add the value of SR1 and SR2 and store the result to SR1
- M == 1:
  Add the value of SR1 and IMM and store the result to SR1

## SUB

| 1010 | 000 | 0   | 00000000   |
| ---- | --- | --- | ---------- |
| OPC  | SR1 | M   | SR2 \| IMM |

- M == 0:
  Subtract the value of SR1 and SR2 and store the result to SR1
- M == 1:
  Subtract the value of SR1 and IMM and store the result to SR1

## MUL

| 1011 | 000 | 0   | 00000000   |
| ---- | --- | --- | ---------- |
| OPC  | SR1 | M   | SR2 \| IMM |

- M == 0:
  Multiply the value of SR1 and SR2 and store the result to SR1
- M == 1:
  Multiply the value of SR1 and IMM and store the result to SR1

## DIV

| 1100 | 000 | 0   | 00000000   |
| ---- | --- | --- | ---------- |
| OPC  | SR1 | M   | SR2 \| IMM |

- M == 0:
  Divide the value of SR1 and SR2 and store the result to SR1
- M == 1:
  Divide the value of SR1 and IMM and store the result to SR1

## REM

| 1101 | 000 | 0   | 00000000   |
| ---- | --- | --- | ---------- |
| OPC  | SR1 | M   | SR2 \| IMM |

- M == 0:
  Divide the value of SR1 and SR2 and store the remainder to SR1
- M == 1:
  Divide the value of SR1 and IMM and store the remainder to SR1

## CLL

| 1110 | 000000000000 |
| ---- | ------------ |
| OPC  | ADDR         |

Push the current RPC to the call stack then set RPC to ADDR

## RET

| 1111 | 000000000000 |
| ---- | ------------ |
| OPC  | Ignored      |

Pop the top of the call stack and set RPC to the result

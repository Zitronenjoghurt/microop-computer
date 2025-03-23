# microop-computer

A CPU emulator based on micro operations.

## Example Output

```
[DEBUG cpu] New Fetch/Decode Cycle
[DEBUG cpu::bus_take         ] ✔
[DEBUG cpu::bus_write_address] PC → 0x0000000000000000
[DEBUG cpu::bus_set_read     ] ✔
[DEBUG cpu::bus_read_word    ] IR ← 12583043
[DEBUG cpu::bus_release      ] ✔
[DEBUG cpu::register_load_imm] TMP0 ← 4
[DEBUG cpu::alu_add          ] PC(4) = PC(0) + TMP0(4)
[DEBUG cpu::decode           ] #1: 00000000110000000000000010000011 | LB x1 = M[x0 + 12]
[DEBUG cpu::register_load_imm] TMP0 ← 12
[DEBUG cpu::alu_add          ] TMP1(12) = x0(0) + TMP0(12)
[DEBUG cpu::bus_take         ] ✔
[DEBUG cpu::bus_write_address] TMP1 → 0x000000000000000c
[DEBUG cpu::bus_set_read     ] ✔
[DEBUG cpu::bus_read_byte    ] x1 ← 17
[DEBUG cpu::bus_release      ] ✔
[DEBUG cpu] New Fetch/Decode Cycle
[DEBUG cpu::bus_take         ] ✔
[DEBUG cpu::bus_write_address] PC → 0x0000000000000004
[DEBUG cpu::bus_set_read     ] ✔
[DEBUG cpu::bus_read_word    ] IR ← 1081523
[DEBUG cpu::bus_release      ] ✔
[DEBUG cpu::register_load_imm] TMP0 ← 4
[DEBUG cpu::alu_add          ] PC(8) = PC(4) + TMP0(4)
[DEBUG cpu::decode           ] #2: 00000000000100001000000010110011 | ADD x1 = x1 + x1
[DEBUG cpu::alu_add          ] x1(34) = x1(17) + x1(17)
[DEBUG cpu] New Fetch/Decode Cycle
[DEBUG cpu::bus_take         ] ✔
[DEBUG cpu::bus_write_address] PC → 0x0000000000000008
[DEBUG cpu::bus_set_read     ] ✔
[DEBUG cpu::bus_read_word    ] IR ← 1048691
[DEBUG cpu::bus_release      ] ✔
[DEBUG cpu::register_load_imm] TMP0 ← 4
[DEBUG cpu::alu_add          ] PC(12) = PC(8) + TMP0(4)
[DEBUG cpu::decode           ] #3: 00000000000100000000000001110011 | EBREAK
[DEBUG cpu::halt             ] Halting

```
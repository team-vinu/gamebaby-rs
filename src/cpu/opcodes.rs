use super::registers::RegisterTrait;
use anyhow::Result;

// TODO: Implement OpCodeTrait for all opcodes
pub trait OpCodeTrait {
    fn execute(&self) -> Result<()>;
}

enum OpCodeValue<'a> {
    ImmediateByte(u8),
    ImmediateWord(u16),
    Register(&'a dyn RegisterTrait),
}

/*
 * ## Pan Docs

## [CPU Instruction Set](chrome-extension://pcmpcfapbekmbjjkdalcgopdkipoggdi/_generated_background_page.html#cpu-instruction-set)

Tables below specify the mnemonic, encoding, clock cycles, affected flags (ordered as znhc), and description. The timings assume a CPU clock frequency of 4.194304 MHz (or 8.4 MHz for CGB in double speed mode), called “T-states”. Because all Game Boy timings are divisible by 4, many people specify timings and clock frequency divided by 4, called “M-cycles”.

## [8-bit Load instructions](chrome-extension://pcmpcfapbekmbjjkdalcgopdkipoggdi/_generated_background_page.html#8-bit-load-instructions)

| Mnemonic | Encoding | Clock cycles | Flags | Description |
| --- | --- | --- | --- | --- |
| ld r,r | xx | 4 | –– | r=r |
| ld r,n | xx nn | 8 | –– | r=n |
| ld r,(HL) | xx | 8 | –– | r=(HL) |
| ld (HL),r | 7x | 8 | –– | (HL)=r |
| ld (HL),n | 36 nn | 12 | –– | (HL)=n |
| ld A,(BC) | 0A | 8 | –– | A=(BC) |
| ld A,(DE) | 1A | 8 | –– | A=(DE) |
| ld A,(nn) | FA | 16 | –– | A=(nn) |
| ld (BC),A | 02 | 8 | –– | (BC)=A |
| ld (DE),A | 12 | 8 | –– | (DE)=A |
| ld (nn),A | EA | 16 | –– | (nn)=A |
| ld A,(FF00+n) | F0 nn | 12 | –– | read from io-port n (memory FF00+n) |
| ld (FF00+n),A | E0 nn | 12 | –– | write to io-port n (memory FF00+n) |
| ld A,(FF00+C) | F2 | 8 | –– | read from io-port C (memory FF00+C) |
| ld (FF00+C),A | E2 | 8 | –– | write to io-port C (memory FF00+C) |
| ldi (HL),A | 22 | 8 | –– | (HL)=A, HL=HL+1 |
| ldi A,(HL) | 2A | 8 | –– | A=(HL), HL=HL+1 |
| ldd (HL),A | 32 | 8 | –– | (HL)=A, HL=HL-1 |
| ldd A,(HL) | 3A | 8 | –– | A=(HL), HL=HL-1 |

## [16-bit Load instructions](chrome-extension://pcmpcfapbekmbjjkdalcgopdkipoggdi/_generated_background_page.html#16-bit-load-instructions)

| Mnemonic | Encoding | Clock cycles | Flags | Description |
| --- | --- | --- | --- | --- |
| ld rr,nn | x1 nn nn | 12 | –– | rr=nn (rr may be BC,DE,HL or SP) |
| ld (nn),SP | 08 nn nn | 20 | –– | (nn)=SP |
| ld SP,HL | F9 | 8 | –– | SP=HL |
| push rr | x5 | 16 | –– | SP=SP-2 (SP)=rr ; rr may be BC,DE,HL,AF |
| pop rr | x1 | 12 | (AF) | rr=(SP) SP=SP+2 ; rr may be BC,DE,HL,AF |

## [8-bit Arithmetic/Logic instructions](chrome-extension://pcmpcfapbekmbjjkdalcgopdkipoggdi/_generated_background_page.html#8-bit-arithmeticlogic-instructions)

| Mnemonic | Encoding | Clock cycles | Flags | Description |
| --- | --- | --- | --- | --- |
| add A,r | 8x | 4 | z0hc | A=A+r |
| add A,n | C6 nn | 8 | z0hc | A=A+n |
| add A,(HL) | 86 | 8 | z0hc | A=A+(HL) |
| adc A,r | 8x | 4 | z0hc | A=A+r+cy |
| adc A,n | CE nn | 8 | z0hc | A=A+n+cy |
| adc A,(HL) | 8E | 8 | z0hc | A=A+(HL)+cy |
| sub r | 9x | 4 | z1hc | A=A-r |
| sub n | D6 nn | 8 | z1hc | A=A-n |
| sub (HL) | 96 | 8 | z1hc | A=A-(HL) |
| sbc A,r | 9x | 4 | z1hc | A=A-r-cy |
| sbc A,n | DE nn | 8 | z1hc | A=A-n-cy |
| sbc A,(HL) | 9E | 8 | z1hc | A=A-(HL)-cy |
| and r | Ax | 4 | z010 | A=A & r |
| and n | E6 nn | 8 | z010 | A=A & n |
| and (HL) | A6 | 8 | z010 | A=A & (HL) |
| xor r | Ax | 4 | z000 | A=A xor r |
| xor n | EE nn | 8 | z000 | A=A xor n |
| xor (HL) | AE | 8 | z000 | A=A xor (HL) |
| or r | Bx | 4 | z000 | A=A | r |
| or n | F6 nn | 8 | z000 | A=A | n |
| or (HL) | B6 | 8 | z000 | A=A | (HL) |
| cp r | Bx | 4 | z1hc | compare A-r |
| cp n | FE nn | 8 | z1hc | compare A-n |
| cp (HL) | BE | 8 | z1hc | compare A-(HL) |
| inc r | xx | 4 | z0h- | r=r+1 |
| inc (HL) | 34 | 12 | z0h- | (HL)=(HL)+1 |
| dec r | xx | 4 | z1h- | r=r-1 |
| dec (HL) | 35 | 12 | z1h- | (HL)=(HL)-1 |
| daa | 27 | 4 | z-0c | decimal adjust A |
| cpl | 2F | 4 | \-11- | A = A xor FF |

## [16-bit Arithmetic/Logic instructions](chrome-extension://pcmpcfapbekmbjjkdalcgopdkipoggdi/_generated_background_page.html#16-bit-arithmeticlogic-instructions)

| Mnemonic | Encoding | Clock cycles | Flags | Description |
| --- | --- | --- | --- | --- |
| add HL,rr | x9 | 8 | \-0hc | HL = HL+rr ; rr may be BC,DE,HL,SP |
| inc rr | x3 | 8 | –– | rr = rr+1 ; rr may be BC,DE,HL,SP |
| dec rr | xB | 8 | –– | rr = rr-1 ; rr may be BC,DE,HL,SP |
| add SP,dd | E8 dd | 16 | 00hc | SP = SP +/- dd ; dd is 8-bit signed number |
| ld HL,SP+dd | F8 dd | 12 | 00hc | HL = SP +/- dd ; dd is 8-bit signed number |

## [Rotate and Shift instructions](chrome-extension://pcmpcfapbekmbjjkdalcgopdkipoggdi/_generated_background_page.html#rotate-and-shift-instructions)

| Mnemonic | Encoding | Clock cycles | Flags | Description |
| --- | --- | --- | --- | --- |
| rlca | 07 | 4 | 000c | rotate A left |
| rla | 17 | 4 | 000c | rotate A left through carry |
| rrca | 0F | 4 | 000c | rotate A right |
| rra | 1F | 4 | 000c | rotate A right through carry |
| rlc r | CB 0x | 8 | z00c | rotate left |
| rlc (HL) | CB 06 | 16 | z00c | rotate left |
| rl r | CB 1x | 8 | z00c | rotate left through carry |
| rl (HL) | CB 16 | 16 | z00c | rotate left through carry |
| rrc r | CB 0x | 8 | z00c | rotate right |
| rrc (HL) | CB 0E | 16 | z00c | rotate right |
| rr r | CB 1x | 8 | z00c | rotate right through carry |
| rr (HL) | CB 1E | 16 | z00c | rotate right through carry |
| sla r | CB 2x | 8 | z00c | shift left arithmetic (b0=0) |
| sla (HL) | CB 26 | 16 | z00c | shift left arithmetic (b0=0) |
| swap r | CB 3x | 8 | z000 | exchange low/hi-nibble |
| swap (HL) | CB 36 | 16 | z000 | exchange low/hi-nibble |
| sra r | CB 2x | 8 | z00c | shift right arithmetic (b7=b7) |
| sra (HL) | CB 2E | 16 | z00c | shift right arithmetic (b7=b7) |
| srl r | CB 3x | 8 | z00c | shift right logical (b7=0) |
| srl (HL) | CB 3E | 16 | z00c | shift right logical (b7=0) |

## [Single-bit Operation instructions](chrome-extension://pcmpcfapbekmbjjkdalcgopdkipoggdi/_generated_background_page.html#single-bit-operation-instructions)

| Mnemonic | Encoding | Clock cycles | Flags | Description |
| --- | --- | --- | --- | --- |
| bit n,r | CB xx | 8 | z01- | test bit n |
| bit n,(HL) | CB xx | 12 | z01- | test bit n |
| set n,r | CB xx | 8 | –– | set bit n |
| set n,(HL) | CB xx | 16 | –– | set bit n |
| res n,r | CB xx | 8 | –– | reset bit n |
| res n,(HL) | CB xx | 16 | –– | reset bit n |

## [CPU Control instructions](chrome-extension://pcmpcfapbekmbjjkdalcgopdkipoggdi/_generated_background_page.html#cpu-control-instructions)

| Mnemonic | Encoding | Clock cycles | Flags | Description |
| --- | --- | --- | --- | --- |
| ccf | 3F | 4 | \-00c | cy=cy xor 1 |
| scf | 37 | 4 | \-001 | cy=1 |
| nop | 00 | 4 | –– | no operation |
| halt | 76 | N\*4 | –– | halt until interrupt occurs (low power) |
| stop | 10 00 | ? | –– | low power standby mode (VERY low power) |
| di | F3 | 4 | –– | disable interrupts, IME=0 |
| ei | FB | 4 | –– | enable interrupts, IME=1 |

## [Jump instructions](chrome-extension://pcmpcfapbekmbjjkdalcgopdkipoggdi/_generated_background_page.html#jump-instructions)

| Mnemonic | Encoding | Clock cycles | Flags | Description |
| --- | --- | --- | --- | --- |
| jp nn | C3 nn nn | 16 | –– | jump to nn, PC=nn |
| jp HL | E9 | 4 | –– | jump to HL, PC=HL |
| jp f,nn | xx nn nn | 16/12 | –– | conditional jump if nz,z,nc,c |
| jr PC+dd | 18 dd | 12 | –– | relative jump to nn (PC=PC+8-bit signed) |
| jr f,PC+dd | xx dd | 12/8 | –– | conditional relative jump if nz,z,nc,c |
| call nn | CD nn nn | 24 | –– | call to nn, SP=SP-2, (SP)=PC, PC=nn |
| call f,nn | xx nn nn | 24/12 | –– | conditional call if nz,z,nc,c |
| ret | C9 | 16 | –– | return, PC=(SP), SP=SP+2 |
| ret f | xx | 20/8 | –– | conditional return if nz,z,nc,c |
| reti | D9 | 16 | –– | return and enable interrupts (IME=1) |
| rst n | xx | 16 | –– | call to 00,08,10,18,20,28,30,38 |
 */
#[allow(non_camel_case_types)]
pub enum OpCode {
    Ld_R_R,
    Ld_R_N(u8),
    Ld_R_HL,
    Ld_HL_R,
    Ld_HL_N(u8),
    Ld_A_BC,
    Ld_A_DE,
    Ld_A_NN(u8),
    Ld_BC_A,
    Ld_DE_A,
    Ld_NN_A(u8),
    Ld_A_FF00_N(u8),
    Ld_FF00_N_A(u8),
    Ld_A_FF00_C,
    Ld_FF00_C_A,
    Ldi_HL_A,
    Ldi_A_HL,
    Ldd_HL_A,
    Ldd_A_HL,
    Ld_RR_NN(u16),
    Ld_NN_SP(u16),
    Ld_SP_HL,
    Push_RR,
    Pop_RR,
    Add_A_R,
    Add_A_N(u8),
    Add_A_HL,
    Adc_A_R,
    Adc_A_N(u8),
    Adc_A_HL,
    Sub_R,
    Sub_N(u8),
    Sub_HL,
    Sbc_A_R,
    Sbc_A_N(u8),
    Sbc_A_HL,
    And_R,
    And_N(u8),
    And_HL,
    Xor_R,
    Xor_N(u8),
    Xor_HL,
    Or_R,
    Or_N(u8),
    Or_HL,
    Cp_R,
    Cp_N(u8),
    Cp_HL,
    Inc_R,
    Inc_HL,
    Dec_R,
    Dec_HL,
    Daa,
    Cpl,
    Add_HL_RR,
    Inc_RR,
    Dec_RR,
    Add_SP_DD(i8),
    Ld_HL_SP_DD(i8),
    Rlca,
    Rla,
    Rrca,
    Rra,
    Rlc_R,
    Rlc_HL,
    Rl_R,
    Rl_HL,
    Rrc_R,
    Rrc_HL,
    Rr_R,
    Rr_HL,
    Sla_R,
    Sla_HL,
    Swap_R,
    Swap_HL,
    Sra_R,
    Sra_HL,
    Srl_R,
    Srl_HL,
    Bit_N_R(u8),
    Bit_N_HL(u8),
    Set_N_R(u8),
    Set_N_HL(u8),
    Res_N_R(u8),
    Res_N_HL(u8),
    Ccf,
    Scf,
    Nop,
    Halt,
    Stop,
    Di,
    Ei,
    Jp_NN(u16),
    Jp_HL,
    Jp_F_NN(u16),
    Jr_PC_DD(i8),
    Jr_F_PC_DD(i8),
    Call_NN(u16),
    Call_F_NN(u16),
    Ret,
    Ret_F,
    Reti,
    Rst_N(u8),
}

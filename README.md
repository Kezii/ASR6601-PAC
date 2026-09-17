# ASR6601 PAC

Peripheral Access Crate for the ASR6601 LPWAN SoC (Arm China STAR-MC1),
generated with `svd2rust` 0.37.1 + `form` 0.13.0.

## Source layout and regeneration

`svd/peripherals/*.yaml` plus `svd/svd.yaml` are canonical. `svd/ASR6601.svd`
is generated (git-ignored) and `src/` is generated from it. Never hand-edit
generated files:

```sh
python3 svd/yaml2svd.py svd -o svd/ASR6601.svd
bash run.sh            # svd2rust + form + cargo fmt
cargo check
cargo check --features rt
```

Reference documents used for this work live in `documents/`:

| File | Role |
|---|---|
| `ASR6601_Reference_Manual_V1.5.0.pdf` | Canonical register reference (English) |
| `ASR6601_Reference_Manual_V1.3.0.pdf` | Older manual, kept for history |
| `ASR6601_Datasheet_V1.7.0_EN.pdf` | Pinout/electrical data; contains no register definitions |
| `ASR6601_GPIO_MUX_Table_V1.1.pdf` | Per-pin alternate-function mapping |
| `tremo.svd` / `TREMO.h` | Official vendor SVD v1.6.2 and its generated header |
| `tremo_regs.h` | SDK v1.6.2 register structs |
| `tremo_dma.h` | SDK v2.0 DMA register map (full DesignWare view) |
| `algorithm.h` / `sec_regs.h` | SDK v2.0 crypto (`SAE`/`RNG`) register structs and bases |

## Vendor SVD problems (`documents/tremo.svd`)

The official SVD was used as a cross-check, not as ground truth. Defects found:

* **QSPI fields are copy-paste noise.** Every register carries the same
  `ADDRFAULTEN` bit 0; only `QSPI_ABR` differs, with unrelated power bits.
  The QSPI peripheral was removed from this PAC until a trusted source exists.
* **Wrong interrupts for crypto.** `SAE` and `RNG` report `SEC:0`; the manual
  interrupt table gives `sac:30` and no interrupt for the random generator.
* **Missing `I2C2`.** The SVD omits it; the manual memory map (`0x40015000`),
  register chapter (`I2Cx, x=0,1,2`) and IRQ 5 prove it exists.
* **DMA is a flat subset.** `DMAC0/1` expose only `SAR/DAR/LLP/CTL/CFG` per
  channel as 32-bit registers; the manual (§21.8) defines 64-bit layouts and
  the SDK driver uses the full DesignWare map.
* **`DMA_CFG0@0x148` is misnamed.** It is channel 3's `CFG` register.
* **`STATUS_DST_TRAN` description says “source…”.** Copy-paste of the SRC entry.
* **GPIO `INT_CR`/`FR` packing contradicts the manual.** The SVD packs
  `POS[15:0]`/`NEG[31:16]`; the manual interleaves `POS@2n`/`NEG@2n+1`.
* **`GPIOD_AFRH` copied from port A.** The SVD uses 4-bit fields; the manual
  (§11.14.17) defines 3-bit fields for Port D pins 8–15.
* **RM table typo inherited:** `DMA_MaskDstTran@0x300` should read
  `STATUS_DST_TRAN`; `SINC`/`DINC` print `10: no change` twice (second is 11);
  `CFG` lists two “Bit 18” entries (`SRC_HS_POL@19`, `DST_HS_POL@18` per the
  header row); `CTL` labels bits 24–23 “SMS” where it means `DMS`.

## Mismatches between references

The manual, the vendor SVD/headers, and the SDK disagree in places. Notable cases:

* **GPIO `0x28/0x2C`:** manual + `TREMO.h` + SVD say `INT_CR`/`FR`;
  the SDK says `ICR`/`IFR`.
* **LORAC `0x100–0x118`:** manual + `TREMO.h` + SVD say `LORAC_CR0…`;
  the SDK says `CR0…`.
* **RTC `0x00`:** manual says `RTC_CR`; SDK + SVD + `TREMO.h` say `CTRL`.
* **RTC long names:** manual `PPMADJUST`, `CYC_MAX_VALUE`, `ASYNDATA`,
  `SUB_SECOND`, `CYC_CNT_VALUE`, `ALARMx_SUB`; SDK/SVD use longer variants
  (`PPM_ADJUST`, `ASYN_DATA`, …).
* **EFC names:** manual `PROG_DATAx`, `SERIAL_NUM_*`, `OPTION_EXE_ONLY_*`,
  `OPTION_WR_PROTECT_*`, `OPTION_SECURE_*`; SDK/SVD abbreviate
  (`PROGRAM_DATAx`, `SN_*`, `OPTION_EO/WP/SEC_*`).
* **Basic timer:** memory map + IRQ + RCC bits say `BASICTIM`;
  chapter 17 and register prefix say `BSTIM`.
* **EFC factory window (`0x20–0x38`):** SDK/SVD/`TREMO.h` document
  `CHIP_PATTERN`, `IP_TRIM_L/H`, `TEST_INFO_L/H`; the manual marks the range
  `RESERVED`.
* **SEC layout:** SDK packs `SR`/`FILTERx` contiguously; `TREMO.h` + SVD place
  `SR@0x0C`, `FILTER2@0x024` with reserved gaps. No manual chapter exists.
* **AFEC base:** SDK-derived sources used `0x40008200`; memory map +
  `TREMO.h` + SVD use `0x40008000` with registers at `+0x200`.
* **SCC ghost:** `SCC_*` clock/reset/status bits exist in SVD/SDK/`TREMO.h`
  (e.g. `CGR0:9`, `RST0:19`, `SR1:14`) for a block with no base address, no
  IRQ, no driver, and zero manual mentions. Omitted.
* **RM V1.5.0 has no chapters** for `PWR`, `I2S`, `DAC`, `LCD`, `WDG`, `IWDG`,
  `CRC`, `QSPI`, `SEC`, `SAE`, `RNG`, `AFEC`; those follow SDK/SVD/`TREMO.h`.
  The datasheet adds no register information (pinout/electrical only).

## Conventions adopted

* **Manual V1.5.0 is canonical for names.** SVD/SDK spellings give way to it.
* **Peripheral prefix is stripped** (`RCC_CR0` → `rcc::cr0`, `GPIOx_OER` →
  `gpioa::oer`), following STM32 PAC practice and the SDK. Modules already
  namespace registers, so the prefix would only stutter.
* **Shared instances use `derivedFrom`** (`UART1–3` from `UART0`, `GPTIM1–3`
  from `GPTIM0`, …).
* **64-bit layout where the manual defines it** (DMA `SAR/DAR/LLP/CTL/CFG`,
  status/mask/clear words generate `u64` accessors).
* **Uniform numeric enums are shared** (`stm32f4` precedent): GPIO `FUN0–7`
  (MUX-table values; per-pin meaning stays in the MUX document),
  `STOP3_WU_SEL` pin groups, DMA transfer modes/sizes/widths/masters/locks.
* **No invented documentation.** Registers without a trusted field source stay
  bare; reserved regions are not published as registers.

## Exceptions adopted (deliberate deviations)

* **`I2C2` kept** although the vendor SVD omits it (manual proves it).
* **`SAE` keeps interrupt `SAC:30`, `RNG` keeps none** (manual over vendor SVD).
* **EFC factory registers restored** (`CHIP_PATTERN`, `IP_TRIM_L/H`,
  `TEST_INFO_L/H`) although the manual marks them reserved — all vendor
  sources document them unanimously and they are read-only ID data.
* **SCC bits omitted** (ghost block, see above).
* **QSPI peripheral removed** (its SVD field definitions are noise; clocks,
  reset and remap bits in RCC/SYSCFG stay).
* **GPIO keeps 32×1-bit `INT_CR`/`FR` fields** instead of the SVD's 2×16-bit
  packing, matching the manual's interleaved layout.
* **Basic-timer peripherals are `BSTIMx`** (chapter + register aligned) while
  their RCC bits use the manual's `BASICTIMx` spelling.
* **`LPTIM0_INF_CLK_EN`** (not `PCLK`) and **`LPTIMx_EXT_CLK_SEL`** (not
  `EXTCLK`) follow manual/RCC-bit spellings; `SR DONE` bits carry the manual's
  `SET_` prefix.

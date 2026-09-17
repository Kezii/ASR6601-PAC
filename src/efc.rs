#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    int_en: IntEn,
    sr: Sr,
    prog_data0: ProgData0,
    prog_data1: ProgData1,
    timing_cfg: TimingCfg,
    protect_seq: ProtectSeq,
    _reserved7: [u8; 0x04],
    chip_pattern: ChipPattern,
    ip_trim_l: IpTrimL,
    ip_trim_h: IpTrimH,
    serial_num_low: SerialNumLow,
    serial_num_high: SerialNumHigh,
    test_info_l: TestInfoL,
    test_info_h: TestInfoH,
    option_csr_bytes: OptionCsrBytes,
    option_exe_only_bytes: OptionExeOnlyBytes,
    option_wr_protect_bytes: OptionWrProtectBytes,
    option_secure_bytes0: OptionSecureBytes0,
    option_secure_bytes1: OptionSecureBytes1,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - interrupt enable register"]
    #[inline(always)]
    pub const fn int_en(&self) -> &IntEn {
        &self.int_en
    }
    #[doc = "0x08 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x0c - program data0 register"]
    #[inline(always)]
    pub const fn prog_data0(&self) -> &ProgData0 {
        &self.prog_data0
    }
    #[doc = "0x10 - program data1 register"]
    #[inline(always)]
    pub const fn prog_data1(&self) -> &ProgData1 {
        &self.prog_data1
    }
    #[doc = "0x14 - timing config register"]
    #[inline(always)]
    pub const fn timing_cfg(&self) -> &TimingCfg {
        &self.timing_cfg
    }
    #[doc = "0x18 - protect sequence register"]
    #[inline(always)]
    pub const fn protect_seq(&self) -> &ProtectSeq {
        &self.protect_seq
    }
    #[doc = "0x20 - chip pattern register"]
    #[inline(always)]
    pub const fn chip_pattern(&self) -> &ChipPattern {
        &self.chip_pattern
    }
    #[doc = "0x24 - analog ip trimming low register"]
    #[inline(always)]
    pub const fn ip_trim_l(&self) -> &IpTrimL {
        &self.ip_trim_l
    }
    #[doc = "0x28 - analog ip trimming high register"]
    #[inline(always)]
    pub const fn ip_trim_h(&self) -> &IpTrimH {
        &self.ip_trim_h
    }
    #[doc = "0x2c - serial number low register"]
    #[inline(always)]
    pub const fn serial_num_low(&self) -> &SerialNumLow {
        &self.serial_num_low
    }
    #[doc = "0x30 - serial number high register"]
    #[inline(always)]
    pub const fn serial_num_high(&self) -> &SerialNumHigh {
        &self.serial_num_high
    }
    #[doc = "0x34 - test info low register"]
    #[inline(always)]
    pub const fn test_info_l(&self) -> &TestInfoL {
        &self.test_info_l
    }
    #[doc = "0x38 - test info high register"]
    #[inline(always)]
    pub const fn test_info_h(&self) -> &TestInfoH {
        &self.test_info_h
    }
    #[doc = "0x3c - option control and status register"]
    #[inline(always)]
    pub const fn option_csr_bytes(&self) -> &OptionCsrBytes {
        &self.option_csr_bytes
    }
    #[doc = "0x40 - option exe-only bytes register"]
    #[inline(always)]
    pub const fn option_exe_only_bytes(&self) -> &OptionExeOnlyBytes {
        &self.option_exe_only_bytes
    }
    #[doc = "0x44 - option write-protect bytes register"]
    #[inline(always)]
    pub const fn option_wr_protect_bytes(&self) -> &OptionWrProtectBytes {
        &self.option_wr_protect_bytes
    }
    #[doc = "0x48 - option secure byte 0 register"]
    #[inline(always)]
    pub const fn option_secure_bytes0(&self) -> &OptionSecureBytes0 {
        &self.option_secure_bytes0
    }
    #[doc = "0x4c - option secure byte 1 register"]
    #[inline(always)]
    pub const fn option_secure_bytes1(&self) -> &OptionSecureBytes1 {
        &self.option_secure_bytes1
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "control register"]
pub mod cr;
#[doc = "INT_EN (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`] module"]
#[doc(alias = "INT_EN")]
pub type IntEn = crate::Reg<int_en::IntEnSpec>;
#[doc = "interrupt enable register"]
pub mod int_en;
#[doc = "SR (rw) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;
#[doc = "PROG_DATA0 (rw) register accessor: program data0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`prog_data0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prog_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prog_data0`] module"]
#[doc(alias = "PROG_DATA0")]
pub type ProgData0 = crate::Reg<prog_data0::ProgData0Spec>;
#[doc = "program data0 register"]
pub mod prog_data0;
#[doc = "PROG_DATA1 (rw) register accessor: program data1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`prog_data1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prog_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prog_data1`] module"]
#[doc(alias = "PROG_DATA1")]
pub type ProgData1 = crate::Reg<prog_data1::ProgData1Spec>;
#[doc = "program data1 register"]
pub mod prog_data1;
#[doc = "TIMING_CFG (rw) register accessor: timing config register\n\nYou can [`read`](crate::Reg::read) this register and get [`timing_cfg::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timing_cfg`] module"]
#[doc(alias = "TIMING_CFG")]
pub type TimingCfg = crate::Reg<timing_cfg::TimingCfgSpec>;
#[doc = "timing config register"]
pub mod timing_cfg;
#[doc = "PROTECT_SEQ (w) register accessor: protect sequence register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`protect_seq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@protect_seq`] module"]
#[doc(alias = "PROTECT_SEQ")]
pub type ProtectSeq = crate::Reg<protect_seq::ProtectSeqSpec>;
#[doc = "protect sequence register"]
pub mod protect_seq;
#[doc = "CHIP_PATTERN (r) register accessor: chip pattern register\n\nYou can [`read`](crate::Reg::read) this register and get [`chip_pattern::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chip_pattern`] module"]
#[doc(alias = "CHIP_PATTERN")]
pub type ChipPattern = crate::Reg<chip_pattern::ChipPatternSpec>;
#[doc = "chip pattern register"]
pub mod chip_pattern;
#[doc = "IP_TRIM_L (r) register accessor: analog ip trimming low register\n\nYou can [`read`](crate::Reg::read) this register and get [`ip_trim_l::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ip_trim_l`] module"]
#[doc(alias = "IP_TRIM_L")]
pub type IpTrimL = crate::Reg<ip_trim_l::IpTrimLSpec>;
#[doc = "analog ip trimming low register"]
pub mod ip_trim_l;
#[doc = "IP_TRIM_H (r) register accessor: analog ip trimming high register\n\nYou can [`read`](crate::Reg::read) this register and get [`ip_trim_h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ip_trim_h`] module"]
#[doc(alias = "IP_TRIM_H")]
pub type IpTrimH = crate::Reg<ip_trim_h::IpTrimHSpec>;
#[doc = "analog ip trimming high register"]
pub mod ip_trim_h;
#[doc = "SERIAL_NUM_LOW (r) register accessor: serial number low register\n\nYou can [`read`](crate::Reg::read) this register and get [`serial_num_low::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@serial_num_low`] module"]
#[doc(alias = "SERIAL_NUM_LOW")]
pub type SerialNumLow = crate::Reg<serial_num_low::SerialNumLowSpec>;
#[doc = "serial number low register"]
pub mod serial_num_low;
#[doc = "SERIAL_NUM_HIGH (r) register accessor: serial number high register\n\nYou can [`read`](crate::Reg::read) this register and get [`serial_num_high::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@serial_num_high`] module"]
#[doc(alias = "SERIAL_NUM_HIGH")]
pub type SerialNumHigh = crate::Reg<serial_num_high::SerialNumHighSpec>;
#[doc = "serial number high register"]
pub mod serial_num_high;
#[doc = "TEST_INFO_L (r) register accessor: test info low register\n\nYou can [`read`](crate::Reg::read) this register and get [`test_info_l::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test_info_l`] module"]
#[doc(alias = "TEST_INFO_L")]
pub type TestInfoL = crate::Reg<test_info_l::TestInfoLSpec>;
#[doc = "test info low register"]
pub mod test_info_l;
#[doc = "TEST_INFO_H (r) register accessor: test info high register\n\nYou can [`read`](crate::Reg::read) this register and get [`test_info_h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test_info_h`] module"]
#[doc(alias = "TEST_INFO_H")]
pub type TestInfoH = crate::Reg<test_info_h::TestInfoHSpec>;
#[doc = "test info high register"]
pub mod test_info_h;
#[doc = "OPTION_CSR_BYTES (r) register accessor: option control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_csr_bytes::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@option_csr_bytes`] module"]
#[doc(alias = "OPTION_CSR_BYTES")]
pub type OptionCsrBytes = crate::Reg<option_csr_bytes::OptionCsrBytesSpec>;
#[doc = "option control and status register"]
pub mod option_csr_bytes;
#[doc = "OPTION_EXE_ONLY_BYTES (r) register accessor: option exe-only bytes register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_exe_only_bytes::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@option_exe_only_bytes`] module"]
#[doc(alias = "OPTION_EXE_ONLY_BYTES")]
pub type OptionExeOnlyBytes = crate::Reg<option_exe_only_bytes::OptionExeOnlyBytesSpec>;
#[doc = "option exe-only bytes register"]
pub mod option_exe_only_bytes;
#[doc = "OPTION_WR_PROTECT_BYTES (r) register accessor: option write-protect bytes register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_wr_protect_bytes::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@option_wr_protect_bytes`] module"]
#[doc(alias = "OPTION_WR_PROTECT_BYTES")]
pub type OptionWrProtectBytes = crate::Reg<option_wr_protect_bytes::OptionWrProtectBytesSpec>;
#[doc = "option write-protect bytes register"]
pub mod option_wr_protect_bytes;
#[doc = "OPTION_SECURE_BYTES0 (r) register accessor: option secure byte 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_secure_bytes0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@option_secure_bytes0`] module"]
#[doc(alias = "OPTION_SECURE_BYTES0")]
pub type OptionSecureBytes0 = crate::Reg<option_secure_bytes0::OptionSecureBytes0Spec>;
#[doc = "option secure byte 0 register"]
pub mod option_secure_bytes0;
#[doc = "OPTION_SECURE_BYTES1 (r) register accessor: option secure byte 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_secure_bytes1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@option_secure_bytes1`] module"]
#[doc(alias = "OPTION_SECURE_BYTES1")]
pub type OptionSecureBytes1 = crate::Reg<option_secure_bytes1::OptionSecureBytes1Spec>;
#[doc = "option secure byte 1 register"]
pub mod option_secure_bytes1;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    int_en: IntEn,
    sr: Sr,
    program_data0: ProgramData0,
    program_data1: ProgramData1,
    timing_cfg: TimingCfg,
    protect_seq: ProtectSeq,
    _reserved7: [u8; 0x10],
    sn_l: SnL,
    sn_h: SnH,
    _reserved9: [u8; 0x08],
    option_csr_bytes: OptionCsrBytes,
    option_eo_bytes: OptionEoBytes,
    option_wp_bytes: OptionWpBytes,
    option_sec_bytes0: OptionSecBytes0,
    option_sec_bytes1: OptionSecBytes1,
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
    pub const fn program_data0(&self) -> &ProgramData0 {
        &self.program_data0
    }
    #[doc = "0x10 - program data1 register"]
    #[inline(always)]
    pub const fn program_data1(&self) -> &ProgramData1 {
        &self.program_data1
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
    #[doc = "0x2c - serial number low register"]
    #[inline(always)]
    pub const fn sn_l(&self) -> &SnL {
        &self.sn_l
    }
    #[doc = "0x30 - serial number high register"]
    #[inline(always)]
    pub const fn sn_h(&self) -> &SnH {
        &self.sn_h
    }
    #[doc = "0x3c - option control and status register"]
    #[inline(always)]
    pub const fn option_csr_bytes(&self) -> &OptionCsrBytes {
        &self.option_csr_bytes
    }
    #[doc = "0x40 - option exe-only bytes register"]
    #[inline(always)]
    pub const fn option_eo_bytes(&self) -> &OptionEoBytes {
        &self.option_eo_bytes
    }
    #[doc = "0x44 - option write-protect bytes register"]
    #[inline(always)]
    pub const fn option_wp_bytes(&self) -> &OptionWpBytes {
        &self.option_wp_bytes
    }
    #[doc = "0x48 - option secure byte 0 register"]
    #[inline(always)]
    pub const fn option_sec_bytes0(&self) -> &OptionSecBytes0 {
        &self.option_sec_bytes0
    }
    #[doc = "0x4c - option secure byte 1 register"]
    #[inline(always)]
    pub const fn option_sec_bytes1(&self) -> &OptionSecBytes1 {
        &self.option_sec_bytes1
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
#[doc = "PROGRAM_DATA0 (rw) register accessor: program data0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`program_data0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`program_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@program_data0`] module"]
#[doc(alias = "PROGRAM_DATA0")]
pub type ProgramData0 = crate::Reg<program_data0::ProgramData0Spec>;
#[doc = "program data0 register"]
pub mod program_data0;
#[doc = "PROGRAM_DATA1 (rw) register accessor: program data1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`program_data1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`program_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@program_data1`] module"]
#[doc(alias = "PROGRAM_DATA1")]
pub type ProgramData1 = crate::Reg<program_data1::ProgramData1Spec>;
#[doc = "program data1 register"]
pub mod program_data1;
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
#[doc = "SN_L (r) register accessor: serial number low register\n\nYou can [`read`](crate::Reg::read) this register and get [`sn_l::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sn_l`] module"]
#[doc(alias = "SN_L")]
pub type SnL = crate::Reg<sn_l::SnLSpec>;
#[doc = "serial number low register"]
pub mod sn_l;
#[doc = "SN_H (r) register accessor: serial number high register\n\nYou can [`read`](crate::Reg::read) this register and get [`sn_h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sn_h`] module"]
#[doc(alias = "SN_H")]
pub type SnH = crate::Reg<sn_h::SnHSpec>;
#[doc = "serial number high register"]
pub mod sn_h;
#[doc = "OPTION_CSR_BYTES (r) register accessor: option control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_csr_bytes::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@option_csr_bytes`] module"]
#[doc(alias = "OPTION_CSR_BYTES")]
pub type OptionCsrBytes = crate::Reg<option_csr_bytes::OptionCsrBytesSpec>;
#[doc = "option control and status register"]
pub mod option_csr_bytes;
#[doc = "OPTION_EO_BYTES (r) register accessor: option exe-only bytes register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_eo_bytes::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@option_eo_bytes`] module"]
#[doc(alias = "OPTION_EO_BYTES")]
pub type OptionEoBytes = crate::Reg<option_eo_bytes::OptionEoBytesSpec>;
#[doc = "option exe-only bytes register"]
pub mod option_eo_bytes;
#[doc = "OPTION_WP_BYTES (r) register accessor: option write-protect bytes register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_wp_bytes::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@option_wp_bytes`] module"]
#[doc(alias = "OPTION_WP_BYTES")]
pub type OptionWpBytes = crate::Reg<option_wp_bytes::OptionWpBytesSpec>;
#[doc = "option write-protect bytes register"]
pub mod option_wp_bytes;
#[doc = "OPTION_SEC_BYTES0 (r) register accessor: option secure byte 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_sec_bytes0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@option_sec_bytes0`] module"]
#[doc(alias = "OPTION_SEC_BYTES0")]
pub type OptionSecBytes0 = crate::Reg<option_sec_bytes0::OptionSecBytes0Spec>;
#[doc = "option secure byte 0 register"]
pub mod option_sec_bytes0;
#[doc = "OPTION_SEC_BYTES1 (r) register accessor: option secure byte 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_sec_bytes1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@option_sec_bytes1`] module"]
#[doc(alias = "OPTION_SEC_BYTES1")]
pub type OptionSecBytes1 = crate::Reg<option_sec_bytes1::OptionSecBytes1Spec>;
#[doc = "option secure byte 1 register"]
pub mod option_sec_bytes1;

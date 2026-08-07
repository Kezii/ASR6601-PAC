#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    int_sr: IntSr,
    raw_sr: RawSr,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - interrupt status register"]
    #[inline(always)]
    pub const fn int_sr(&self) -> &IntSr {
        &self.int_sr
    }
    #[doc = "0x08 - raw status register"]
    #[inline(always)]
    pub const fn raw_sr(&self) -> &RawSr {
        &self.raw_sr
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "control register"]
pub mod cr;
#[doc = "INT_SR (rw) register accessor: interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_sr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_sr`] module"]
#[doc(alias = "INT_SR")]
pub type IntSr = crate::Reg<int_sr::IntSrSpec>;
#[doc = "interrupt status register"]
pub mod int_sr;
#[doc = "RAW_SR (r) register accessor: raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_sr`] module"]
#[doc(alias = "RAW_SR")]
pub type RawSr = crate::Reg<raw_sr::RawSrSpec>;
#[doc = "raw status register"]
pub mod raw_sr;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    swtrigr: Swtrigr,
    dhr: Dhr,
    dor: Dor,
    sr: Sr,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - software trigger register"]
    #[inline(always)]
    pub const fn swtrigr(&self) -> &Swtrigr {
        &self.swtrigr
    }
    #[doc = "0x08 - data holding register"]
    #[inline(always)]
    pub const fn dhr(&self) -> &Dhr {
        &self.dhr
    }
    #[doc = "0x0c - data output register"]
    #[inline(always)]
    pub const fn dor(&self) -> &Dor {
        &self.dor
    }
    #[doc = "0x10 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "control register"]
pub mod cr;
#[doc = "SWTRIGR (w) register accessor: software trigger register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrigr`] module"]
#[doc(alias = "SWTRIGR")]
pub type Swtrigr = crate::Reg<swtrigr::SwtrigrSpec>;
#[doc = "software trigger register"]
pub mod swtrigr;
#[doc = "DHR (rw) register accessor: data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr`] module"]
#[doc(alias = "DHR")]
pub type Dhr = crate::Reg<dhr::DhrSpec>;
#[doc = "data holding register"]
pub mod dhr;
#[doc = "DOR (r) register accessor: data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor`] module"]
#[doc(alias = "DOR")]
pub type Dor = crate::Reg<dor::DorSpec>;
#[doc = "data output register"]
pub mod dor;
#[doc = "SR (rw) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;

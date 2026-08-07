#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: Isr,
    icr: Icr,
    ier: Ier,
    cfgr: Cfgr,
    cr: Cr,
    cmp: Cmp,
    arr: Arr,
    cnt: Cnt,
    csr: Csr,
    sr1: Sr1,
}
impl RegisterBlock {
    #[doc = "0x00 - LPTIMER flag and status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x04 - LPTIMER flag clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x08 - LPTIMER interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x0c - LPTIMER configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x10 - LPTIMER control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x14 - LPTIMER compare register"]
    #[inline(always)]
    pub const fn cmp(&self) -> &Cmp {
        &self.cmp
    }
    #[doc = "0x18 - LPTIMER autoreload register"]
    #[inline(always)]
    pub const fn arr(&self) -> &Arr {
        &self.arr
    }
    #[doc = "0x1c - LPTIMER counter register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x20 - LPTIMER CSR register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x24 - LPTIMER SR1 register"]
    #[inline(always)]
    pub const fn sr1(&self) -> &Sr1 {
        &self.sr1
    }
}
#[doc = "ISR (r) register accessor: LPTIMER flag and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "LPTIMER flag and status register"]
pub mod isr;
#[doc = "ICR (w) register accessor: LPTIMER flag clear register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "LPTIMER flag clear register"]
pub mod icr;
#[doc = "IER (rw) register accessor: LPTIMER interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "LPTIMER interrupt enable register"]
pub mod ier;
#[doc = "CFGR (rw) register accessor: LPTIMER configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "LPTIMER configuration register"]
pub mod cfgr;
#[doc = "CR (rw) register accessor: LPTIMER control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "LPTIMER control register"]
pub mod cr;
#[doc = "CMP (rw) register accessor: LPTIMER compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp`] module"]
#[doc(alias = "CMP")]
pub type Cmp = crate::Reg<cmp::CmpSpec>;
#[doc = "LPTIMER compare register"]
pub mod cmp;
#[doc = "ARR (rw) register accessor: LPTIMER autoreload register\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`] module"]
#[doc(alias = "ARR")]
pub type Arr = crate::Reg<arr::ArrSpec>;
#[doc = "LPTIMER autoreload register"]
pub mod arr;
#[doc = "CNT (r) register accessor: LPTIMER counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "LPTIMER counter register"]
pub mod cnt;
#[doc = "CSR (r) register accessor: LPTIMER CSR register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "LPTIMER CSR register"]
pub mod csr;
#[doc = "SR1 (r) register accessor: LPTIMER SR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`] module"]
#[doc(alias = "SR1")]
pub type Sr1 = crate::Reg<sr1::Sr1Spec>;
#[doc = "LPTIMER SR1 register"]
pub mod sr1;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    max: Max,
    win: Win,
    sr: Sr,
    sr1: Sr1,
    cr1: Cr1,
    sr2: Sr2,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - max value register"]
    #[inline(always)]
    pub const fn max(&self) -> &Max {
        &self.max
    }
    #[doc = "0x08 - window value register"]
    #[inline(always)]
    pub const fn win(&self) -> &Win {
        &self.win
    }
    #[doc = "0x0c - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - status register 1"]
    #[inline(always)]
    pub const fn sr1(&self) -> &Sr1 {
        &self.sr1
    }
    #[doc = "0x14 - control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x18 - status register 2"]
    #[inline(always)]
    pub const fn sr2(&self) -> &Sr2 {
        &self.sr2
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "control register"]
pub mod cr;
#[doc = "MAX (rw) register accessor: max value register\n\nYou can [`read`](crate::Reg::read) this register and get [`max::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max`] module"]
#[doc(alias = "MAX")]
pub type Max = crate::Reg<max::MaxSpec>;
#[doc = "max value register"]
pub mod max;
#[doc = "WIN (rw) register accessor: window value register\n\nYou can [`read`](crate::Reg::read) this register and get [`win::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win`] module"]
#[doc(alias = "WIN")]
pub type Win = crate::Reg<win::WinSpec>;
#[doc = "window value register"]
pub mod win;
#[doc = "SR (r) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;
#[doc = "SR1 (r) register accessor: status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`] module"]
#[doc(alias = "SR1")]
pub type Sr1 = crate::Reg<sr1::Sr1Spec>;
#[doc = "status register 1"]
pub mod sr1;
#[doc = "CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "SR2 (rw) register accessor: status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sr2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr2`] module"]
#[doc(alias = "SR2")]
pub type Sr2 = crate::Reg<sr2::Sr2Spec>;
#[doc = "status register 2"]
pub mod sr2;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    cfgr: Cfgr,
    seqr0: Seqr0,
    seqr1: Seqr1,
    diffsel: Diffsel,
    isr: Isr,
    ier: Ier,
    dr: Dr,
    awd0_cfgr: Awd0Cfgr,
    awd1_cfgr: Awd1Cfgr,
    awd2_cfgr: Awd2Cfgr,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x08 - sequence0 register"]
    #[inline(always)]
    pub const fn seqr0(&self) -> &Seqr0 {
        &self.seqr0
    }
    #[doc = "0x0c - sequence1 register"]
    #[inline(always)]
    pub const fn seqr1(&self) -> &Seqr1 {
        &self.seqr1
    }
    #[doc = "0x10 - difference register"]
    #[inline(always)]
    pub const fn diffsel(&self) -> &Diffsel {
        &self.diffsel
    }
    #[doc = "0x14 - interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x18 - interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x1c - data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x20 - AWD0 register"]
    #[inline(always)]
    pub const fn awd0_cfgr(&self) -> &Awd0Cfgr {
        &self.awd0_cfgr
    }
    #[doc = "0x24 - AWD1 register"]
    #[inline(always)]
    pub const fn awd1_cfgr(&self) -> &Awd1Cfgr {
        &self.awd1_cfgr
    }
    #[doc = "0x28 - AWD2 register"]
    #[inline(always)]
    pub const fn awd2_cfgr(&self) -> &Awd2Cfgr {
        &self.awd2_cfgr
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "control register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "configuration register"]
pub mod cfgr;
#[doc = "SEQR0 (rw) register accessor: sequence0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`seqr0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqr0`] module"]
#[doc(alias = "SEQR0")]
pub type Seqr0 = crate::Reg<seqr0::Seqr0Spec>;
#[doc = "sequence0 register"]
pub mod seqr0;
#[doc = "SEQR1 (rw) register accessor: sequence1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`seqr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqr1`] module"]
#[doc(alias = "SEQR1")]
pub type Seqr1 = crate::Reg<seqr1::Seqr1Spec>;
#[doc = "sequence1 register"]
pub mod seqr1;
#[doc = "DIFFSEL (rw) register accessor: difference register\n\nYou can [`read`](crate::Reg::read) this register and get [`diffsel::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diffsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diffsel`] module"]
#[doc(alias = "DIFFSEL")]
pub type Diffsel = crate::Reg<diffsel::DiffselSpec>;
#[doc = "difference register"]
pub mod diffsel;
#[doc = "ISR (rw) register accessor: interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "interrupt status register"]
pub mod isr;
#[doc = "IER (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "DR (r) register accessor: data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "data register"]
pub mod dr;
#[doc = "AWD0_CFGR (rw) register accessor: AWD0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd0_cfgr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd0_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd0_cfgr`] module"]
#[doc(alias = "AWD0_CFGR")]
pub type Awd0Cfgr = crate::Reg<awd0_cfgr::Awd0CfgrSpec>;
#[doc = "AWD0 register"]
pub mod awd0_cfgr;
#[doc = "AWD1_CFGR (rw) register accessor: AWD1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd1_cfgr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd1_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd1_cfgr`] module"]
#[doc(alias = "AWD1_CFGR")]
pub type Awd1Cfgr = crate::Reg<awd1_cfgr::Awd1CfgrSpec>;
#[doc = "AWD1 register"]
pub mod awd1_cfgr;
#[doc = "AWD2_CFGR (rw) register accessor: AWD2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd2_cfgr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd2_cfgr`] module"]
#[doc(alias = "AWD2_CFGR")]
pub type Awd2Cfgr = crate::Reg<awd2_cfgr::Awd2CfgrSpec>;
#[doc = "AWD2 register"]
pub mod awd2_cfgr;

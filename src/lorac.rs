#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ssp_cr0: SspCr0,
    ssp_cr1: SspCr1,
    ssp_dr: SspDr,
    ssp_sr: SspSr,
    ssp_cpsr: SspCpsr,
    ssp_imsc: SspImsc,
    ssp_ris: SspRis,
    ssp_mis: SspMis,
    ssp_icr: SspIcr,
    ssp_dma_cr: SspDmaCr,
    _reserved10: [u8; 0xd8],
    cr0: Cr0,
    cr1: Cr1,
    sr: Sr,
    nss_cr: NssCr,
    sck_cr: SckCr,
    mosi_cr: MosiCr,
    miso_sr: MisoSr,
}
impl RegisterBlock {
    #[doc = "0x00 - ssp control register 0"]
    #[inline(always)]
    pub const fn ssp_cr0(&self) -> &SspCr0 {
        &self.ssp_cr0
    }
    #[doc = "0x04 - ssp control register 1"]
    #[inline(always)]
    pub const fn ssp_cr1(&self) -> &SspCr1 {
        &self.ssp_cr1
    }
    #[doc = "0x08 - ssp data register"]
    #[inline(always)]
    pub const fn ssp_dr(&self) -> &SspDr {
        &self.ssp_dr
    }
    #[doc = "0x0c - ssp status register"]
    #[inline(always)]
    pub const fn ssp_sr(&self) -> &SspSr {
        &self.ssp_sr
    }
    #[doc = "0x10 - ssp clock prescale register"]
    #[inline(always)]
    pub const fn ssp_cpsr(&self) -> &SspCpsr {
        &self.ssp_cpsr
    }
    #[doc = "0x14 - ssp interrupt mask set or clear register"]
    #[inline(always)]
    pub const fn ssp_imsc(&self) -> &SspImsc {
        &self.ssp_imsc
    }
    #[doc = "0x18 - ssp raw interrupt status register"]
    #[inline(always)]
    pub const fn ssp_ris(&self) -> &SspRis {
        &self.ssp_ris
    }
    #[doc = "0x1c - ssp masked interrupt status register"]
    #[inline(always)]
    pub const fn ssp_mis(&self) -> &SspMis {
        &self.ssp_mis
    }
    #[doc = "0x20 - ssp interrupt clear register"]
    #[inline(always)]
    pub const fn ssp_icr(&self) -> &SspIcr {
        &self.ssp_icr
    }
    #[doc = "0x24 - ssp DMA control register"]
    #[inline(always)]
    pub const fn ssp_dma_cr(&self) -> &SspDmaCr {
        &self.ssp_dma_cr
    }
    #[doc = "0x100 - control register 0"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0x104 - control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x108 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10c - nss control register"]
    #[inline(always)]
    pub const fn nss_cr(&self) -> &NssCr {
        &self.nss_cr
    }
    #[doc = "0x110 - sck control register"]
    #[inline(always)]
    pub const fn sck_cr(&self) -> &SckCr {
        &self.sck_cr
    }
    #[doc = "0x114 - mosi control register"]
    #[inline(always)]
    pub const fn mosi_cr(&self) -> &MosiCr {
        &self.mosi_cr
    }
    #[doc = "0x118 - miso control register"]
    #[inline(always)]
    pub const fn miso_sr(&self) -> &MisoSr {
        &self.miso_sr
    }
}
#[doc = "SSP_CR0 (rw) register accessor: ssp control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_cr0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_cr0`] module"]
#[doc(alias = "SSP_CR0")]
pub type SspCr0 = crate::Reg<ssp_cr0::SspCr0Spec>;
#[doc = "ssp control register 0"]
pub mod ssp_cr0;
#[doc = "SSP_CR1 (rw) register accessor: ssp control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_cr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_cr1`] module"]
#[doc(alias = "SSP_CR1")]
pub type SspCr1 = crate::Reg<ssp_cr1::SspCr1Spec>;
#[doc = "ssp control register 1"]
pub mod ssp_cr1;
#[doc = "SSP_DR (rw) register accessor: ssp data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_dr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_dr`] module"]
#[doc(alias = "SSP_DR")]
pub type SspDr = crate::Reg<ssp_dr::SspDrSpec>;
#[doc = "ssp data register"]
pub mod ssp_dr;
#[doc = "SSP_SR (r) register accessor: ssp status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_sr`] module"]
#[doc(alias = "SSP_SR")]
pub type SspSr = crate::Reg<ssp_sr::SspSrSpec>;
#[doc = "ssp status register"]
pub mod ssp_sr;
#[doc = "SSP_CPSR (rw) register accessor: ssp clock prescale register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_cpsr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_cpsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_cpsr`] module"]
#[doc(alias = "SSP_CPSR")]
pub type SspCpsr = crate::Reg<ssp_cpsr::SspCpsrSpec>;
#[doc = "ssp clock prescale register"]
pub mod ssp_cpsr;
#[doc = "SSP_IMSC (rw) register accessor: ssp interrupt mask set or clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_imsc::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_imsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_imsc`] module"]
#[doc(alias = "SSP_IMSC")]
pub type SspImsc = crate::Reg<ssp_imsc::SspImscSpec>;
#[doc = "ssp interrupt mask set or clear register"]
pub mod ssp_imsc;
#[doc = "SSP_RIS (r) register accessor: ssp raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_ris`] module"]
#[doc(alias = "SSP_RIS")]
pub type SspRis = crate::Reg<ssp_ris::SspRisSpec>;
#[doc = "ssp raw interrupt status register"]
pub mod ssp_ris;
#[doc = "SSP_MIS (r) register accessor: ssp masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_mis`] module"]
#[doc(alias = "SSP_MIS")]
pub type SspMis = crate::Reg<ssp_mis::SspMisSpec>;
#[doc = "ssp masked interrupt status register"]
pub mod ssp_mis;
#[doc = "SSP_ICR (rw) register accessor: ssp interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_icr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_icr`] module"]
#[doc(alias = "SSP_ICR")]
pub type SspIcr = crate::Reg<ssp_icr::SspIcrSpec>;
#[doc = "ssp interrupt clear register"]
pub mod ssp_icr;
#[doc = "SSP_DMA_CR (rw) register accessor: ssp DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_dma_cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_dma_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_dma_cr`] module"]
#[doc(alias = "SSP_DMA_CR")]
pub type SspDmaCr = crate::Reg<ssp_dma_cr::SspDmaCrSpec>;
#[doc = "ssp DMA control register"]
pub mod ssp_dma_cr;
#[doc = "CR0 (rw) register accessor: control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`] module"]
#[doc(alias = "CR0")]
pub type Cr0 = crate::Reg<cr0::Cr0Spec>;
#[doc = "control register 0"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "SR (r) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;
#[doc = "NSS_CR (rw) register accessor: nss control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nss_cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nss_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nss_cr`] module"]
#[doc(alias = "NSS_CR")]
pub type NssCr = crate::Reg<nss_cr::NssCrSpec>;
#[doc = "nss control register"]
pub mod nss_cr;
#[doc = "SCK_CR (rw) register accessor: sck control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sck_cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sck_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sck_cr`] module"]
#[doc(alias = "SCK_CR")]
pub type SckCr = crate::Reg<sck_cr::SckCrSpec>;
#[doc = "sck control register"]
pub mod sck_cr;
#[doc = "MOSI_CR (rw) register accessor: mosi control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mosi_cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosi_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mosi_cr`] module"]
#[doc(alias = "MOSI_CR")]
pub type MosiCr = crate::Reg<mosi_cr::MosiCrSpec>;
#[doc = "mosi control register"]
pub mod mosi_cr;
#[doc = "MISO_SR (rw) register accessor: miso control register\n\nYou can [`read`](crate::Reg::read) this register and get [`miso_sr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miso_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miso_sr`] module"]
#[doc(alias = "MISO_SR")]
pub type MisoSr = crate::Reg<miso_sr::MisoSrSpec>;
#[doc = "miso control register"]
pub mod miso_sr;

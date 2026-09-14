#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr0: Cr0,
    cr1: Cr1,
    dr: Dr,
    sr: Sr,
    cpsr: Cpsr,
    imsc: Imsc,
    ris: Ris,
    mis: Mis,
    icr: Icr,
    dma_cr: DmaCr,
    _reserved10: [u8; 0x0fb8],
    periph_id0: PeriphId0,
    periph_id1: PeriphId1,
    periph_id2: PeriphId2,
    periph_id3: PeriphId3,
    pcell_id0: PcellId0,
    pcell_id1: PcellId1,
    pcell_id2: PcellId2,
    pcell_id3: PcellId3,
}
impl RegisterBlock {
    #[doc = "0x00 - control register 0"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0x04 - control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x08 - data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x0c - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - clock prescale register"]
    #[inline(always)]
    pub const fn cpsr(&self) -> &Cpsr {
        &self.cpsr
    }
    #[doc = "0x14 - interrupt mask set or clear register"]
    #[inline(always)]
    pub const fn imsc(&self) -> &Imsc {
        &self.imsc
    }
    #[doc = "0x18 - raw interrupt status register"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x1c - masked interrupt status register"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x20 - interrupt clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x24 - DMA control register"]
    #[inline(always)]
    pub const fn dma_cr(&self) -> &DmaCr {
        &self.dma_cr
    }
    #[doc = "0xfe0 - peripheral identification register 0"]
    #[inline(always)]
    pub const fn periph_id0(&self) -> &PeriphId0 {
        &self.periph_id0
    }
    #[doc = "0xfe4 - peripheral identification register 1"]
    #[inline(always)]
    pub const fn periph_id1(&self) -> &PeriphId1 {
        &self.periph_id1
    }
    #[doc = "0xfe8 - peripheral identification register 2"]
    #[inline(always)]
    pub const fn periph_id2(&self) -> &PeriphId2 {
        &self.periph_id2
    }
    #[doc = "0xfec - peripheral identification register 3"]
    #[inline(always)]
    pub const fn periph_id3(&self) -> &PeriphId3 {
        &self.periph_id3
    }
    #[doc = "0xff0 - prime cell identification register 0"]
    #[inline(always)]
    pub const fn pcell_id0(&self) -> &PcellId0 {
        &self.pcell_id0
    }
    #[doc = "0xff4 - prime cell identification register 1"]
    #[inline(always)]
    pub const fn pcell_id1(&self) -> &PcellId1 {
        &self.pcell_id1
    }
    #[doc = "0xff8 - prime cell identification register 2"]
    #[inline(always)]
    pub const fn pcell_id2(&self) -> &PcellId2 {
        &self.pcell_id2
    }
    #[doc = "0xffc - prime cell identification register 3"]
    #[inline(always)]
    pub const fn pcell_id3(&self) -> &PcellId3 {
        &self.pcell_id3
    }
}
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
#[doc = "DR (rw) register accessor: data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "data register"]
pub mod dr;
#[doc = "SR (r) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;
#[doc = "CPSR (rw) register accessor: clock prescale register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsr`] module"]
#[doc(alias = "CPSR")]
pub type Cpsr = crate::Reg<cpsr::CpsrSpec>;
#[doc = "clock prescale register"]
pub mod cpsr;
#[doc = "IMSC (rw) register accessor: interrupt mask set or clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`imsc::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imsc`] module"]
#[doc(alias = "IMSC")]
pub type Imsc = crate::Reg<imsc::ImscSpec>;
#[doc = "interrupt mask set or clear register"]
pub mod imsc;
#[doc = "RIS (r) register accessor: raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`] module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "raw interrupt status register"]
pub mod ris;
#[doc = "MIS (r) register accessor: masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`] module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "masked interrupt status register"]
pub mod mis;
#[doc = "ICR (w) register accessor: interrupt clear register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "DMA_CR (rw) register accessor: DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cr`] module"]
#[doc(alias = "DMA_CR")]
pub type DmaCr = crate::Reg<dma_cr::DmaCrSpec>;
#[doc = "DMA control register"]
pub mod dma_cr;
#[doc = "PERIPH_ID0 (r) register accessor: peripheral identification register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id0`] module"]
#[doc(alias = "PERIPH_ID0")]
pub type PeriphId0 = crate::Reg<periph_id0::PeriphId0Spec>;
#[doc = "peripheral identification register 0"]
pub mod periph_id0;
#[doc = "PERIPH_ID1 (r) register accessor: peripheral identification register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id1`] module"]
#[doc(alias = "PERIPH_ID1")]
pub type PeriphId1 = crate::Reg<periph_id1::PeriphId1Spec>;
#[doc = "peripheral identification register 1"]
pub mod periph_id1;
#[doc = "PERIPH_ID2 (r) register accessor: peripheral identification register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id2`] module"]
#[doc(alias = "PERIPH_ID2")]
pub type PeriphId2 = crate::Reg<periph_id2::PeriphId2Spec>;
#[doc = "peripheral identification register 2"]
pub mod periph_id2;
#[doc = "PERIPH_ID3 (r) register accessor: peripheral identification register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id3`] module"]
#[doc(alias = "PERIPH_ID3")]
pub type PeriphId3 = crate::Reg<periph_id3::PeriphId3Spec>;
#[doc = "peripheral identification register 3"]
pub mod periph_id3;
#[doc = "PCELL_ID0 (r) register accessor: prime cell identification register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcell_id0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcell_id0`] module"]
#[doc(alias = "PCELL_ID0")]
pub type PcellId0 = crate::Reg<pcell_id0::PcellId0Spec>;
#[doc = "prime cell identification register 0"]
pub mod pcell_id0;
#[doc = "PCELL_ID1 (r) register accessor: prime cell identification register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcell_id1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcell_id1`] module"]
#[doc(alias = "PCELL_ID1")]
pub type PcellId1 = crate::Reg<pcell_id1::PcellId1Spec>;
#[doc = "prime cell identification register 1"]
pub mod pcell_id1;
#[doc = "PCELL_ID2 (r) register accessor: prime cell identification register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pcell_id2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcell_id2`] module"]
#[doc(alias = "PCELL_ID2")]
pub type PcellId2 = crate::Reg<pcell_id2::PcellId2Spec>;
#[doc = "prime cell identification register 2"]
pub mod pcell_id2;
#[doc = "PCELL_ID3 (r) register accessor: prime cell identification register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pcell_id3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcell_id3`] module"]
#[doc(alias = "PCELL_ID3")]
pub type PcellId3 = crate::Reg<pcell_id3::PcellId3Spec>;
#[doc = "prime cell identification register 3"]
pub mod pcell_id3;

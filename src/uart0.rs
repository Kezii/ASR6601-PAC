#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dr: Dr,
    rsr_ecr: RsrEcr,
    _reserved2: [u8; 0x10],
    fr: Fr,
    _reserved3: [u8; 0x04],
    ilpr: Ilpr,
    ibrd: Ibrd,
    fbrd: Fbrd,
    lcr_h: LcrH,
    cr: Cr,
    ifls: Ifls,
    imsc: Imsc,
    ris: Ris,
    mis: Mis,
    icr: Icr,
    dmacr: Dmacr,
    _reserved14: [u8; 0x0f84],
    pcellid0: Pcellid0,
    pcellid1: Pcellid1,
    pcellid2: Pcellid2,
    pcellid3: Pcellid3,
    periphid0: Periphid0,
    periphid1: Periphid1,
    periphid2: Periphid2,
    periphid3: Periphid3,
}
impl RegisterBlock {
    #[doc = "0x00 - data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x04 - receive status register / error clear register"]
    #[inline(always)]
    pub const fn rsr_ecr(&self) -> &RsrEcr {
        &self.rsr_ecr
    }
    #[doc = "0x18 - flag register"]
    #[inline(always)]
    pub const fn fr(&self) -> &Fr {
        &self.fr
    }
    #[doc = "0x20 - IRDA low power counter register"]
    #[inline(always)]
    pub const fn ilpr(&self) -> &Ilpr {
        &self.ilpr
    }
    #[doc = "0x24 - integer baudrate register"]
    #[inline(always)]
    pub const fn ibrd(&self) -> &Ibrd {
        &self.ibrd
    }
    #[doc = "0x28 - fractional baudrate register"]
    #[inline(always)]
    pub const fn fbrd(&self) -> &Fbrd {
        &self.fbrd
    }
    #[doc = "0x2c - line control register"]
    #[inline(always)]
    pub const fn lcr_h(&self) -> &LcrH {
        &self.lcr_h
    }
    #[doc = "0x30 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x34 - interrupt fifo level select register"]
    #[inline(always)]
    pub const fn ifls(&self) -> &Ifls {
        &self.ifls
    }
    #[doc = "0x38 - interrupt mask set/clear register"]
    #[inline(always)]
    pub const fn imsc(&self) -> &Imsc {
        &self.imsc
    }
    #[doc = "0x3c - raw interrupt status register"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x40 - masked interrupt status register"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x44 - interrupt clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x48 - DMA control register"]
    #[inline(always)]
    pub const fn dmacr(&self) -> &Dmacr {
        &self.dmacr
    }
    #[doc = "0xfd0 - primecell ID register 0"]
    #[inline(always)]
    pub const fn pcellid0(&self) -> &Pcellid0 {
        &self.pcellid0
    }
    #[doc = "0xfd4 - primecell ID register 1"]
    #[inline(always)]
    pub const fn pcellid1(&self) -> &Pcellid1 {
        &self.pcellid1
    }
    #[doc = "0xfd8 - primecell ID register 2"]
    #[inline(always)]
    pub const fn pcellid2(&self) -> &Pcellid2 {
        &self.pcellid2
    }
    #[doc = "0xfdc - primecell ID register 3"]
    #[inline(always)]
    pub const fn pcellid3(&self) -> &Pcellid3 {
        &self.pcellid3
    }
    #[doc = "0xfe0 - peripheral ID register 0"]
    #[inline(always)]
    pub const fn periphid0(&self) -> &Periphid0 {
        &self.periphid0
    }
    #[doc = "0xfe4 - peripheral ID register 1"]
    #[inline(always)]
    pub const fn periphid1(&self) -> &Periphid1 {
        &self.periphid1
    }
    #[doc = "0xfe8 - peripheral ID register 2"]
    #[inline(always)]
    pub const fn periphid2(&self) -> &Periphid2 {
        &self.periphid2
    }
    #[doc = "0xfec - peripheral ID register 3"]
    #[inline(always)]
    pub const fn periphid3(&self) -> &Periphid3 {
        &self.periphid3
    }
}
#[doc = "DR (rw) register accessor: data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "data register"]
pub mod dr;
#[doc = "RSR_ECR (rw) register accessor: receive status register / error clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsr_ecr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr_ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr_ecr`] module"]
#[doc(alias = "RSR_ECR")]
pub type RsrEcr = crate::Reg<rsr_ecr::RsrEcrSpec>;
#[doc = "receive status register / error clear register"]
pub mod rsr_ecr;
#[doc = "FR (r) register accessor: flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`fr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fr`] module"]
#[doc(alias = "FR")]
pub type Fr = crate::Reg<fr::FrSpec>;
#[doc = "flag register"]
pub mod fr;
#[doc = "ILPR (rw) register accessor: IRDA low power counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ilpr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ilpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ilpr`] module"]
#[doc(alias = "ILPR")]
pub type Ilpr = crate::Reg<ilpr::IlprSpec>;
#[doc = "IRDA low power counter register"]
pub mod ilpr;
#[doc = "IBRD (rw) register accessor: integer baudrate register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibrd::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibrd`] module"]
#[doc(alias = "IBRD")]
pub type Ibrd = crate::Reg<ibrd::IbrdSpec>;
#[doc = "integer baudrate register"]
pub mod ibrd;
#[doc = "FBRD (rw) register accessor: fractional baudrate register\n\nYou can [`read`](crate::Reg::read) this register and get [`fbrd::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbrd`] module"]
#[doc(alias = "FBRD")]
pub type Fbrd = crate::Reg<fbrd::FbrdSpec>;
#[doc = "fractional baudrate register"]
pub mod fbrd;
#[doc = "LCR_H (rw) register accessor: line control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr_h`] module"]
#[doc(alias = "LCR_H")]
pub type LcrH = crate::Reg<lcr_h::LcrHSpec>;
#[doc = "line control register"]
pub mod lcr_h;
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "control register"]
pub mod cr;
#[doc = "IFLS (rw) register accessor: interrupt fifo level select register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifls::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifls`] module"]
#[doc(alias = "IFLS")]
pub type Ifls = crate::Reg<ifls::IflsSpec>;
#[doc = "interrupt fifo level select register"]
pub mod ifls;
#[doc = "IMSC (rw) register accessor: interrupt mask set/clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`imsc::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imsc`] module"]
#[doc(alias = "IMSC")]
pub type Imsc = crate::Reg<imsc::ImscSpec>;
#[doc = "interrupt mask set/clear register"]
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
#[doc = "DMACR (rw) register accessor: DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacr`] module"]
#[doc(alias = "DMACR")]
pub type Dmacr = crate::Reg<dmacr::DmacrSpec>;
#[doc = "DMA control register"]
pub mod dmacr;
#[doc = "PERIPHID0 (r) register accessor: peripheral ID register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid0`] module"]
#[doc(alias = "PERIPHID0")]
pub type Periphid0 = crate::Reg<periphid0::Periphid0Spec>;
#[doc = "peripheral ID register 0"]
pub mod periphid0;
#[doc = "PERIPHID1 (r) register accessor: peripheral ID register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid1`] module"]
#[doc(alias = "PERIPHID1")]
pub type Periphid1 = crate::Reg<periphid1::Periphid1Spec>;
#[doc = "peripheral ID register 1"]
pub mod periphid1;
#[doc = "PERIPHID2 (r) register accessor: peripheral ID register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid2`] module"]
#[doc(alias = "PERIPHID2")]
pub type Periphid2 = crate::Reg<periphid2::Periphid2Spec>;
#[doc = "peripheral ID register 2"]
pub mod periphid2;
#[doc = "PERIPHID3 (r) register accessor: peripheral ID register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid3`] module"]
#[doc(alias = "PERIPHID3")]
pub type Periphid3 = crate::Reg<periphid3::Periphid3Spec>;
#[doc = "peripheral ID register 3"]
pub mod periphid3;
#[doc = "PCELLID0 (r) register accessor: primecell ID register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcellid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcellid0`] module"]
#[doc(alias = "PCELLID0")]
pub type Pcellid0 = crate::Reg<pcellid0::Pcellid0Spec>;
#[doc = "primecell ID register 0"]
pub mod pcellid0;
#[doc = "PCELLID1 (r) register accessor: primecell ID register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcellid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcellid1`] module"]
#[doc(alias = "PCELLID1")]
pub type Pcellid1 = crate::Reg<pcellid1::Pcellid1Spec>;
#[doc = "primecell ID register 1"]
pub mod pcellid1;
#[doc = "PCELLID2 (r) register accessor: primecell ID register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pcellid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcellid2`] module"]
#[doc(alias = "PCELLID2")]
pub type Pcellid2 = crate::Reg<pcellid2::Pcellid2Spec>;
#[doc = "primecell ID register 2"]
pub mod pcellid2;
#[doc = "PCELLID3 (r) register accessor: primecell ID register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pcellid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcellid3`] module"]
#[doc(alias = "PCELLID3")]
pub type Pcellid3 = crate::Reg<pcellid3::Pcellid3Spec>;
#[doc = "primecell ID register 3"]
pub mod pcellid3;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dr: Dr,
    rsc_ecr: RscEcr,
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
    _reserved14: [u8; 0x0f94],
    id: [Id; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x04 - receive status register / error clear register"]
    #[inline(always)]
    pub const fn rsc_ecr(&self) -> &RscEcr {
        &self.rsc_ecr
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
    #[doc = "0xfe0..0x1000 - ID registers"]
    #[inline(always)]
    pub const fn id(&self, n: usize) -> &Id {
        &self.id[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xfe0..0x1000 - ID registers"]
    #[inline(always)]
    pub fn id_iter(&self) -> impl Iterator<Item = &Id> {
        self.id.iter()
    }
}
#[doc = "DR (rw) register accessor: data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "data register"]
pub mod dr;
#[doc = "RSC_ECR (rw) register accessor: receive status register / error clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsc_ecr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsc_ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsc_ecr`] module"]
#[doc(alias = "RSC_ECR")]
pub type RscEcr = crate::Reg<rsc_ecr::RscEcrSpec>;
#[doc = "receive status register / error clear register"]
pub mod rsc_ecr;
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
#[doc = "ID (r) register accessor: ID registers\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`] module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "ID registers"]
pub mod id;

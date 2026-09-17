#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: Cr1,
    cr2: Cr2,
    smcr: Smcr,
    dier: Dier,
    sr: Sr,
    egr: Egr,
    ccmr1: Ccmr1,
    ccmr2: Ccmr2,
    ccer: Ccer,
    cnt: Cnt,
    psc: Psc,
    arr: Arr,
    _reserved12: [u8; 0x04],
    ccr0: Ccr0,
    ccr1: Ccr1,
    ccr2: Ccr2,
    ccr3: Ccr3,
    _reserved16: [u8; 0x04],
    dcr: Dcr,
    dmar: Dmar,
    or: Or,
}
impl RegisterBlock {
    #[doc = "0x00 - GPTIM control register 1, Address"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x04 - GPTIM control register 2, Address"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x08 - GPTIM slave Mode Control register, Address"]
    #[inline(always)]
    pub const fn smcr(&self) -> &Smcr {
        &self.smcr
    }
    #[doc = "0x0c - GPTIM DMA/interrupt enable register, Address"]
    #[inline(always)]
    pub const fn dier(&self) -> &Dier {
        &self.dier
    }
    #[doc = "0x10 - GPTIM status register, Address"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x14 - GPTIM event generation register, Address"]
    #[inline(always)]
    pub const fn egr(&self) -> &Egr {
        &self.egr
    }
    #[doc = "0x18 - GPTIM capture/compare mode register 1, Address"]
    #[inline(always)]
    pub const fn ccmr1(&self) -> &Ccmr1 {
        &self.ccmr1
    }
    #[doc = "0x1c - GPTIM capture/compare mode register 2, Address"]
    #[inline(always)]
    pub const fn ccmr2(&self) -> &Ccmr2 {
        &self.ccmr2
    }
    #[doc = "0x20 - GPTIM capture/compare enable register, Address"]
    #[inline(always)]
    pub const fn ccer(&self) -> &Ccer {
        &self.ccer
    }
    #[doc = "0x24 - GPTIM counter register, Address"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x28 - GPTIM prescaler register, Address"]
    #[inline(always)]
    pub const fn psc(&self) -> &Psc {
        &self.psc
    }
    #[doc = "0x2c - GPTIM auto-reload register, Address"]
    #[inline(always)]
    pub const fn arr(&self) -> &Arr {
        &self.arr
    }
    #[doc = "0x34 - GPTIM capture/compare register 0, Address"]
    #[inline(always)]
    pub const fn ccr0(&self) -> &Ccr0 {
        &self.ccr0
    }
    #[doc = "0x38 - GPTIM capture/compare register 1, Address"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    #[doc = "0x3c - GPTIM capture/compare register 2, Address"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    #[doc = "0x40 - GPTIM capture/compare register 3, Address"]
    #[inline(always)]
    pub const fn ccr3(&self) -> &Ccr3 {
        &self.ccr3
    }
    #[doc = "0x48 - GPTIM DMA control register, Address"]
    #[inline(always)]
    pub const fn dcr(&self) -> &Dcr {
        &self.dcr
    }
    #[doc = "0x4c - GPTIM DMA address for full transfer register, Address"]
    #[inline(always)]
    pub const fn dmar(&self) -> &Dmar {
        &self.dmar
    }
    #[doc = "0x50 - GPTIM option register, Address"]
    #[inline(always)]
    pub const fn or(&self) -> &Or {
        &self.or
    }
}
#[doc = "CR1 (rw) register accessor: GPTIM control register 1, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "GPTIM control register 1, Address"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: GPTIM control register 2, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "GPTIM control register 2, Address"]
pub mod cr2;
#[doc = "SMCR (rw) register accessor: GPTIM slave Mode Control register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcr`] module"]
#[doc(alias = "SMCR")]
pub type Smcr = crate::Reg<smcr::SmcrSpec>;
#[doc = "GPTIM slave Mode Control register, Address"]
pub mod smcr;
#[doc = "DIER (rw) register accessor: GPTIM DMA/interrupt enable register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier`] module"]
#[doc(alias = "DIER")]
pub type Dier = crate::Reg<dier::DierSpec>;
#[doc = "GPTIM DMA/interrupt enable register, Address"]
pub mod dier;
#[doc = "SR (rw) register accessor: GPTIM status register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "GPTIM status register, Address"]
pub mod sr;
#[doc = "EGR (w) register accessor: GPTIM event generation register, Address\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@egr`] module"]
#[doc(alias = "EGR")]
pub type Egr = crate::Reg<egr::EgrSpec>;
#[doc = "GPTIM event generation register, Address"]
pub mod egr;
#[doc = "CCMR1 (rw) register accessor: GPTIM capture/compare mode register 1, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1`] module"]
#[doc(alias = "CCMR1")]
pub type Ccmr1 = crate::Reg<ccmr1::Ccmr1Spec>;
#[doc = "GPTIM capture/compare mode register 1, Address"]
pub mod ccmr1;
#[doc = "CCMR2 (rw) register accessor: GPTIM capture/compare mode register 2, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2`] module"]
#[doc(alias = "CCMR2")]
pub type Ccmr2 = crate::Reg<ccmr2::Ccmr2Spec>;
#[doc = "GPTIM capture/compare mode register 2, Address"]
pub mod ccmr2;
#[doc = "CCER (rw) register accessor: GPTIM capture/compare enable register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccer::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer`] module"]
#[doc(alias = "CCER")]
pub type Ccer = crate::Reg<ccer::CcerSpec>;
#[doc = "GPTIM capture/compare enable register, Address"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: GPTIM counter register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "GPTIM counter register, Address"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: GPTIM prescaler register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`] module"]
#[doc(alias = "PSC")]
pub type Psc = crate::Reg<psc::PscSpec>;
#[doc = "GPTIM prescaler register, Address"]
pub mod psc;
#[doc = "ARR (rw) register accessor: GPTIM auto-reload register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`] module"]
#[doc(alias = "ARR")]
pub type Arr = crate::Reg<arr::ArrSpec>;
#[doc = "GPTIM auto-reload register, Address"]
pub mod arr;
#[doc = "CCR0 (rw) register accessor: GPTIM capture/compare register 0, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr0`] module"]
#[doc(alias = "CCR0")]
pub type Ccr0 = crate::Reg<ccr0::Ccr0Spec>;
#[doc = "GPTIM capture/compare register 0, Address"]
pub mod ccr0;
#[doc = "CCR1 (rw) register accessor: GPTIM capture/compare register 1, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`] module"]
#[doc(alias = "CCR1")]
pub type Ccr1 = crate::Reg<ccr1::Ccr1Spec>;
#[doc = "GPTIM capture/compare register 1, Address"]
pub mod ccr1;
#[doc = "CCR2 (rw) register accessor: GPTIM capture/compare register 2, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`] module"]
#[doc(alias = "CCR2")]
pub type Ccr2 = crate::Reg<ccr2::Ccr2Spec>;
#[doc = "GPTIM capture/compare register 2, Address"]
pub mod ccr2;
#[doc = "CCR3 (rw) register accessor: GPTIM capture/compare register 3, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3`] module"]
#[doc(alias = "CCR3")]
pub type Ccr3 = crate::Reg<ccr3::Ccr3Spec>;
#[doc = "GPTIM capture/compare register 3, Address"]
pub mod ccr3;
#[doc = "DCR (rw) register accessor: GPTIM DMA control register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`] module"]
#[doc(alias = "DCR")]
pub type Dcr = crate::Reg<dcr::DcrSpec>;
#[doc = "GPTIM DMA control register, Address"]
pub mod dcr;
#[doc = "DMAR (rw) register accessor: GPTIM DMA address for full transfer register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmar::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmar`] module"]
#[doc(alias = "DMAR")]
pub type Dmar = crate::Reg<dmar::DmarSpec>;
#[doc = "GPTIM DMA address for full transfer register, Address"]
pub mod dmar;
#[doc = "OR (rw) register accessor: GPTIM option register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@or`] module"]
#[doc(alias = "OR")]
pub type Or = crate::Reg<or::OrSpec>;
#[doc = "GPTIM option register, Address"]
pub mod or;

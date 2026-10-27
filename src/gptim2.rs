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
    #[doc = "0x24 - GPTIM counter count value, Address"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x28 - GPTIM prescaler value, counter clock is PSC plus 1, Address"]
    #[inline(always)]
    pub const fn psc(&self) -> &Psc {
        &self.psc
    }
    #[doc = "0x2c - GPTIM counter reload value, Address"]
    #[inline(always)]
    pub const fn arr(&self) -> &Arr {
        &self.arr
    }
    #[doc = "0x34 - GPTIM capture compare 0 value, Address"]
    #[inline(always)]
    pub const fn ccr0(&self) -> &Ccr0 {
        &self.ccr0
    }
    #[doc = "0x38 - GPTIM capture compare 1 value, Address"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    #[doc = "0x3c - GPTIM capture compare 2 value, Address"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    #[doc = "0x40 - GPTIM capture compare 3 value, Address"]
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
    #[doc = "0x50 - GPTIM option register (GPTIM2 remap layout)"]
    #[inline(always)]
    pub const fn or(&self) -> &Or {
        &self.or
    }
}
pub use crate::gptim0::arr;
pub use crate::gptim0::ccer;
pub use crate::gptim0::ccmr1;
pub use crate::gptim0::ccmr2;
pub use crate::gptim0::ccr0;
pub use crate::gptim0::ccr1;
pub use crate::gptim0::ccr2;
pub use crate::gptim0::ccr3;
pub use crate::gptim0::cnt;
pub use crate::gptim0::cr1;
pub use crate::gptim0::cr2;
pub use crate::gptim0::dcr;
pub use crate::gptim0::dier;
pub use crate::gptim0::dmar;
pub use crate::gptim0::egr;
pub use crate::gptim0::psc;
pub use crate::gptim0::smcr;
pub use crate::gptim0::sr;
pub use crate::gptim0::Arr;
pub use crate::gptim0::Ccer;
pub use crate::gptim0::Ccmr1;
pub use crate::gptim0::Ccmr2;
pub use crate::gptim0::Ccr0;
pub use crate::gptim0::Ccr1;
pub use crate::gptim0::Ccr2;
pub use crate::gptim0::Ccr3;
pub use crate::gptim0::Cnt;
pub use crate::gptim0::Cr1;
pub use crate::gptim0::Cr2;
pub use crate::gptim0::Dcr;
pub use crate::gptim0::Dier;
pub use crate::gptim0::Dmar;
pub use crate::gptim0::Egr;
pub use crate::gptim0::Psc;
pub use crate::gptim0::Smcr;
pub use crate::gptim0::Sr;
#[doc = "OR (rw) register accessor: GPTIM option register (GPTIM2 remap layout)\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@or`] module"]
#[doc(alias = "OR")]
pub type Or = crate::Reg<or::OrSpec>;
#[doc = "GPTIM option register (GPTIM2 remap layout)"]
pub mod or;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rngdet: Rngdet,
    rngsr: Rngsr,
    rngclk: Rngclk,
    rngcr: Rngcr,
    rngreseed: Rngreseed,
    rngdata: Rngdata,
    rngpknum: Rngpknum,
    rngpkres0: Rngpkres0,
    rngpkres1: Rngpkres1,
    rngpkres2: Rngpkres2,
}
impl RegisterBlock {
    #[doc = "0x00 - Detection control register"]
    #[inline(always)]
    pub const fn rngdet(&self) -> &Rngdet {
        &self.rngdet
    }
    #[doc = "0x04 - Status register"]
    #[inline(always)]
    pub const fn rngsr(&self) -> &Rngsr {
        &self.rngsr
    }
    #[doc = "0x08 - Clock control register"]
    #[inline(always)]
    pub const fn rngclk(&self) -> &Rngclk {
        &self.rngclk
    }
    #[doc = "0x0c - Control register"]
    #[inline(always)]
    pub const fn rngcr(&self) -> &Rngcr {
        &self.rngcr
    }
    #[doc = "0x10 - Reseed register"]
    #[inline(always)]
    pub const fn rngreseed(&self) -> &Rngreseed {
        &self.rngreseed
    }
    #[doc = "0x14 - Random data register"]
    #[inline(always)]
    pub const fn rngdata(&self) -> &Rngdata {
        &self.rngdata
    }
    #[doc = "0x18 - Poker test sample count"]
    #[inline(always)]
    pub const fn rngpknum(&self) -> &Rngpknum {
        &self.rngpknum
    }
    #[doc = "0x1c - Poker test result 0"]
    #[inline(always)]
    pub const fn rngpkres0(&self) -> &Rngpkres0 {
        &self.rngpkres0
    }
    #[doc = "0x20 - Poker test result 1"]
    #[inline(always)]
    pub const fn rngpkres1(&self) -> &Rngpkres1 {
        &self.rngpkres1
    }
    #[doc = "0x24 - Poker test result 2"]
    #[inline(always)]
    pub const fn rngpkres2(&self) -> &Rngpkres2 {
        &self.rngpkres2
    }
}
#[doc = "RNGDET (rw) register accessor: Detection control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngdet::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngdet::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngdet`] module"]
#[doc(alias = "RNGDET")]
pub type Rngdet = crate::Reg<rngdet::RngdetSpec>;
#[doc = "Detection control register"]
pub mod rngdet;
#[doc = "RNGSR (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngsr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngsr`] module"]
#[doc(alias = "RNGSR")]
pub type Rngsr = crate::Reg<rngsr::RngsrSpec>;
#[doc = "Status register"]
pub mod rngsr;
#[doc = "RNGCLK (rw) register accessor: Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngclk::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngclk`] module"]
#[doc(alias = "RNGCLK")]
pub type Rngclk = crate::Reg<rngclk::RngclkSpec>;
#[doc = "Clock control register"]
pub mod rngclk;
#[doc = "RNGCR (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngcr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngcr`] module"]
#[doc(alias = "RNGCR")]
pub type Rngcr = crate::Reg<rngcr::RngcrSpec>;
#[doc = "Control register"]
pub mod rngcr;
#[doc = "RNGRESEED (rw) register accessor: Reseed register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngreseed::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngreseed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngreseed`] module"]
#[doc(alias = "RNGRESEED")]
pub type Rngreseed = crate::Reg<rngreseed::RngreseedSpec>;
#[doc = "Reseed register"]
pub mod rngreseed;
#[doc = "RNGDATA (rw) register accessor: Random data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngdata::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngdata`] module"]
#[doc(alias = "RNGDATA")]
pub type Rngdata = crate::Reg<rngdata::RngdataSpec>;
#[doc = "Random data register"]
pub mod rngdata;
#[doc = "RNGPKNUM (rw) register accessor: Poker test sample count\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpknum::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngpknum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngpknum`] module"]
#[doc(alias = "RNGPKNUM")]
pub type Rngpknum = crate::Reg<rngpknum::RngpknumSpec>;
#[doc = "Poker test sample count"]
pub mod rngpknum;
#[doc = "RNGPKRES0 (rw) register accessor: Poker test result 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpkres0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngpkres0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngpkres0`] module"]
#[doc(alias = "RNGPKRES0")]
pub type Rngpkres0 = crate::Reg<rngpkres0::Rngpkres0Spec>;
#[doc = "Poker test result 0"]
pub mod rngpkres0;
#[doc = "RNGPKRES1 (rw) register accessor: Poker test result 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpkres1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngpkres1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngpkres1`] module"]
#[doc(alias = "RNGPKRES1")]
pub type Rngpkres1 = crate::Reg<rngpkres1::Rngpkres1Spec>;
#[doc = "Poker test result 1"]
pub mod rngpkres1;
#[doc = "RNGPKRES2 (rw) register accessor: Poker test result 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpkres2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngpkres2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngpkres2`] module"]
#[doc(alias = "RNGPKRES2")]
pub type Rngpkres2 = crate::Reg<rngpkres2::Rngpkres2Spec>;
#[doc = "Poker test result 2"]
pub mod rngpkres2;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    saecr: Saecr,
    saehcr: Saehcr,
    saesr: Saesr,
    saegpr0: Saegpr0,
    saegpr1: Saegpr1,
    saegpr2: Saegpr2,
    saegpr3: Saegpr3,
    saegpr4: Saegpr4,
    saegpr5: Saegpr5,
    saemaskcr: Saemaskcr,
    saemaskdat0: Saemaskdat0,
    saemaskdat1: Saemaskdat1,
    saemaskdat2: Saemaskdat2,
    saebufcr: Saebufcr,
}
impl RegisterBlock {
    #[doc = "0x00 - Common control register"]
    #[inline(always)]
    pub const fn saecr(&self) -> &Saecr {
        &self.saecr
    }
    #[doc = "0x04 - Security control register"]
    #[inline(always)]
    pub const fn saehcr(&self) -> &Saehcr {
        &self.saehcr
    }
    #[doc = "0x08 - Status register"]
    #[inline(always)]
    pub const fn saesr(&self) -> &Saesr {
        &self.saesr
    }
    #[doc = "0x0c - General-purpose register 0"]
    #[inline(always)]
    pub const fn saegpr0(&self) -> &Saegpr0 {
        &self.saegpr0
    }
    #[doc = "0x10 - General-purpose register 1"]
    #[inline(always)]
    pub const fn saegpr1(&self) -> &Saegpr1 {
        &self.saegpr1
    }
    #[doc = "0x14 - General-purpose register 2"]
    #[inline(always)]
    pub const fn saegpr2(&self) -> &Saegpr2 {
        &self.saegpr2
    }
    #[doc = "0x18 - General-purpose register 3"]
    #[inline(always)]
    pub const fn saegpr3(&self) -> &Saegpr3 {
        &self.saegpr3
    }
    #[doc = "0x1c - General-purpose register 4"]
    #[inline(always)]
    pub const fn saegpr4(&self) -> &Saegpr4 {
        &self.saegpr4
    }
    #[doc = "0x20 - General-purpose register 5"]
    #[inline(always)]
    pub const fn saegpr5(&self) -> &Saegpr5 {
        &self.saegpr5
    }
    #[doc = "0x24 - Mask control register"]
    #[inline(always)]
    pub const fn saemaskcr(&self) -> &Saemaskcr {
        &self.saemaskcr
    }
    #[doc = "0x28 - Mask data register 0"]
    #[inline(always)]
    pub const fn saemaskdat0(&self) -> &Saemaskdat0 {
        &self.saemaskdat0
    }
    #[doc = "0x2c - Mask data register 1"]
    #[inline(always)]
    pub const fn saemaskdat1(&self) -> &Saemaskdat1 {
        &self.saemaskdat1
    }
    #[doc = "0x30 - Mask data register 2"]
    #[inline(always)]
    pub const fn saemaskdat2(&self) -> &Saemaskdat2 {
        &self.saemaskdat2
    }
    #[doc = "0x34 - Buffer control register"]
    #[inline(always)]
    pub const fn saebufcr(&self) -> &Saebufcr {
        &self.saebufcr
    }
}
#[doc = "SAECR (rw) register accessor: Common control register\n\nYou can [`read`](crate::Reg::read) this register and get [`saecr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saecr`] module"]
#[doc(alias = "SAECR")]
pub type Saecr = crate::Reg<saecr::SaecrSpec>;
#[doc = "Common control register"]
pub mod saecr;
#[doc = "SAEHCR (rw) register accessor: Security control register\n\nYou can [`read`](crate::Reg::read) this register and get [`saehcr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saehcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saehcr`] module"]
#[doc(alias = "SAEHCR")]
pub type Saehcr = crate::Reg<saehcr::SaehcrSpec>;
#[doc = "Security control register"]
pub mod saehcr;
#[doc = "SAESR (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`saesr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saesr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saesr`] module"]
#[doc(alias = "SAESR")]
pub type Saesr = crate::Reg<saesr::SaesrSpec>;
#[doc = "Status register"]
pub mod saesr;
#[doc = "SAEGPR0 (rw) register accessor: General-purpose register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`saegpr0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saegpr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saegpr0`] module"]
#[doc(alias = "SAEGPR0")]
pub type Saegpr0 = crate::Reg<saegpr0::Saegpr0Spec>;
#[doc = "General-purpose register 0"]
pub mod saegpr0;
#[doc = "SAEGPR1 (rw) register accessor: General-purpose register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`saegpr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saegpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saegpr1`] module"]
#[doc(alias = "SAEGPR1")]
pub type Saegpr1 = crate::Reg<saegpr1::Saegpr1Spec>;
#[doc = "General-purpose register 1"]
pub mod saegpr1;
#[doc = "SAEGPR2 (rw) register accessor: General-purpose register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`saegpr2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saegpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saegpr2`] module"]
#[doc(alias = "SAEGPR2")]
pub type Saegpr2 = crate::Reg<saegpr2::Saegpr2Spec>;
#[doc = "General-purpose register 2"]
pub mod saegpr2;
#[doc = "SAEGPR3 (rw) register accessor: General-purpose register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`saegpr3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saegpr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saegpr3`] module"]
#[doc(alias = "SAEGPR3")]
pub type Saegpr3 = crate::Reg<saegpr3::Saegpr3Spec>;
#[doc = "General-purpose register 3"]
pub mod saegpr3;
#[doc = "SAEGPR4 (rw) register accessor: General-purpose register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`saegpr4::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saegpr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saegpr4`] module"]
#[doc(alias = "SAEGPR4")]
pub type Saegpr4 = crate::Reg<saegpr4::Saegpr4Spec>;
#[doc = "General-purpose register 4"]
pub mod saegpr4;
#[doc = "SAEGPR5 (rw) register accessor: General-purpose register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`saegpr5::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saegpr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saegpr5`] module"]
#[doc(alias = "SAEGPR5")]
pub type Saegpr5 = crate::Reg<saegpr5::Saegpr5Spec>;
#[doc = "General-purpose register 5"]
pub mod saegpr5;
#[doc = "SAEMASKCR (rw) register accessor: Mask control register\n\nYou can [`read`](crate::Reg::read) this register and get [`saemaskcr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saemaskcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saemaskcr`] module"]
#[doc(alias = "SAEMASKCR")]
pub type Saemaskcr = crate::Reg<saemaskcr::SaemaskcrSpec>;
#[doc = "Mask control register"]
pub mod saemaskcr;
#[doc = "SAEMASKDAT0 (rw) register accessor: Mask data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`saemaskdat0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saemaskdat0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saemaskdat0`] module"]
#[doc(alias = "SAEMASKDAT0")]
pub type Saemaskdat0 = crate::Reg<saemaskdat0::Saemaskdat0Spec>;
#[doc = "Mask data register 0"]
pub mod saemaskdat0;
#[doc = "SAEMASKDAT1 (rw) register accessor: Mask data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`saemaskdat1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saemaskdat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saemaskdat1`] module"]
#[doc(alias = "SAEMASKDAT1")]
pub type Saemaskdat1 = crate::Reg<saemaskdat1::Saemaskdat1Spec>;
#[doc = "Mask data register 1"]
pub mod saemaskdat1;
#[doc = "SAEMASKDAT2 (rw) register accessor: Mask data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`saemaskdat2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saemaskdat2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saemaskdat2`] module"]
#[doc(alias = "SAEMASKDAT2")]
pub type Saemaskdat2 = crate::Reg<saemaskdat2::Saemaskdat2Spec>;
#[doc = "Mask data register 2"]
pub mod saemaskdat2;
#[doc = "SAEBUFCR (rw) register accessor: Buffer control register\n\nYou can [`read`](crate::Reg::read) this register and get [`saebufcr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saebufcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saebufcr`] module"]
#[doc(alias = "SAEBUFCR")]
pub type Saebufcr = crate::Reg<saebufcr::SaebufcrSpec>;
#[doc = "Buffer control register"]
pub mod saebufcr;

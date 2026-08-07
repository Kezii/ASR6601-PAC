#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr0: Cr0,
    cr1: Cr1,
    sr0: Sr0,
    sr1: Sr1,
    cr2: Cr2,
    cr3: Cr3,
    cr4: Cr4,
    cr5: Cr5,
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
    #[doc = "0x08 - status register 0"]
    #[inline(always)]
    pub const fn sr0(&self) -> &Sr0 {
        &self.sr0
    }
    #[doc = "0x0c - status register 2"]
    #[inline(always)]
    pub const fn sr1(&self) -> &Sr1 {
        &self.sr1
    }
    #[doc = "0x10 - control register 3"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x14 - control register 4"]
    #[inline(always)]
    pub const fn cr3(&self) -> &Cr3 {
        &self.cr3
    }
    #[doc = "0x18 - control register 5"]
    #[inline(always)]
    pub const fn cr4(&self) -> &Cr4 {
        &self.cr4
    }
    #[doc = "0x1c - control register 6"]
    #[inline(always)]
    pub const fn cr5(&self) -> &Cr5 {
        &self.cr5
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
#[doc = "SR0 (rw) register accessor: status register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sr0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr0`] module"]
#[doc(alias = "SR0")]
pub type Sr0 = crate::Reg<sr0::Sr0Spec>;
#[doc = "status register 0"]
pub mod sr0;
#[doc = "SR1 (rw) register accessor: status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`] module"]
#[doc(alias = "SR1")]
pub type Sr1 = crate::Reg<sr1::Sr1Spec>;
#[doc = "status register 2"]
pub mod sr1;
#[doc = "CR2 (rw) register accessor: control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "control register 3"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: control register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`] module"]
#[doc(alias = "CR3")]
pub type Cr3 = crate::Reg<cr3::Cr3Spec>;
#[doc = "control register 4"]
pub mod cr3;
#[doc = "CR4 (rw) register accessor: control register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`cr4::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr4`] module"]
#[doc(alias = "CR4")]
pub type Cr4 = crate::Reg<cr4::Cr4Spec>;
#[doc = "control register 5"]
pub mod cr4;
#[doc = "CR5 (rw) register accessor: control register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`cr5::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr5`] module"]
#[doc(alias = "CR5")]
pub type Cr5 = crate::Reg<cr5::Cr5Spec>;
#[doc = "control register 6"]
pub mod cr5;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr0: Cr0,
    cr1: Cr1,
    cr2: Cr2,
    cr3: Cr3,
    cr4: Cr4,
    cr5: Cr5,
    cr6: Cr6,
    cr7: Cr7,
    cr8: Cr8,
    cr9: Cr9,
    cr10: Cr10,
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
    #[doc = "0x08 - control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x0c - control register 3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &Cr3 {
        &self.cr3
    }
    #[doc = "0x10 - control register 4"]
    #[inline(always)]
    pub const fn cr4(&self) -> &Cr4 {
        &self.cr4
    }
    #[doc = "0x14 - control register 5"]
    #[inline(always)]
    pub const fn cr5(&self) -> &Cr5 {
        &self.cr5
    }
    #[doc = "0x18 - control register 6"]
    #[inline(always)]
    pub const fn cr6(&self) -> &Cr6 {
        &self.cr6
    }
    #[doc = "0x1c - control register 7"]
    #[inline(always)]
    pub const fn cr7(&self) -> &Cr7 {
        &self.cr7
    }
    #[doc = "0x20 - control register 8"]
    #[inline(always)]
    pub const fn cr8(&self) -> &Cr8 {
        &self.cr8
    }
    #[doc = "0x24 - control register 9"]
    #[inline(always)]
    pub const fn cr9(&self) -> &Cr9 {
        &self.cr9
    }
    #[doc = "0x28 - control register 10"]
    #[inline(always)]
    pub const fn cr10(&self) -> &Cr10 {
        &self.cr10
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
#[doc = "CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`] module"]
#[doc(alias = "CR3")]
pub type Cr3 = crate::Reg<cr3::Cr3Spec>;
#[doc = "control register 3"]
pub mod cr3;
#[doc = "CR4 (rw) register accessor: control register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`cr4::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr4`] module"]
#[doc(alias = "CR4")]
pub type Cr4 = crate::Reg<cr4::Cr4Spec>;
#[doc = "control register 4"]
pub mod cr4;
#[doc = "CR5 (rw) register accessor: control register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`cr5::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr5`] module"]
#[doc(alias = "CR5")]
pub type Cr5 = crate::Reg<cr5::Cr5Spec>;
#[doc = "control register 5"]
pub mod cr5;
#[doc = "CR6 (rw) register accessor: control register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`cr6::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr6`] module"]
#[doc(alias = "CR6")]
pub type Cr6 = crate::Reg<cr6::Cr6Spec>;
#[doc = "control register 6"]
pub mod cr6;
#[doc = "CR7 (rw) register accessor: control register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`cr7::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr7`] module"]
#[doc(alias = "CR7")]
pub type Cr7 = crate::Reg<cr7::Cr7Spec>;
#[doc = "control register 7"]
pub mod cr7;
#[doc = "CR8 (rw) register accessor: control register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`cr8::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr8`] module"]
#[doc(alias = "CR8")]
pub type Cr8 = crate::Reg<cr8::Cr8Spec>;
#[doc = "control register 8"]
pub mod cr8;
#[doc = "CR9 (rw) register accessor: control register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`cr9::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr9`] module"]
#[doc(alias = "CR9")]
pub type Cr9 = crate::Reg<cr9::Cr9Spec>;
#[doc = "control register 9"]
pub mod cr9;
#[doc = "CR10 (rw) register accessor: control register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`cr10::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr10`] module"]
#[doc(alias = "CR10")]
pub type Cr10 = crate::Reg<cr10::Cr10Spec>;
#[doc = "control register 10"]
pub mod cr10;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr0: Cr0,
    cr1: Cr1,
    dr0: Dr0,
    dr1: Dr1,
    dr2: Dr2,
    dr3: Dr3,
    dr4: Dr4,
    dr5: Dr5,
    dr6: Dr6,
    dr7: Dr7,
    sr: Sr,
    cr2: Cr2,
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
    #[doc = "0x08 - data register 0"]
    #[inline(always)]
    pub const fn dr0(&self) -> &Dr0 {
        &self.dr0
    }
    #[doc = "0x0c - data register 1"]
    #[inline(always)]
    pub const fn dr1(&self) -> &Dr1 {
        &self.dr1
    }
    #[doc = "0x10 - data register 2"]
    #[inline(always)]
    pub const fn dr2(&self) -> &Dr2 {
        &self.dr2
    }
    #[doc = "0x14 - data register 3"]
    #[inline(always)]
    pub const fn dr3(&self) -> &Dr3 {
        &self.dr3
    }
    #[doc = "0x18 - data register 4"]
    #[inline(always)]
    pub const fn dr4(&self) -> &Dr4 {
        &self.dr4
    }
    #[doc = "0x1c - data register 5"]
    #[inline(always)]
    pub const fn dr5(&self) -> &Dr5 {
        &self.dr5
    }
    #[doc = "0x20 - data register 6"]
    #[inline(always)]
    pub const fn dr6(&self) -> &Dr6 {
        &self.dr6
    }
    #[doc = "0x24 - data register 7"]
    #[inline(always)]
    pub const fn dr7(&self) -> &Dr7 {
        &self.dr7
    }
    #[doc = "0x28 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x2c - control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
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
#[doc = "DR0 (rw) register accessor: data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dr0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr0`] module"]
#[doc(alias = "DR0")]
pub type Dr0 = crate::Reg<dr0::Dr0Spec>;
#[doc = "data register 0"]
pub mod dr0;
#[doc = "DR1 (rw) register accessor: data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr1`] module"]
#[doc(alias = "DR1")]
pub type Dr1 = crate::Reg<dr1::Dr1Spec>;
#[doc = "data register 1"]
pub mod dr1;
#[doc = "DR2 (rw) register accessor: data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dr2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr2`] module"]
#[doc(alias = "DR2")]
pub type Dr2 = crate::Reg<dr2::Dr2Spec>;
#[doc = "data register 2"]
pub mod dr2;
#[doc = "DR3 (rw) register accessor: data register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`dr3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr3`] module"]
#[doc(alias = "DR3")]
pub type Dr3 = crate::Reg<dr3::Dr3Spec>;
#[doc = "data register 3"]
pub mod dr3;
#[doc = "DR4 (rw) register accessor: data register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dr4::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr4`] module"]
#[doc(alias = "DR4")]
pub type Dr4 = crate::Reg<dr4::Dr4Spec>;
#[doc = "data register 4"]
pub mod dr4;
#[doc = "DR5 (rw) register accessor: data register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`dr5::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr5`] module"]
#[doc(alias = "DR5")]
pub type Dr5 = crate::Reg<dr5::Dr5Spec>;
#[doc = "data register 5"]
pub mod dr5;
#[doc = "DR6 (rw) register accessor: data register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`dr6::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr6`] module"]
#[doc(alias = "DR6")]
pub type Dr6 = crate::Reg<dr6::Dr6Spec>;
#[doc = "data register 6"]
pub mod dr6;
#[doc = "DR7 (rw) register accessor: data register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`dr7::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr7`] module"]
#[doc(alias = "DR7")]
pub type Dr7 = crate::Reg<dr7::Dr7Spec>;
#[doc = "data register 7"]
pub mod dr7;
#[doc = "SR (r) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;
#[doc = "CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "control register 2"]
pub mod cr2;

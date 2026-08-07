#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr0: Cr0,
    cr1: Cr1,
    cr2: Cr2,
    cgr0: Cgr0,
    cgr1: Cgr1,
    cgr2: Cgr2,
    rst0: Rst0,
    rst1: Rst1,
    rst_sr: RstSr,
    rst_cr: RstCr,
    sr: Sr,
    sr1: Sr1,
    cr3: Cr3,
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
    #[doc = "0x0c - clock generation register 0"]
    #[inline(always)]
    pub const fn cgr0(&self) -> &Cgr0 {
        &self.cgr0
    }
    #[doc = "0x10 - clock generation register 1"]
    #[inline(always)]
    pub const fn cgr1(&self) -> &Cgr1 {
        &self.cgr1
    }
    #[doc = "0x14 - clock generation register 2"]
    #[inline(always)]
    pub const fn cgr2(&self) -> &Cgr2 {
        &self.cgr2
    }
    #[doc = "0x18 - reset register 0"]
    #[inline(always)]
    pub const fn rst0(&self) -> &Rst0 {
        &self.rst0
    }
    #[doc = "0x1c - reset register 1"]
    #[inline(always)]
    pub const fn rst1(&self) -> &Rst1 {
        &self.rst1
    }
    #[doc = "0x20 - reset status register 0"]
    #[inline(always)]
    pub const fn rst_sr(&self) -> &RstSr {
        &self.rst_sr
    }
    #[doc = "0x24 - reset control register 0"]
    #[inline(always)]
    pub const fn rst_cr(&self) -> &RstCr {
        &self.rst_cr
    }
    #[doc = "0x28 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x2c - status register 1"]
    #[inline(always)]
    pub const fn sr1(&self) -> &Sr1 {
        &self.sr1
    }
    #[doc = "0x30 - control register 3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &Cr3 {
        &self.cr3
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
#[doc = "CGR0 (rw) register accessor: clock generation register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cgr0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgr0`] module"]
#[doc(alias = "CGR0")]
pub type Cgr0 = crate::Reg<cgr0::Cgr0Spec>;
#[doc = "clock generation register 0"]
pub mod cgr0;
#[doc = "CGR1 (rw) register accessor: clock generation register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cgr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgr1`] module"]
#[doc(alias = "CGR1")]
pub type Cgr1 = crate::Reg<cgr1::Cgr1Spec>;
#[doc = "clock generation register 1"]
pub mod cgr1;
#[doc = "CGR2 (rw) register accessor: clock generation register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cgr2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgr2`] module"]
#[doc(alias = "CGR2")]
pub type Cgr2 = crate::Reg<cgr2::Cgr2Spec>;
#[doc = "clock generation register 2"]
pub mod cgr2;
#[doc = "RST0 (rw) register accessor: reset register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rst0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst0`] module"]
#[doc(alias = "RST0")]
pub type Rst0 = crate::Reg<rst0::Rst0Spec>;
#[doc = "reset register 0"]
pub mod rst0;
#[doc = "RST1 (rw) register accessor: reset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rst1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst1`] module"]
#[doc(alias = "RST1")]
pub type Rst1 = crate::Reg<rst1::Rst1Spec>;
#[doc = "reset register 1"]
pub mod rst1;
#[doc = "RST_SR (rw) register accessor: reset status register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_sr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_sr`] module"]
#[doc(alias = "RST_SR")]
pub type RstSr = crate::Reg<rst_sr::RstSrSpec>;
#[doc = "reset status register 0"]
pub mod rst_sr;
#[doc = "RST_CR (rw) register accessor: reset control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_cr`] module"]
#[doc(alias = "RST_CR")]
pub type RstCr = crate::Reg<rst_cr::RstCrSpec>;
#[doc = "reset control register 0"]
pub mod rst_cr;
#[doc = "SR (r) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;
#[doc = "SR1 (r) register accessor: status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`] module"]
#[doc(alias = "SR1")]
pub type Sr1 = crate::Reg<sr1::Sr1Spec>;
#[doc = "status register 1"]
pub mod sr1;
#[doc = "CR3 (rw) register accessor: control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`] module"]
#[doc(alias = "CR3")]
pub type Cr3 = crate::Reg<cr3::Cr3Spec>;
#[doc = "control register 3"]
pub mod cr3;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    dr: Dr,
    init: Init,
    poly: Poly,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x08 - init value register"]
    #[inline(always)]
    pub const fn init(&self) -> &Init {
        &self.init
    }
    #[doc = "0x0c - polynomial register"]
    #[inline(always)]
    pub const fn poly(&self) -> &Poly {
        &self.poly
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "control register"]
pub mod cr;
#[doc = "DR (rw) register accessor: data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "data register"]
pub mod dr;
#[doc = "INIT (rw) register accessor: init value register\n\nYou can [`read`](crate::Reg::read) this register and get [`init::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@init`] module"]
#[doc(alias = "INIT")]
pub type Init = crate::Reg<init::InitSpec>;
#[doc = "init value register"]
pub mod init;
#[doc = "POLY (rw) register accessor: polynomial register\n\nYou can [`read`](crate::Reg::read) this register and get [`poly::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@poly`] module"]
#[doc(alias = "POLY")]
pub type Poly = crate::Reg<poly::PolySpec>;
#[doc = "polynomial register"]
pub mod poly;

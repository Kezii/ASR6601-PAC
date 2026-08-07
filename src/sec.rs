#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    int: Int,
    rst: Rst,
    sr: Sr,
    filter0: Filter0,
    filter1: Filter1,
    filter2: Filter2,
    filter3: Filter3,
}
impl RegisterBlock {
    #[doc = "0x00 - interrupt enable register"]
    #[inline(always)]
    pub const fn int(&self) -> &Int {
        &self.int
    }
    #[doc = "0x04 - reset enable register"]
    #[inline(always)]
    pub const fn rst(&self) -> &Rst {
        &self.rst
    }
    #[doc = "0x08 - status"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x0c - filter0"]
    #[inline(always)]
    pub const fn filter0(&self) -> &Filter0 {
        &self.filter0
    }
    #[doc = "0x10 - filter1"]
    #[inline(always)]
    pub const fn filter1(&self) -> &Filter1 {
        &self.filter1
    }
    #[doc = "0x14 - filter2"]
    #[inline(always)]
    pub const fn filter2(&self) -> &Filter2 {
        &self.filter2
    }
    #[doc = "0x18 - filter3"]
    #[inline(always)]
    pub const fn filter3(&self) -> &Filter3 {
        &self.filter3
    }
}
#[doc = "INT (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int`] module"]
#[doc(alias = "INT")]
pub type Int = crate::Reg<int::IntSpec>;
#[doc = "interrupt enable register"]
pub mod int;
#[doc = "RST (rw) register accessor: reset enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst`] module"]
#[doc(alias = "RST")]
pub type Rst = crate::Reg<rst::RstSpec>;
#[doc = "reset enable register"]
pub mod rst;
#[doc = "SR (rw) register accessor: status\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status"]
pub mod sr;
#[doc = "FILTER0 (rw) register accessor: filter0\n\nYou can [`read`](crate::Reg::read) this register and get [`filter0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter0`] module"]
#[doc(alias = "FILTER0")]
pub type Filter0 = crate::Reg<filter0::Filter0Spec>;
#[doc = "filter0"]
pub mod filter0;
#[doc = "FILTER1 (rw) register accessor: filter1\n\nYou can [`read`](crate::Reg::read) this register and get [`filter1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter1`] module"]
#[doc(alias = "FILTER1")]
pub type Filter1 = crate::Reg<filter1::Filter1Spec>;
#[doc = "filter1"]
pub mod filter1;
#[doc = "FILTER2 (rw) register accessor: filter2\n\nYou can [`read`](crate::Reg::read) this register and get [`filter2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter2`] module"]
#[doc(alias = "FILTER2")]
pub type Filter2 = crate::Reg<filter2::Filter2Spec>;
#[doc = "filter2"]
pub mod filter2;
#[doc = "FILTER3 (rw) register accessor: filter3\n\nYou can [`read`](crate::Reg::read) this register and get [`filter3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter3`] module"]
#[doc(alias = "FILTER3")]
pub type Filter3 = crate::Reg<filter3::Filter3Spec>;
#[doc = "filter3"]
pub mod filter3;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    sr: Sr,
    sar: Sar,
    dbr: Dbr,
    lcr: Lcr,
    wcr: Wcr,
    rst_cycl: RstCycl,
    bmr: Bmr,
    wfifo: Wfifo,
    wfifo_wptr: WfifoWptr,
    wfifo_rptr: WfifoRptr,
    rfifo: Rfifo,
    rfifo_wptr: RfifoWptr,
    rfifo_rptr: RfifoRptr,
    _reserved14: [u8; 0x08],
    wfifo_status: WfifoStatus,
    rfifo_status: RfifoStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x08 - slave address register"]
    #[inline(always)]
    pub const fn sar(&self) -> &Sar {
        &self.sar
    }
    #[doc = "0x0c - data buffer register"]
    #[inline(always)]
    pub const fn dbr(&self) -> &Dbr {
        &self.dbr
    }
    #[doc = "0x10 - load count register"]
    #[inline(always)]
    pub const fn lcr(&self) -> &Lcr {
        &self.lcr
    }
    #[doc = "0x14 - wait count register"]
    #[inline(always)]
    pub const fn wcr(&self) -> &Wcr {
        &self.wcr
    }
    #[doc = "0x18 - reset cycle register"]
    #[inline(always)]
    pub const fn rst_cycl(&self) -> &RstCycl {
        &self.rst_cycl
    }
    #[doc = "0x1c - bus monitor register"]
    #[inline(always)]
    pub const fn bmr(&self) -> &Bmr {
        &self.bmr
    }
    #[doc = "0x20 - write fifo register"]
    #[inline(always)]
    pub const fn wfifo(&self) -> &Wfifo {
        &self.wfifo
    }
    #[doc = "0x24 - write fifo write pointer register"]
    #[inline(always)]
    pub const fn wfifo_wptr(&self) -> &WfifoWptr {
        &self.wfifo_wptr
    }
    #[doc = "0x28 - write fifo read pointer register"]
    #[inline(always)]
    pub const fn wfifo_rptr(&self) -> &WfifoRptr {
        &self.wfifo_rptr
    }
    #[doc = "0x2c - read fifo register"]
    #[inline(always)]
    pub const fn rfifo(&self) -> &Rfifo {
        &self.rfifo
    }
    #[doc = "0x30 - read fifo write pointer register"]
    #[inline(always)]
    pub const fn rfifo_wptr(&self) -> &RfifoWptr {
        &self.rfifo_wptr
    }
    #[doc = "0x34 - read fifo read pointer register"]
    #[inline(always)]
    pub const fn rfifo_rptr(&self) -> &RfifoRptr {
        &self.rfifo_rptr
    }
    #[doc = "0x40 - write fifo status register"]
    #[inline(always)]
    pub const fn wfifo_status(&self) -> &WfifoStatus {
        &self.wfifo_status
    }
    #[doc = "0x44 - read fifo status register"]
    #[inline(always)]
    pub const fn rfifo_status(&self) -> &RfifoStatus {
        &self.rfifo_status
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "control register"]
pub mod cr;
#[doc = "SR (rw) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;
#[doc = "SAR (rw) register accessor: slave address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar`] module"]
#[doc(alias = "SAR")]
pub type Sar = crate::Reg<sar::SarSpec>;
#[doc = "slave address register"]
pub mod sar;
#[doc = "DBR (rw) register accessor: data buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbr`] module"]
#[doc(alias = "DBR")]
pub type Dbr = crate::Reg<dbr::DbrSpec>;
#[doc = "data buffer register"]
pub mod dbr;
#[doc = "LCR (rw) register accessor: load count register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`] module"]
#[doc(alias = "LCR")]
pub type Lcr = crate::Reg<lcr::LcrSpec>;
#[doc = "load count register"]
pub mod lcr;
#[doc = "WCR (rw) register accessor: wait count register\n\nYou can [`read`](crate::Reg::read) this register and get [`wcr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcr`] module"]
#[doc(alias = "WCR")]
pub type Wcr = crate::Reg<wcr::WcrSpec>;
#[doc = "wait count register"]
pub mod wcr;
#[doc = "RST_CYCL (rw) register accessor: reset cycle register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_cycl::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_cycl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_cycl`] module"]
#[doc(alias = "RST_CYCL")]
pub type RstCycl = crate::Reg<rst_cycl::RstCyclSpec>;
#[doc = "reset cycle register"]
pub mod rst_cycl;
#[doc = "BMR (r) register accessor: bus monitor register\n\nYou can [`read`](crate::Reg::read) this register and get [`bmr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmr`] module"]
#[doc(alias = "BMR")]
pub type Bmr = crate::Reg<bmr::BmrSpec>;
#[doc = "bus monitor register"]
pub mod bmr;
#[doc = "WFIFO (rw) register accessor: write fifo register\n\nYou can [`read`](crate::Reg::read) this register and get [`wfifo::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wfifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wfifo`] module"]
#[doc(alias = "WFIFO")]
pub type Wfifo = crate::Reg<wfifo::WfifoSpec>;
#[doc = "write fifo register"]
pub mod wfifo;
#[doc = "WFIFO_WPTR (rw) register accessor: write fifo write pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`wfifo_wptr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wfifo_wptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wfifo_wptr`] module"]
#[doc(alias = "WFIFO_WPTR")]
pub type WfifoWptr = crate::Reg<wfifo_wptr::WfifoWptrSpec>;
#[doc = "write fifo write pointer register"]
pub mod wfifo_wptr;
#[doc = "WFIFO_RPTR (rw) register accessor: write fifo read pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`wfifo_rptr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wfifo_rptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wfifo_rptr`] module"]
#[doc(alias = "WFIFO_RPTR")]
pub type WfifoRptr = crate::Reg<wfifo_rptr::WfifoRptrSpec>;
#[doc = "write fifo read pointer register"]
pub mod wfifo_rptr;
#[doc = "RFIFO (rw) register accessor: read fifo register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfifo::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifo`] module"]
#[doc(alias = "RFIFO")]
pub type Rfifo = crate::Reg<rfifo::RfifoSpec>;
#[doc = "read fifo register"]
pub mod rfifo;
#[doc = "RFIFO_WPTR (rw) register accessor: read fifo write pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfifo_wptr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfifo_wptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifo_wptr`] module"]
#[doc(alias = "RFIFO_WPTR")]
pub type RfifoWptr = crate::Reg<rfifo_wptr::RfifoWptrSpec>;
#[doc = "read fifo write pointer register"]
pub mod rfifo_wptr;
#[doc = "RFIFO_RPTR (rw) register accessor: read fifo read pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfifo_rptr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfifo_rptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifo_rptr`] module"]
#[doc(alias = "RFIFO_RPTR")]
pub type RfifoRptr = crate::Reg<rfifo_rptr::RfifoRptrSpec>;
#[doc = "read fifo read pointer register"]
pub mod rfifo_rptr;
#[doc = "WFIFO_STATUS (r) register accessor: write fifo status register\n\nYou can [`read`](crate::Reg::read) this register and get [`wfifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wfifo_status`] module"]
#[doc(alias = "WFIFO_STATUS")]
pub type WfifoStatus = crate::Reg<wfifo_status::WfifoStatusSpec>;
#[doc = "write fifo status register"]
pub mod wfifo_status;
#[doc = "RFIFO_STATUS (r) register accessor: read fifo status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifo_status`] module"]
#[doc(alias = "RFIFO_STATUS")]
pub type RfifoStatus = crate::Reg<rfifo_status::RfifoStatusSpec>;
#[doc = "read fifo status register"]
pub mod rfifo_status;

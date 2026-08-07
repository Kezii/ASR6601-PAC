#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    oer: Oer,
    otyper: Otyper,
    ier: Ier,
    per: Per,
    psr: Psr,
    idr: Idr,
    odr: Odr,
    brr: Brr,
    bsr: Bsr,
    dsr: Dsr,
    icr: Icr,
    ifr: Ifr,
    wucr: Wucr,
    wulvl: Wulvl,
    afrl: Afrl,
    afrh: Afrh,
    stop3_wucr: Stop3Wucr,
}
impl RegisterBlock {
    #[doc = "0x00 - output enable register"]
    #[inline(always)]
    pub const fn oer(&self) -> &Oer {
        &self.oer
    }
    #[doc = "0x04 - output type register"]
    #[inline(always)]
    pub const fn otyper(&self) -> &Otyper {
        &self.otyper
    }
    #[doc = "0x08 - input enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x0c - pull enable register"]
    #[inline(always)]
    pub const fn per(&self) -> &Per {
        &self.per
    }
    #[doc = "0x10 - pull select register"]
    #[inline(always)]
    pub const fn psr(&self) -> &Psr {
        &self.psr
    }
    #[doc = "0x14 - input data register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x18 - output data register"]
    #[inline(always)]
    pub const fn odr(&self) -> &Odr {
        &self.odr
    }
    #[doc = "0x1c - bit reset register"]
    #[inline(always)]
    pub const fn brr(&self) -> &Brr {
        &self.brr
    }
    #[doc = "0x20 - bit set register"]
    #[inline(always)]
    pub const fn bsr(&self) -> &Bsr {
        &self.bsr
    }
    #[doc = "0x24 - dirve set register"]
    #[inline(always)]
    pub const fn dsr(&self) -> &Dsr {
        &self.dsr
    }
    #[doc = "0x28 - interrupt control register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x2c - interrupt flag register"]
    #[inline(always)]
    pub const fn ifr(&self) -> &Ifr {
        &self.ifr
    }
    #[doc = "0x30 - wakeup control register"]
    #[inline(always)]
    pub const fn wucr(&self) -> &Wucr {
        &self.wucr
    }
    #[doc = "0x34 - wakeup level register"]
    #[inline(always)]
    pub const fn wulvl(&self) -> &Wulvl {
        &self.wulvl
    }
    #[doc = "0x38 - alternate function low register"]
    #[inline(always)]
    pub const fn afrl(&self) -> &Afrl {
        &self.afrl
    }
    #[doc = "0x3c - alternate function high register"]
    #[inline(always)]
    pub const fn afrh(&self) -> &Afrh {
        &self.afrh
    }
    #[doc = "0x40 - stop3 wakeup control register"]
    #[inline(always)]
    pub const fn stop3_wucr(&self) -> &Stop3Wucr {
        &self.stop3_wucr
    }
}
#[doc = "OER (rw) register accessor: output enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`oer::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oer`] module"]
#[doc(alias = "OER")]
pub type Oer = crate::Reg<oer::OerSpec>;
#[doc = "output enable register"]
pub mod oer;
#[doc = "OTYPER (rw) register accessor: output type register\n\nYou can [`read`](crate::Reg::read) this register and get [`otyper::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otyper`] module"]
#[doc(alias = "OTYPER")]
pub type Otyper = crate::Reg<otyper::OtyperSpec>;
#[doc = "output type register"]
pub mod otyper;
#[doc = "IER (rw) register accessor: input enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "input enable register"]
pub mod ier;
#[doc = "PER (rw) register accessor: pull enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`per::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per`] module"]
#[doc(alias = "PER")]
pub type Per = crate::Reg<per::PerSpec>;
#[doc = "pull enable register"]
pub mod per;
#[doc = "PSR (rw) register accessor: pull select register\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`] module"]
#[doc(alias = "PSR")]
pub type Psr = crate::Reg<psr::PsrSpec>;
#[doc = "pull select register"]
pub mod psr;
#[doc = "IDR (r) register accessor: input data register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "input data register"]
pub mod idr;
#[doc = "ODR (rw) register accessor: output data register\n\nYou can [`read`](crate::Reg::read) this register and get [`odr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`] module"]
#[doc(alias = "ODR")]
pub type Odr = crate::Reg<odr::OdrSpec>;
#[doc = "output data register"]
pub mod odr;
#[doc = "BRR (w) register accessor: bit reset register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`] module"]
#[doc(alias = "BRR")]
pub type Brr = crate::Reg<brr::BrrSpec>;
#[doc = "bit reset register"]
pub mod brr;
#[doc = "BSR (w) register accessor: bit set register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsr`] module"]
#[doc(alias = "BSR")]
pub type Bsr = crate::Reg<bsr::BsrSpec>;
#[doc = "bit set register"]
pub mod bsr;
#[doc = "DSR (rw) register accessor: dirve set register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr`] module"]
#[doc(alias = "DSR")]
pub type Dsr = crate::Reg<dsr::DsrSpec>;
#[doc = "dirve set register"]
pub mod dsr;
#[doc = "ICR (rw) register accessor: interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "interrupt control register"]
pub mod icr;
#[doc = "IFR (rw) register accessor: interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifr`] module"]
#[doc(alias = "IFR")]
pub type Ifr = crate::Reg<ifr::IfrSpec>;
#[doc = "interrupt flag register"]
pub mod ifr;
#[doc = "WUCR (rw) register accessor: wakeup control register\n\nYou can [`read`](crate::Reg::read) this register and get [`wucr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wucr`] module"]
#[doc(alias = "WUCR")]
pub type Wucr = crate::Reg<wucr::WucrSpec>;
#[doc = "wakeup control register"]
pub mod wucr;
#[doc = "WULVL (rw) register accessor: wakeup level register\n\nYou can [`read`](crate::Reg::read) this register and get [`wulvl::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wulvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wulvl`] module"]
#[doc(alias = "WULVL")]
pub type Wulvl = crate::Reg<wulvl::WulvlSpec>;
#[doc = "wakeup level register"]
pub mod wulvl;
#[doc = "AFRL (rw) register accessor: alternate function low register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrl`] module"]
#[doc(alias = "AFRL")]
pub type Afrl = crate::Reg<afrl::AfrlSpec>;
#[doc = "alternate function low register"]
pub mod afrl;
#[doc = "AFRH (rw) register accessor: alternate function high register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrh`] module"]
#[doc(alias = "AFRH")]
pub type Afrh = crate::Reg<afrh::AfrhSpec>;
#[doc = "alternate function high register"]
pub mod afrh;
#[doc = "STOP3_WUCR (rw) register accessor: stop3 wakeup control register\n\nYou can [`read`](crate::Reg::read) this register and get [`stop3_wucr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop3_wucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stop3_wucr`] module"]
#[doc(alias = "STOP3_WUCR")]
pub type Stop3Wucr = crate::Reg<stop3_wucr::Stop3WucrSpec>;
#[doc = "stop3 wakeup control register"]
pub mod stop3_wucr;

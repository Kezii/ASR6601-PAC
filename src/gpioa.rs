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
    bsrr: Bsrr,
    dsr: Dsr,
    int_cr: IntCr,
    fr: Fr,
    wu_en: WuEn,
    wu_lvl: WuLvl,
    afrl: Afrl,
    afrh: Afrh,
    stop3_wu_cr: Stop3WuCr,
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
    #[doc = "0x20 - bit set-clear register"]
    #[inline(always)]
    pub const fn bsrr(&self) -> &Bsrr {
        &self.bsrr
    }
    #[doc = "0x24 - drive strength register"]
    #[inline(always)]
    pub const fn dsr(&self) -> &Dsr {
        &self.dsr
    }
    #[doc = "0x28 - interrupt control register"]
    #[inline(always)]
    pub const fn int_cr(&self) -> &IntCr {
        &self.int_cr
    }
    #[doc = "0x2c - interrupt flag register"]
    #[inline(always)]
    pub const fn fr(&self) -> &Fr {
        &self.fr
    }
    #[doc = "0x30 - wakeup control register"]
    #[inline(always)]
    pub const fn wu_en(&self) -> &WuEn {
        &self.wu_en
    }
    #[doc = "0x34 - wakeup level register"]
    #[inline(always)]
    pub const fn wu_lvl(&self) -> &WuLvl {
        &self.wu_lvl
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
    pub const fn stop3_wu_cr(&self) -> &Stop3WuCr {
        &self.stop3_wu_cr
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
#[doc = "BSRR (w) register accessor: bit set-clear register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr`] module"]
#[doc(alias = "BSRR")]
pub type Bsrr = crate::Reg<bsrr::BsrrSpec>;
#[doc = "bit set-clear register"]
pub mod bsrr;
#[doc = "DSR (w) register accessor: drive strength register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr`] module"]
#[doc(alias = "DSR")]
pub type Dsr = crate::Reg<dsr::DsrSpec>;
#[doc = "drive strength register"]
pub mod dsr;
#[doc = "INT_CR (rw) register accessor: interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_cr`] module"]
#[doc(alias = "INT_CR")]
pub type IntCr = crate::Reg<int_cr::IntCrSpec>;
#[doc = "interrupt control register"]
pub mod int_cr;
#[doc = "FR (rw) register accessor: interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`fr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fr`] module"]
#[doc(alias = "FR")]
pub type Fr = crate::Reg<fr::FrSpec>;
#[doc = "interrupt flag register"]
pub mod fr;
#[doc = "WU_EN (rw) register accessor: wakeup control register\n\nYou can [`read`](crate::Reg::read) this register and get [`wu_en::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wu_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wu_en`] module"]
#[doc(alias = "WU_EN")]
pub type WuEn = crate::Reg<wu_en::WuEnSpec>;
#[doc = "wakeup control register"]
pub mod wu_en;
#[doc = "WU_LVL (rw) register accessor: wakeup level register\n\nYou can [`read`](crate::Reg::read) this register and get [`wu_lvl::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wu_lvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wu_lvl`] module"]
#[doc(alias = "WU_LVL")]
pub type WuLvl = crate::Reg<wu_lvl::WuLvlSpec>;
#[doc = "wakeup level register"]
pub mod wu_lvl;
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
#[doc = "STOP3_WU_CR (rw) register accessor: stop3 wakeup control register\n\nYou can [`read`](crate::Reg::read) this register and get [`stop3_wu_cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop3_wu_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stop3_wu_cr`] module"]
#[doc(alias = "STOP3_WU_CR")]
pub type Stop3WuCr = crate::Reg<stop3_wu_cr::Stop3WuCrSpec>;
#[doc = "stop3 wakeup control register"]
pub mod stop3_wu_cr;

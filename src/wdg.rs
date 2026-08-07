#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    load: Load,
    value: Value,
    control: Control,
    intclr: Intclr,
    ris: Ris,
    mis: Mis,
    _reserved6: [u8; 0x0be8],
    lock: Lock,
    _reserved7: [u8; 0x02fc],
    itcr: Itcr,
    itop: Itop,
    _reserved9: [u8; 0xc8],
    periphid4: Periphid4,
    periphid5: Periphid5,
    periphid6: Periphid6,
    periphid7: Periphid7,
    periphid0: Periphid0,
    periphid1: Periphid1,
    periphid2: Periphid2,
    periphid3: Periphid3,
    pcellid0: Pcellid0,
    pcellid1: Pcellid1,
    pcellid2: Pcellid2,
    pcellid3: Pcellid3,
}
impl RegisterBlock {
    #[doc = "0x00 - load register"]
    #[inline(always)]
    pub const fn load(&self) -> &Load {
        &self.load
    }
    #[doc = "0x04 - value register"]
    #[inline(always)]
    pub const fn value(&self) -> &Value {
        &self.value
    }
    #[doc = "0x08 - control register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x0c - clear interrupt register"]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
    #[doc = "0x10 - raw interrupt status register"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x14 - interrupt status register"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0xc00 - lock register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0xf00 - integration test control register"]
    #[inline(always)]
    pub const fn itcr(&self) -> &Itcr {
        &self.itcr
    }
    #[doc = "0xf04 - integration test output set register"]
    #[inline(always)]
    pub const fn itop(&self) -> &Itop {
        &self.itop
    }
    #[doc = "0xfd0 - peripheral id register 4"]
    #[inline(always)]
    pub const fn periphid4(&self) -> &Periphid4 {
        &self.periphid4
    }
    #[doc = "0xfd4 - peripheral id register 5"]
    #[inline(always)]
    pub const fn periphid5(&self) -> &Periphid5 {
        &self.periphid5
    }
    #[doc = "0xfd8 - peripheral id register 6"]
    #[inline(always)]
    pub const fn periphid6(&self) -> &Periphid6 {
        &self.periphid6
    }
    #[doc = "0xfdc - peripheral id register 7"]
    #[inline(always)]
    pub const fn periphid7(&self) -> &Periphid7 {
        &self.periphid7
    }
    #[doc = "0xfe0 - peripheral id register 0"]
    #[inline(always)]
    pub const fn periphid0(&self) -> &Periphid0 {
        &self.periphid0
    }
    #[doc = "0xfe4 - peripheral id register 1"]
    #[inline(always)]
    pub const fn periphid1(&self) -> &Periphid1 {
        &self.periphid1
    }
    #[doc = "0xfe8 - peripheral id register 2"]
    #[inline(always)]
    pub const fn periphid2(&self) -> &Periphid2 {
        &self.periphid2
    }
    #[doc = "0xfec - peripheral id register 3"]
    #[inline(always)]
    pub const fn periphid3(&self) -> &Periphid3 {
        &self.periphid3
    }
    #[doc = "0xff0 - component id register 0"]
    #[inline(always)]
    pub const fn pcellid0(&self) -> &Pcellid0 {
        &self.pcellid0
    }
    #[doc = "0xff4 - component id register 1"]
    #[inline(always)]
    pub const fn pcellid1(&self) -> &Pcellid1 {
        &self.pcellid1
    }
    #[doc = "0xff8 - component id register 2"]
    #[inline(always)]
    pub const fn pcellid2(&self) -> &Pcellid2 {
        &self.pcellid2
    }
    #[doc = "0xffc - component id register 3"]
    #[inline(always)]
    pub const fn pcellid3(&self) -> &Pcellid3 {
        &self.pcellid3
    }
}
#[doc = "LOAD (rw) register accessor: load register\n\nYou can [`read`](crate::Reg::read) this register and get [`load::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load`] module"]
#[doc(alias = "LOAD")]
pub type Load = crate::Reg<load::LoadSpec>;
#[doc = "load register"]
pub mod load;
#[doc = "VALUE (r) register accessor: value register\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`] module"]
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
#[doc = "value register"]
pub mod value;
#[doc = "CONTROL (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "control register"]
pub mod control;
#[doc = "INTCLR (w) register accessor: clear interrupt register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`] module"]
#[doc(alias = "INTCLR")]
pub type Intclr = crate::Reg<intclr::IntclrSpec>;
#[doc = "clear interrupt register"]
pub mod intclr;
#[doc = "RIS (r) register accessor: raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`] module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "raw interrupt status register"]
pub mod ris;
#[doc = "MIS (r) register accessor: interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`] module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "interrupt status register"]
pub mod mis;
#[doc = "LOCK (rw) register accessor: lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "lock register"]
pub mod lock;
#[doc = "ITCR (rw) register accessor: integration test control register\n\nYou can [`read`](crate::Reg::read) this register and get [`itcr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itcr`] module"]
#[doc(alias = "ITCR")]
pub type Itcr = crate::Reg<itcr::ItcrSpec>;
#[doc = "integration test control register"]
pub mod itcr;
#[doc = "ITOP (w) register accessor: integration test output set register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itop`] module"]
#[doc(alias = "ITOP")]
pub type Itop = crate::Reg<itop::ItopSpec>;
#[doc = "integration test output set register"]
pub mod itop;
#[doc = "PERIPHID4 (r) register accessor: peripheral id register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid4`] module"]
#[doc(alias = "PERIPHID4")]
pub type Periphid4 = crate::Reg<periphid4::Periphid4Spec>;
#[doc = "peripheral id register 4"]
pub mod periphid4;
#[doc = "PERIPHID5 (r) register accessor: peripheral id register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid5`] module"]
#[doc(alias = "PERIPHID5")]
pub type Periphid5 = crate::Reg<periphid5::Periphid5Spec>;
#[doc = "peripheral id register 5"]
pub mod periphid5;
#[doc = "PERIPHID6 (r) register accessor: peripheral id register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid6`] module"]
#[doc(alias = "PERIPHID6")]
pub type Periphid6 = crate::Reg<periphid6::Periphid6Spec>;
#[doc = "peripheral id register 6"]
pub mod periphid6;
#[doc = "PERIPHID7 (r) register accessor: peripheral id register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid7`] module"]
#[doc(alias = "PERIPHID7")]
pub type Periphid7 = crate::Reg<periphid7::Periphid7Spec>;
#[doc = "peripheral id register 7"]
pub mod periphid7;
#[doc = "PERIPHID0 (r) register accessor: peripheral id register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid0`] module"]
#[doc(alias = "PERIPHID0")]
pub type Periphid0 = crate::Reg<periphid0::Periphid0Spec>;
#[doc = "peripheral id register 0"]
pub mod periphid0;
#[doc = "PERIPHID1 (r) register accessor: peripheral id register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid1`] module"]
#[doc(alias = "PERIPHID1")]
pub type Periphid1 = crate::Reg<periphid1::Periphid1Spec>;
#[doc = "peripheral id register 1"]
pub mod periphid1;
#[doc = "PERIPHID2 (r) register accessor: peripheral id register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid2`] module"]
#[doc(alias = "PERIPHID2")]
pub type Periphid2 = crate::Reg<periphid2::Periphid2Spec>;
#[doc = "peripheral id register 2"]
pub mod periphid2;
#[doc = "PERIPHID3 (r) register accessor: peripheral id register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid3`] module"]
#[doc(alias = "PERIPHID3")]
pub type Periphid3 = crate::Reg<periphid3::Periphid3Spec>;
#[doc = "peripheral id register 3"]
pub mod periphid3;
#[doc = "PCELLID0 (r) register accessor: component id register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcellid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcellid0`] module"]
#[doc(alias = "PCELLID0")]
pub type Pcellid0 = crate::Reg<pcellid0::Pcellid0Spec>;
#[doc = "component id register 0"]
pub mod pcellid0;
#[doc = "PCELLID1 (r) register accessor: component id register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcellid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcellid1`] module"]
#[doc(alias = "PCELLID1")]
pub type Pcellid1 = crate::Reg<pcellid1::Pcellid1Spec>;
#[doc = "component id register 1"]
pub mod pcellid1;
#[doc = "PCELLID2 (r) register accessor: component id register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pcellid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcellid2`] module"]
#[doc(alias = "PCELLID2")]
pub type Pcellid2 = crate::Reg<pcellid2::Pcellid2Spec>;
#[doc = "component id register 2"]
pub mod pcellid2;
#[doc = "PCELLID3 (r) register accessor: component id register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pcellid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcellid3`] module"]
#[doc(alias = "PCELLID3")]
pub type Pcellid3 = crate::Reg<pcellid3::Pcellid3Spec>;
#[doc = "component id register 3"]
pub mod pcellid3;

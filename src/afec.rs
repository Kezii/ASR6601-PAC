#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    analog_02: Analog02,
    analog_03: Analog03,
    _reserved2: [u8; 0x04],
    analog_05: Analog05,
    analog_06: Analog06,
    _reserved4: [u8; 0x14],
    analog_0c: Analog0c,
    _reserved5: [u8; 0x10],
    analog_11: Analog11,
    analog_12: Analog12,
    _reserved7: [u8; 0x50],
    analog_27: Analog27,
    _reserved8: [u8; 0x10],
    analog_2c: Analog2c,
    _reserved9: [u8; 0x014c],
    cr: Cr,
    int_sr: IntSr,
    raw_sr: RawSr,
}
impl RegisterBlock {
    #[doc = "0x08 - Analog register 0x02 (RCO32K/XO32K control)"]
    #[inline(always)]
    pub const fn analog_02(&self) -> &Analog02 {
        &self.analog_02
    }
    #[doc = "0x0c - Analog register 0x03 (XO32K low-power control)"]
    #[inline(always)]
    pub const fn analog_03(&self) -> &Analog03 {
        &self.analog_03
    }
    #[doc = "0x14 - Analog register 0x05 (low-power-run control 0)"]
    #[inline(always)]
    pub const fn analog_05(&self) -> &Analog05 {
        &self.analog_05
    }
    #[doc = "0x18 - Analog register 0x06 (oscillator/low-power-run/LCD analog control)"]
    #[inline(always)]
    pub const fn analog_06(&self) -> &Analog06 {
        &self.analog_06
    }
    #[doc = "0x30 - Analog register 0x0C (STOP3 analog control)"]
    #[inline(always)]
    pub const fn analog_0c(&self) -> &Analog0c {
        &self.analog_0c
    }
    #[doc = "0x44 - Analog register 0x11 (ADC/DAC analog control)"]
    #[inline(always)]
    pub const fn analog_11(&self) -> &Analog11 {
        &self.analog_11
    }
    #[doc = "0x48 - Analog register 0x12 (ADC reference/DAC analog control)"]
    #[inline(always)]
    pub const fn analog_12(&self) -> &Analog12 {
        &self.analog_12
    }
    #[doc = "0x9c - Analog register 0x27 (DAC analog output control)"]
    #[inline(always)]
    pub const fn analog_27(&self) -> &Analog27 {
        &self.analog_27
    }
    #[doc = "0xb0 - Analog register 0x2C (VBAT/3 measurement-path control)"]
    #[inline(always)]
    pub const fn analog_2c(&self) -> &Analog2c {
        &self.analog_2c
    }
    #[doc = "0x200 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x204 - interrupt status register"]
    #[inline(always)]
    pub const fn int_sr(&self) -> &IntSr {
        &self.int_sr
    }
    #[doc = "0x208 - raw status register"]
    #[inline(always)]
    pub const fn raw_sr(&self) -> &RawSr {
        &self.raw_sr
    }
}
#[doc = "ANALOG_02 (rw) register accessor: Analog register 0x02 (RCO32K/XO32K control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_02::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_02`] module"]
#[doc(alias = "ANALOG_02")]
pub type Analog02 = crate::Reg<analog_02::Analog02Spec>;
#[doc = "Analog register 0x02 (RCO32K/XO32K control)"]
pub mod analog_02;
#[doc = "ANALOG_03 (rw) register accessor: Analog register 0x03 (XO32K low-power control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_03::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_03`] module"]
#[doc(alias = "ANALOG_03")]
pub type Analog03 = crate::Reg<analog_03::Analog03Spec>;
#[doc = "Analog register 0x03 (XO32K low-power control)"]
pub mod analog_03;
#[doc = "ANALOG_05 (rw) register accessor: Analog register 0x05 (low-power-run control 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_05::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_05::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_05`] module"]
#[doc(alias = "ANALOG_05")]
pub type Analog05 = crate::Reg<analog_05::Analog05Spec>;
#[doc = "Analog register 0x05 (low-power-run control 0)"]
pub mod analog_05;
#[doc = "ANALOG_06 (rw) register accessor: Analog register 0x06 (oscillator/low-power-run/LCD analog control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_06::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_06::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_06`] module"]
#[doc(alias = "ANALOG_06")]
pub type Analog06 = crate::Reg<analog_06::Analog06Spec>;
#[doc = "Analog register 0x06 (oscillator/low-power-run/LCD analog control)"]
pub mod analog_06;
#[doc = "ANALOG_0C (rw) register accessor: Analog register 0x0C (STOP3 analog control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_0c::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_0c`] module"]
#[doc(alias = "ANALOG_0C")]
pub type Analog0c = crate::Reg<analog_0c::Analog0cSpec>;
#[doc = "Analog register 0x0C (STOP3 analog control)"]
pub mod analog_0c;
#[doc = "ANALOG_11 (rw) register accessor: Analog register 0x11 (ADC/DAC analog control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_11::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_11`] module"]
#[doc(alias = "ANALOG_11")]
pub type Analog11 = crate::Reg<analog_11::Analog11Spec>;
#[doc = "Analog register 0x11 (ADC/DAC analog control)"]
pub mod analog_11;
#[doc = "ANALOG_12 (rw) register accessor: Analog register 0x12 (ADC reference/DAC analog control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_12::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_12`] module"]
#[doc(alias = "ANALOG_12")]
pub type Analog12 = crate::Reg<analog_12::Analog12Spec>;
#[doc = "Analog register 0x12 (ADC reference/DAC analog control)"]
pub mod analog_12;
#[doc = "ANALOG_27 (rw) register accessor: Analog register 0x27 (DAC analog output control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_27::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_27`] module"]
#[doc(alias = "ANALOG_27")]
pub type Analog27 = crate::Reg<analog_27::Analog27Spec>;
#[doc = "Analog register 0x27 (DAC analog output control)"]
pub mod analog_27;
#[doc = "ANALOG_2C (rw) register accessor: Analog register 0x2C (VBAT/3 measurement-path control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_2c::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_2c`] module"]
#[doc(alias = "ANALOG_2C")]
pub type Analog2c = crate::Reg<analog_2c::Analog2cSpec>;
#[doc = "Analog register 0x2C (VBAT/3 measurement-path control)"]
pub mod analog_2c;
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "control register"]
pub mod cr;
#[doc = "INT_SR (rw) register accessor: interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_sr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_sr`] module"]
#[doc(alias = "INT_SR")]
pub type IntSr = crate::Reg<int_sr::IntSrSpec>;
#[doc = "interrupt status register"]
pub mod int_sr;
#[doc = "RAW_SR (r) register accessor: raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_sr`] module"]
#[doc(alias = "RAW_SR")]
pub type RawSr = crate::Reg<raw_sr::RawSrSpec>;
#[doc = "raw status register"]
pub mod raw_sr;

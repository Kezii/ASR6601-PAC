#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    alarm0: Alarm0,
    alarm1: Alarm1,
    ppmadjust: Ppmadjust,
    calendar: Calendar,
    calendar_h: CalendarH,
    cyc_max_value: CycMaxValue,
    sr: Sr,
    asyndata: Asyndata,
    asyndata_h: AsyndataH,
    cr1: Cr1,
    sr1: Sr1,
    cr2: Cr2,
    sub_second: SubSecond,
    cyc_cnt_value: CycCntValue,
    alarm0_sub: Alarm0Sub,
    alarm1_sub: Alarm1Sub,
    calendar_r: CalendarR,
    calendar_r_h: CalendarRH,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - alarm 0"]
    #[inline(always)]
    pub const fn alarm0(&self) -> &Alarm0 {
        &self.alarm0
    }
    #[doc = "0x08 - alarm 1"]
    #[inline(always)]
    pub const fn alarm1(&self) -> &Alarm1 {
        &self.alarm1
    }
    #[doc = "0x0c - ppm adjust value"]
    #[inline(always)]
    pub const fn ppmadjust(&self) -> &Ppmadjust {
        &self.ppmadjust
    }
    #[doc = "0x10 - time hour/minute/second"]
    #[inline(always)]
    pub const fn calendar(&self) -> &Calendar {
        &self.calendar
    }
    #[doc = "0x14 - time year/month/date"]
    #[inline(always)]
    pub const fn calendar_h(&self) -> &CalendarH {
        &self.calendar_h
    }
    #[doc = "0x18 - cyc max value"]
    #[inline(always)]
    pub const fn cyc_max_value(&self) -> &CycMaxValue {
        &self.cyc_max_value
    }
    #[doc = "0x1c - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x20 - asynchronization time hour/minute/second"]
    #[inline(always)]
    pub const fn asyndata(&self) -> &Asyndata {
        &self.asyndata
    }
    #[doc = "0x24 - asynchronization time year/month/date"]
    #[inline(always)]
    pub const fn asyndata_h(&self) -> &AsyndataH {
        &self.asyndata_h
    }
    #[doc = "0x28 - control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x2c - status register 1"]
    #[inline(always)]
    pub const fn sr1(&self) -> &Sr1 {
        &self.sr1
    }
    #[doc = "0x30 - control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x34 - subsecond counter"]
    #[inline(always)]
    pub const fn sub_second(&self) -> &SubSecond {
        &self.sub_second
    }
    #[doc = "0x38 - cyc counter"]
    #[inline(always)]
    pub const fn cyc_cnt_value(&self) -> &CycCntValue {
        &self.cyc_cnt_value
    }
    #[doc = "0x3c - alarm0 subsecond"]
    #[inline(always)]
    pub const fn alarm0_sub(&self) -> &Alarm0Sub {
        &self.alarm0_sub
    }
    #[doc = "0x40 - alarm1 subsecond"]
    #[inline(always)]
    pub const fn alarm1_sub(&self) -> &Alarm1Sub {
        &self.alarm1_sub
    }
    #[doc = "0x44 - read time hour/minute/second"]
    #[inline(always)]
    pub const fn calendar_r(&self) -> &CalendarR {
        &self.calendar_r
    }
    #[doc = "0x48 - read time year/month/date"]
    #[inline(always)]
    pub const fn calendar_r_h(&self) -> &CalendarRH {
        &self.calendar_r_h
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "control register"]
pub mod cr;
#[doc = "ALARM0 (rw) register accessor: alarm 0\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm0`] module"]
#[doc(alias = "ALARM0")]
pub type Alarm0 = crate::Reg<alarm0::Alarm0Spec>;
#[doc = "alarm 0"]
pub mod alarm0;
#[doc = "ALARM1 (rw) register accessor: alarm 1\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm1`] module"]
#[doc(alias = "ALARM1")]
pub type Alarm1 = crate::Reg<alarm1::Alarm1Spec>;
#[doc = "alarm 1"]
pub mod alarm1;
#[doc = "PPMADJUST (rw) register accessor: ppm adjust value\n\nYou can [`read`](crate::Reg::read) this register and get [`ppmadjust::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppmadjust::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppmadjust`] module"]
#[doc(alias = "PPMADJUST")]
pub type Ppmadjust = crate::Reg<ppmadjust::PpmadjustSpec>;
#[doc = "ppm adjust value"]
pub mod ppmadjust;
#[doc = "CALENDAR (w) register accessor: time hour/minute/second\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calendar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calendar`] module"]
#[doc(alias = "CALENDAR")]
pub type Calendar = crate::Reg<calendar::CalendarSpec>;
#[doc = "time hour/minute/second"]
pub mod calendar;
#[doc = "CALENDAR_H (w) register accessor: time year/month/date\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calendar_h::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calendar_h`] module"]
#[doc(alias = "CALENDAR_H")]
pub type CalendarH = crate::Reg<calendar_h::CalendarHSpec>;
#[doc = "time year/month/date"]
pub mod calendar_h;
#[doc = "CYC_MAX_VALUE (rw) register accessor: cyc max value\n\nYou can [`read`](crate::Reg::read) this register and get [`cyc_max_value::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cyc_max_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cyc_max_value`] module"]
#[doc(alias = "CYC_MAX_VALUE")]
pub type CycMaxValue = crate::Reg<cyc_max_value::CycMaxValueSpec>;
#[doc = "cyc max value"]
pub mod cyc_max_value;
#[doc = "SR (rw) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;
#[doc = "ASYNDATA (r) register accessor: asynchronization time hour/minute/second\n\nYou can [`read`](crate::Reg::read) this register and get [`asyndata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyndata`] module"]
#[doc(alias = "ASYNDATA")]
pub type Asyndata = crate::Reg<asyndata::AsyndataSpec>;
#[doc = "asynchronization time hour/minute/second"]
pub mod asyndata;
#[doc = "ASYNDATA_H (r) register accessor: asynchronization time year/month/date\n\nYou can [`read`](crate::Reg::read) this register and get [`asyndata_h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyndata_h`] module"]
#[doc(alias = "ASYNDATA_H")]
pub type AsyndataH = crate::Reg<asyndata_h::AsyndataHSpec>;
#[doc = "asynchronization time year/month/date"]
pub mod asyndata_h;
#[doc = "CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "SR1 (r) register accessor: status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`] module"]
#[doc(alias = "SR1")]
pub type Sr1 = crate::Reg<sr1::Sr1Spec>;
#[doc = "status register 1"]
pub mod sr1;
#[doc = "CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "SUB_SECOND (r) register accessor: subsecond counter\n\nYou can [`read`](crate::Reg::read) this register and get [`sub_second::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sub_second`] module"]
#[doc(alias = "SUB_SECOND")]
pub type SubSecond = crate::Reg<sub_second::SubSecondSpec>;
#[doc = "subsecond counter"]
pub mod sub_second;
#[doc = "CYC_CNT_VALUE (r) register accessor: cyc counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cyc_cnt_value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cyc_cnt_value`] module"]
#[doc(alias = "CYC_CNT_VALUE")]
pub type CycCntValue = crate::Reg<cyc_cnt_value::CycCntValueSpec>;
#[doc = "cyc counter"]
pub mod cyc_cnt_value;
#[doc = "ALARM0_SUB (rw) register accessor: alarm0 subsecond\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm0_sub::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm0_sub::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm0_sub`] module"]
#[doc(alias = "ALARM0_SUB")]
pub type Alarm0Sub = crate::Reg<alarm0_sub::Alarm0SubSpec>;
#[doc = "alarm0 subsecond"]
pub mod alarm0_sub;
#[doc = "ALARM1_SUB (rw) register accessor: alarm1 subsecond\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm1_sub::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm1_sub::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm1_sub`] module"]
#[doc(alias = "ALARM1_SUB")]
pub type Alarm1Sub = crate::Reg<alarm1_sub::Alarm1SubSpec>;
#[doc = "alarm1 subsecond"]
pub mod alarm1_sub;
#[doc = "CALENDAR_R (r) register accessor: read time hour/minute/second\n\nYou can [`read`](crate::Reg::read) this register and get [`calendar_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calendar_r`] module"]
#[doc(alias = "CALENDAR_R")]
pub type CalendarR = crate::Reg<calendar_r::CalendarRSpec>;
#[doc = "read time hour/minute/second"]
pub mod calendar_r;
#[doc = "CALENDAR_R_H (r) register accessor: read time year/month/date\n\nYou can [`read`](crate::Reg::read) this register and get [`calendar_r_h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calendar_r_h`] module"]
#[doc(alias = "CALENDAR_R_H")]
pub type CalendarRH = crate::Reg<calendar_r_h::CalendarRHSpec>;
#[doc = "read time year/month/date"]
pub mod calendar_r_h;

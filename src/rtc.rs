#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    alarm0: Alarm0,
    alarm1: Alarm1,
    ppm_adjust: PpmAdjust,
    calendar: Calendar,
    calendar_h: CalendarH,
    cyc_max: CycMax,
    sr: Sr,
    asyn_data: AsynData,
    asyn_data_h: AsynDataH,
    cr1: Cr1,
    sr1: Sr1,
    cr2: Cr2,
    sub_second_cnt: SubSecondCnt,
    cyc_cnt: CycCnt,
    alarm0_subsecond: Alarm0Subsecond,
    alarm1_subsecond: Alarm1Subsecond,
    calendar_r: CalendarR,
    calendar_r_h: CalendarRH,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
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
    pub const fn ppm_adjust(&self) -> &PpmAdjust {
        &self.ppm_adjust
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
    pub const fn cyc_max(&self) -> &CycMax {
        &self.cyc_max
    }
    #[doc = "0x1c - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x20 - asynchronization time hour/minute/second"]
    #[inline(always)]
    pub const fn asyn_data(&self) -> &AsynData {
        &self.asyn_data
    }
    #[doc = "0x24 - asynchronization time year/month/date"]
    #[inline(always)]
    pub const fn asyn_data_h(&self) -> &AsynDataH {
        &self.asyn_data_h
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
    pub const fn sub_second_cnt(&self) -> &SubSecondCnt {
        &self.sub_second_cnt
    }
    #[doc = "0x38 - cyc counter"]
    #[inline(always)]
    pub const fn cyc_cnt(&self) -> &CycCnt {
        &self.cyc_cnt
    }
    #[doc = "0x3c - alarm0 subsecond"]
    #[inline(always)]
    pub const fn alarm0_subsecond(&self) -> &Alarm0Subsecond {
        &self.alarm0_subsecond
    }
    #[doc = "0x40 - alarm1 subsecond"]
    #[inline(always)]
    pub const fn alarm1_subsecond(&self) -> &Alarm1Subsecond {
        &self.alarm1_subsecond
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
#[doc = "CTRL (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "control register"]
pub mod ctrl;
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
#[doc = "PPM_ADJUST (rw) register accessor: ppm adjust value\n\nYou can [`read`](crate::Reg::read) this register and get [`ppm_adjust::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppm_adjust::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppm_adjust`] module"]
#[doc(alias = "PPM_ADJUST")]
pub type PpmAdjust = crate::Reg<ppm_adjust::PpmAdjustSpec>;
#[doc = "ppm adjust value"]
pub mod ppm_adjust;
#[doc = "CALENDAR (rw) register accessor: time hour/minute/second\n\nYou can [`read`](crate::Reg::read) this register and get [`calendar::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calendar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calendar`] module"]
#[doc(alias = "CALENDAR")]
pub type Calendar = crate::Reg<calendar::CalendarSpec>;
#[doc = "time hour/minute/second"]
pub mod calendar;
#[doc = "CALENDAR_H (rw) register accessor: time year/month/date\n\nYou can [`read`](crate::Reg::read) this register and get [`calendar_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calendar_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calendar_h`] module"]
#[doc(alias = "CALENDAR_H")]
pub type CalendarH = crate::Reg<calendar_h::CalendarHSpec>;
#[doc = "time year/month/date"]
pub mod calendar_h;
#[doc = "CYC_MAX (rw) register accessor: cyc max value\n\nYou can [`read`](crate::Reg::read) this register and get [`cyc_max::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cyc_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cyc_max`] module"]
#[doc(alias = "CYC_MAX")]
pub type CycMax = crate::Reg<cyc_max::CycMaxSpec>;
#[doc = "cyc max value"]
pub mod cyc_max;
#[doc = "SR (rw) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;
#[doc = "ASYN_DATA (r) register accessor: asynchronization time hour/minute/second\n\nYou can [`read`](crate::Reg::read) this register and get [`asyn_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyn_data`] module"]
#[doc(alias = "ASYN_DATA")]
pub type AsynData = crate::Reg<asyn_data::AsynDataSpec>;
#[doc = "asynchronization time hour/minute/second"]
pub mod asyn_data;
#[doc = "ASYN_DATA_H (r) register accessor: asynchronization time year/month/date\n\nYou can [`read`](crate::Reg::read) this register and get [`asyn_data_h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyn_data_h`] module"]
#[doc(alias = "ASYN_DATA_H")]
pub type AsynDataH = crate::Reg<asyn_data_h::AsynDataHSpec>;
#[doc = "asynchronization time year/month/date"]
pub mod asyn_data_h;
#[doc = "CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "SR1 (rw) register accessor: status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`] module"]
#[doc(alias = "SR1")]
pub type Sr1 = crate::Reg<sr1::Sr1Spec>;
#[doc = "status register 1"]
pub mod sr1;
#[doc = "CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "SUB_SECOND_CNT (r) register accessor: subsecond counter\n\nYou can [`read`](crate::Reg::read) this register and get [`sub_second_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sub_second_cnt`] module"]
#[doc(alias = "SUB_SECOND_CNT")]
pub type SubSecondCnt = crate::Reg<sub_second_cnt::SubSecondCntSpec>;
#[doc = "subsecond counter"]
pub mod sub_second_cnt;
#[doc = "CYC_CNT (r) register accessor: cyc counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cyc_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cyc_cnt`] module"]
#[doc(alias = "CYC_CNT")]
pub type CycCnt = crate::Reg<cyc_cnt::CycCntSpec>;
#[doc = "cyc counter"]
pub mod cyc_cnt;
#[doc = "ALARM0_SUBSECOND (rw) register accessor: alarm0 subsecond\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm0_subsecond::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm0_subsecond::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm0_subsecond`] module"]
#[doc(alias = "ALARM0_SUBSECOND")]
pub type Alarm0Subsecond = crate::Reg<alarm0_subsecond::Alarm0SubsecondSpec>;
#[doc = "alarm0 subsecond"]
pub mod alarm0_subsecond;
#[doc = "ALARM1_SUBSECOND (rw) register accessor: alarm1 subsecond\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm1_subsecond::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm1_subsecond::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm1_subsecond`] module"]
#[doc(alias = "ALARM1_SUBSECOND")]
pub type Alarm1Subsecond = crate::Reg<alarm1_subsecond::Alarm1SubsecondSpec>;
#[doc = "alarm1 subsecond"]
pub mod alarm1_subsecond;
#[doc = "CALENDAR_R (rw) register accessor: read time hour/minute/second\n\nYou can [`read`](crate::Reg::read) this register and get [`calendar_r::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calendar_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calendar_r`] module"]
#[doc(alias = "CALENDAR_R")]
pub type CalendarR = crate::Reg<calendar_r::CalendarRSpec>;
#[doc = "read time hour/minute/second"]
pub mod calendar_r;
#[doc = "CALENDAR_R_H (rw) register accessor: read time year/month/date\n\nYou can [`read`](crate::Reg::read) this register and get [`calendar_r_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calendar_r_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calendar_r_h`] module"]
#[doc(alias = "CALENDAR_R_H")]
pub type CalendarRH = crate::Reg<calendar_r_h::CalendarRHSpec>;
#[doc = "read time year/month/date"]
pub mod calendar_r_h;

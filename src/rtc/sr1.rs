#[doc = "Register `SR1` reader"]
pub type R = crate::R<Sr1Spec>;
#[doc = "Field `READ_CALENDAR_DONE` reader - Read calendar done"]
pub type ReadCalendarDoneR = crate::BitReader;
#[doc = "Field `WRITE_RTCSR_DONE` reader - Write rtc sr done"]
pub type WriteRtcsrDoneR = crate::BitReader;
#[doc = "Field `WRITE_CYC_MAX_VALUE_DONE` reader - Write cyc max value done"]
pub type WriteCycMaxValueDoneR = crate::BitReader;
#[doc = "Field `WRITE_CALENDAR_DONE` reader - Write calendar done"]
pub type WriteCalendarDoneR = crate::BitReader;
#[doc = "Field `WRITE_PPMADJUST_DONE` reader - Write ppm adjust done"]
pub type WritePpmadjustDoneR = crate::BitReader;
#[doc = "Field `WRITE_ALARM1_DONE` reader - Write alarm1 done"]
pub type WriteAlarm1DoneR = crate::BitReader;
#[doc = "Field `WRITE_ALARM0_DONE` reader - Write alarm0 done"]
pub type WriteAlarm0DoneR = crate::BitReader;
#[doc = "Field `WRITE_RTCCR_DONE` reader - Write rtc cr done"]
pub type WriteRtccrDoneR = crate::BitReader;
#[doc = "Field `WRITE_RTCCR2_DONE` reader - Write rtc cr2 done"]
pub type WriteRtccr2DoneR = crate::BitReader;
#[doc = "Field `SECOND_SR` reader - Second status"]
pub type SecondSrR = crate::BitReader;
#[doc = "Field `WRITE_ALARM1_SUB_DONE` reader - Write alarm1 sub done"]
pub type WriteAlarm1SubDoneR = crate::BitReader;
#[doc = "Field `WRITE_ALARM0_SUB_DONE` reader - Write alarm0 sub done"]
pub type WriteAlarm0SubDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read calendar done"]
    #[inline(always)]
    pub fn read_calendar_done(&self) -> ReadCalendarDoneR {
        ReadCalendarDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write rtc sr done"]
    #[inline(always)]
    pub fn write_rtcsr_done(&self) -> WriteRtcsrDoneR {
        WriteRtcsrDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write cyc max value done"]
    #[inline(always)]
    pub fn write_cyc_max_value_done(&self) -> WriteCycMaxValueDoneR {
        WriteCycMaxValueDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write calendar done"]
    #[inline(always)]
    pub fn write_calendar_done(&self) -> WriteCalendarDoneR {
        WriteCalendarDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write ppm adjust done"]
    #[inline(always)]
    pub fn write_ppmadjust_done(&self) -> WritePpmadjustDoneR {
        WritePpmadjustDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write alarm1 done"]
    #[inline(always)]
    pub fn write_alarm1_done(&self) -> WriteAlarm1DoneR {
        WriteAlarm1DoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write alarm0 done"]
    #[inline(always)]
    pub fn write_alarm0_done(&self) -> WriteAlarm0DoneR {
        WriteAlarm0DoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write rtc cr done"]
    #[inline(always)]
    pub fn write_rtccr_done(&self) -> WriteRtccrDoneR {
        WriteRtccrDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write rtc cr2 done"]
    #[inline(always)]
    pub fn write_rtccr2_done(&self) -> WriteRtccr2DoneR {
        WriteRtccr2DoneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Second status"]
    #[inline(always)]
    pub fn second_sr(&self) -> SecondSrR {
        SecondSrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write alarm1 sub done"]
    #[inline(always)]
    pub fn write_alarm1_sub_done(&self) -> WriteAlarm1SubDoneR {
        WriteAlarm1SubDoneR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write alarm0 sub done"]
    #[inline(always)]
    pub fn write_alarm0_sub_done(&self) -> WriteAlarm0SubDoneR {
        WriteAlarm0SubDoneR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr1Spec;
impl crate::RegisterSpec for Sr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for Sr1Spec {}

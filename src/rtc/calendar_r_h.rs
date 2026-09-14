#[doc = "Register `CALENDAR_R_H` reader"]
pub type R = crate::R<CalendarRHSpec>;
#[doc = "Field `CALENDAR_H_SYNC` reader - Calendar high sync value"]
pub type CalendarHSyncR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Calendar high sync value"]
    #[inline(always)]
    pub fn calendar_h_sync(&self) -> CalendarHSyncR {
        CalendarHSyncR::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "read time year/month/date\n\nYou can [`read`](crate::Reg::read) this register and get [`calendar_r_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalendarRHSpec;
impl crate::RegisterSpec for CalendarRHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calendar_r_h::R`](R) reader structure"]
impl crate::Readable for CalendarRHSpec {}

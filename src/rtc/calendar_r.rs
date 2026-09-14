#[doc = "Register `CALENDAR_R` reader"]
pub type R = crate::R<CalendarRSpec>;
#[doc = "Field `CALENDAR_SYNC` reader - Calendar sync value"]
pub type CalendarSyncR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - Calendar sync value"]
    #[inline(always)]
    pub fn calendar_sync(&self) -> CalendarSyncR {
        CalendarSyncR::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "read time hour/minute/second\n\nYou can [`read`](crate::Reg::read) this register and get [`calendar_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalendarRSpec;
impl crate::RegisterSpec for CalendarRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calendar_r::R`](R) reader structure"]
impl crate::Readable for CalendarRSpec {}

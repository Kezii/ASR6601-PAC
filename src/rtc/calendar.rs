#[doc = "Register `CALENDAR` writer"]
pub type W = crate::W<CalendarSpec>;
#[doc = "Field `CALENDAR_VALUE` writer - Calendar value"]
pub type CalendarValueW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl W {
    #[doc = "Bits 0:19 - Calendar value"]
    #[inline(always)]
    pub fn calendar_value(&mut self) -> CalendarValueW<'_, CalendarSpec> {
        CalendarValueW::new(self, 0)
    }
}
#[doc = "time hour/minute/second\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calendar::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalendarSpec;
impl crate::RegisterSpec for CalendarSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`calendar::W`](W) writer structure"]
impl crate::Writable for CalendarSpec {
    type Safety = crate::Unsafe;
}

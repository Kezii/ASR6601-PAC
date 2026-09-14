#[doc = "Register `CALENDAR_H` writer"]
pub type W = crate::W<CalendarHSpec>;
#[doc = "Field `CALENDAR_H_VALUE` writer - Calendar high value"]
pub type CalendarHValueW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl W {
    #[doc = "Bits 0:21 - Calendar high value"]
    #[inline(always)]
    pub fn calendar_h_value(&mut self) -> CalendarHValueW<'_, CalendarHSpec> {
        CalendarHValueW::new(self, 0)
    }
}
#[doc = "time year/month/date\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calendar_h::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalendarHSpec;
impl crate::RegisterSpec for CalendarHSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`calendar_h::W`](W) writer structure"]
impl crate::Writable for CalendarHSpec {
    type Safety = crate::Unsafe;
}

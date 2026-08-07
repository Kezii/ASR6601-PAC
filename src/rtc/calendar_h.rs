#[doc = "Register `CALENDAR_H` reader"]
pub type R = crate::R<CalendarHSpec>;
#[doc = "Register `CALENDAR_H` writer"]
pub type W = crate::W<CalendarHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "time year/month/date\n\nYou can [`read`](crate::Reg::read) this register and get [`calendar_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calendar_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalendarHSpec;
impl crate::RegisterSpec for CalendarHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calendar_h::R`](R) reader structure"]
impl crate::Readable for CalendarHSpec {}
#[doc = "`write(|w| ..)` method takes [`calendar_h::W`](W) writer structure"]
impl crate::Writable for CalendarHSpec {
    type Safety = crate::Unsafe;
}

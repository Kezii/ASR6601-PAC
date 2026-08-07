#[doc = "Register `CALENDAR_R_H` reader"]
pub type R = crate::R<CalendarRHSpec>;
#[doc = "Register `CALENDAR_R_H` writer"]
pub type W = crate::W<CalendarRHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "read time year/month/date\n\nYou can [`read`](crate::Reg::read) this register and get [`calendar_r_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calendar_r_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalendarRHSpec;
impl crate::RegisterSpec for CalendarRHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calendar_r_h::R`](R) reader structure"]
impl crate::Readable for CalendarRHSpec {}
#[doc = "`write(|w| ..)` method takes [`calendar_r_h::W`](W) writer structure"]
impl crate::Writable for CalendarRHSpec {
    type Safety = crate::Unsafe;
}

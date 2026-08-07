#[doc = "Register `CALENDAR_R` reader"]
pub type R = crate::R<CalendarRSpec>;
#[doc = "Register `CALENDAR_R` writer"]
pub type W = crate::W<CalendarRSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "read time hour/minute/second\n\nYou can [`read`](crate::Reg::read) this register and get [`calendar_r::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calendar_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalendarRSpec;
impl crate::RegisterSpec for CalendarRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calendar_r::R`](R) reader structure"]
impl crate::Readable for CalendarRSpec {}
#[doc = "`write(|w| ..)` method takes [`calendar_r::W`](W) writer structure"]
impl crate::Writable for CalendarRSpec {
    type Safety = crate::Unsafe;
}

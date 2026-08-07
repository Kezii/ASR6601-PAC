#[doc = "Register `CALENDAR` reader"]
pub type R = crate::R<CalendarSpec>;
#[doc = "Register `CALENDAR` writer"]
pub type W = crate::W<CalendarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "time hour/minute/second\n\nYou can [`read`](crate::Reg::read) this register and get [`calendar::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calendar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalendarSpec;
impl crate::RegisterSpec for CalendarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calendar::R`](R) reader structure"]
impl crate::Readable for CalendarSpec {}
#[doc = "`write(|w| ..)` method takes [`calendar::W`](W) writer structure"]
impl crate::Writable for CalendarSpec {
    type Safety = crate::Unsafe;
}

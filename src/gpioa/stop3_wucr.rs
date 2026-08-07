#[doc = "Register `STOP3_WUCR` reader"]
pub type R = crate::R<Stop3WucrSpec>;
#[doc = "Register `STOP3_WUCR` writer"]
pub type W = crate::W<Stop3WucrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "stop3 wakeup control register\n\nYou can [`read`](crate::Reg::read) this register and get [`stop3_wucr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop3_wucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stop3WucrSpec;
impl crate::RegisterSpec for Stop3WucrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stop3_wucr::R`](R) reader structure"]
impl crate::Readable for Stop3WucrSpec {}
#[doc = "`write(|w| ..)` method takes [`stop3_wucr::W`](W) writer structure"]
impl crate::Writable for Stop3WucrSpec {
    type Safety = crate::Unsafe;
}

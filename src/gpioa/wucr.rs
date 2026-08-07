#[doc = "Register `WUCR` reader"]
pub type R = crate::R<WucrSpec>;
#[doc = "Register `WUCR` writer"]
pub type W = crate::W<WucrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "wakeup control register\n\nYou can [`read`](crate::Reg::read) this register and get [`wucr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WucrSpec;
impl crate::RegisterSpec for WucrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wucr::R`](R) reader structure"]
impl crate::Readable for WucrSpec {}
#[doc = "`write(|w| ..)` method takes [`wucr::W`](W) writer structure"]
impl crate::Writable for WucrSpec {
    type Safety = crate::Unsafe;
}

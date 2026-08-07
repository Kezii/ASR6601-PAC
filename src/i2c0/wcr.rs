#[doc = "Register `WCR` reader"]
pub type R = crate::R<WcrSpec>;
#[doc = "Register `WCR` writer"]
pub type W = crate::W<WcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "wait count register\n\nYou can [`read`](crate::Reg::read) this register and get [`wcr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WcrSpec;
impl crate::RegisterSpec for WcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wcr::R`](R) reader structure"]
impl crate::Readable for WcrSpec {}
#[doc = "`write(|w| ..)` method takes [`wcr::W`](W) writer structure"]
impl crate::Writable for WcrSpec {
    type Safety = crate::Unsafe;
}

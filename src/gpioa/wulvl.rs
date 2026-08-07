#[doc = "Register `WULVL` reader"]
pub type R = crate::R<WulvlSpec>;
#[doc = "Register `WULVL` writer"]
pub type W = crate::W<WulvlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "wakeup level register\n\nYou can [`read`](crate::Reg::read) this register and get [`wulvl::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wulvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WulvlSpec;
impl crate::RegisterSpec for WulvlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wulvl::R`](R) reader structure"]
impl crate::Readable for WulvlSpec {}
#[doc = "`write(|w| ..)` method takes [`wulvl::W`](W) writer structure"]
impl crate::Writable for WulvlSpec {
    type Safety = crate::Unsafe;
}

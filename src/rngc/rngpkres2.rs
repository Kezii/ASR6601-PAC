#[doc = "Register `RNGPKRES2` reader"]
pub type R = crate::R<Rngpkres2Spec>;
#[doc = "Register `RNGPKRES2` writer"]
pub type W = crate::W<Rngpkres2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Poker test result 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpkres2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngpkres2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rngpkres2Spec;
impl crate::RegisterSpec for Rngpkres2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngpkres2::R`](R) reader structure"]
impl crate::Readable for Rngpkres2Spec {}
#[doc = "`write(|w| ..)` method takes [`rngpkres2::W`](W) writer structure"]
impl crate::Writable for Rngpkres2Spec {
    type Safety = crate::Unsafe;
}

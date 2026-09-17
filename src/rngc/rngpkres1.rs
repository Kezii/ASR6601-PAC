#[doc = "Register `RNGPKRES1` reader"]
pub type R = crate::R<Rngpkres1Spec>;
#[doc = "Register `RNGPKRES1` writer"]
pub type W = crate::W<Rngpkres1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Poker test result 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpkres1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngpkres1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rngpkres1Spec;
impl crate::RegisterSpec for Rngpkres1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngpkres1::R`](R) reader structure"]
impl crate::Readable for Rngpkres1Spec {}
#[doc = "`write(|w| ..)` method takes [`rngpkres1::W`](W) writer structure"]
impl crate::Writable for Rngpkres1Spec {
    type Safety = crate::Unsafe;
}

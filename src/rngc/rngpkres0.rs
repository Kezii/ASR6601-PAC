#[doc = "Register `RNGPKRES0` reader"]
pub type R = crate::R<Rngpkres0Spec>;
#[doc = "Register `RNGPKRES0` writer"]
pub type W = crate::W<Rngpkres0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Poker test result 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpkres0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngpkres0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rngpkres0Spec;
impl crate::RegisterSpec for Rngpkres0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngpkres0::R`](R) reader structure"]
impl crate::Readable for Rngpkres0Spec {}
#[doc = "`write(|w| ..)` method takes [`rngpkres0::W`](W) writer structure"]
impl crate::Writable for Rngpkres0Spec {
    type Safety = crate::Unsafe;
}

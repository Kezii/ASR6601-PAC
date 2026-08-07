#[doc = "Register `CCMR1` reader"]
pub type R = crate::R<Ccmr1Spec>;
#[doc = "Register `CCMR1` writer"]
pub type W = crate::W<Ccmr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TIMER capture/compare mode register 1, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr1Spec;
impl crate::RegisterSpec for Ccmr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1::R`](R) reader structure"]
impl crate::Readable for Ccmr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ccmr1::W`](W) writer structure"]
impl crate::Writable for Ccmr1Spec {
    type Safety = crate::Unsafe;
}

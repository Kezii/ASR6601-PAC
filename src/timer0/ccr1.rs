#[doc = "Register `CCR1` reader"]
pub type R = crate::R<Ccr1Spec>;
#[doc = "Register `CCR1` writer"]
pub type W = crate::W<Ccr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TIMER capture/compare register 1, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr1Spec;
impl crate::RegisterSpec for Ccr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr1::R`](R) reader structure"]
impl crate::Readable for Ccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr1::W`](W) writer structure"]
impl crate::Writable for Ccr1Spec {
    type Safety = crate::Unsafe;
}

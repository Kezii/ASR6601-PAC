#[doc = "Register `CCR0` reader"]
pub type R = crate::R<Ccr0Spec>;
#[doc = "Register `CCR0` writer"]
pub type W = crate::W<Ccr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TIMER capture/compare register 0, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr0Spec;
impl crate::RegisterSpec for Ccr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr0::R`](R) reader structure"]
impl crate::Readable for Ccr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr0::W`](W) writer structure"]
impl crate::Writable for Ccr0Spec {
    type Safety = crate::Unsafe;
}

#[doc = "Register `SSP_CR1` reader"]
pub type R = crate::R<SspCr1Spec>;
#[doc = "Register `SSP_CR1` writer"]
pub type W = crate::W<SspCr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ssp control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_cr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspCr1Spec;
impl crate::RegisterSpec for SspCr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssp_cr1::R`](R) reader structure"]
impl crate::Readable for SspCr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ssp_cr1::W`](W) writer structure"]
impl crate::Writable for SspCr1Spec {
    type Safety = crate::Unsafe;
}

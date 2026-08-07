#[doc = "Register `SSP_CR0` reader"]
pub type R = crate::R<SspCr0Spec>;
#[doc = "Register `SSP_CR0` writer"]
pub type W = crate::W<SspCr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ssp control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_cr0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspCr0Spec;
impl crate::RegisterSpec for SspCr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssp_cr0::R`](R) reader structure"]
impl crate::Readable for SspCr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ssp_cr0::W`](W) writer structure"]
impl crate::Writable for SspCr0Spec {
    type Safety = crate::Unsafe;
}

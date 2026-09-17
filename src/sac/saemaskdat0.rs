#[doc = "Register `SAEMASKDAT0` reader"]
pub type R = crate::R<Saemaskdat0Spec>;
#[doc = "Register `SAEMASKDAT0` writer"]
pub type W = crate::W<Saemaskdat0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Mask data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`saemaskdat0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saemaskdat0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Saemaskdat0Spec;
impl crate::RegisterSpec for Saemaskdat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saemaskdat0::R`](R) reader structure"]
impl crate::Readable for Saemaskdat0Spec {}
#[doc = "`write(|w| ..)` method takes [`saemaskdat0::W`](W) writer structure"]
impl crate::Writable for Saemaskdat0Spec {
    type Safety = crate::Unsafe;
}

#[doc = "Register `SAEMASKDAT2` reader"]
pub type R = crate::R<Saemaskdat2Spec>;
#[doc = "Register `SAEMASKDAT2` writer"]
pub type W = crate::W<Saemaskdat2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Mask data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`saemaskdat2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saemaskdat2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Saemaskdat2Spec;
impl crate::RegisterSpec for Saemaskdat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saemaskdat2::R`](R) reader structure"]
impl crate::Readable for Saemaskdat2Spec {}
#[doc = "`write(|w| ..)` method takes [`saemaskdat2::W`](W) writer structure"]
impl crate::Writable for Saemaskdat2Spec {
    type Safety = crate::Unsafe;
}

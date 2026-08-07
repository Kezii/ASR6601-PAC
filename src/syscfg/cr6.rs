#[doc = "Register `CR6` reader"]
pub type R = crate::R<Cr6Spec>;
#[doc = "Register `CR6` writer"]
pub type W = crate::W<Cr6Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "control register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`cr6::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr6Spec;
impl crate::RegisterSpec for Cr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr6::R`](R) reader structure"]
impl crate::Readable for Cr6Spec {}
#[doc = "`write(|w| ..)` method takes [`cr6::W`](W) writer structure"]
impl crate::Writable for Cr6Spec {
    type Safety = crate::Unsafe;
}

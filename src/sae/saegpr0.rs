#[doc = "Register `SAEGPR0` reader"]
pub type R = crate::R<Saegpr0Spec>;
#[doc = "Register `SAEGPR0` writer"]
pub type W = crate::W<Saegpr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "General-purpose register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`saegpr0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saegpr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Saegpr0Spec;
impl crate::RegisterSpec for Saegpr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saegpr0::R`](R) reader structure"]
impl crate::Readable for Saegpr0Spec {}
#[doc = "`write(|w| ..)` method takes [`saegpr0::W`](W) writer structure"]
impl crate::Writable for Saegpr0Spec {
    type Safety = crate::Unsafe;
}

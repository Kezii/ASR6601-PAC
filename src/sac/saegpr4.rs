#[doc = "Register `SAEGPR4` reader"]
pub type R = crate::R<Saegpr4Spec>;
#[doc = "Register `SAEGPR4` writer"]
pub type W = crate::W<Saegpr4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "General-purpose register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`saegpr4::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saegpr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Saegpr4Spec;
impl crate::RegisterSpec for Saegpr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saegpr4::R`](R) reader structure"]
impl crate::Readable for Saegpr4Spec {}
#[doc = "`write(|w| ..)` method takes [`saegpr4::W`](W) writer structure"]
impl crate::Writable for Saegpr4Spec {
    type Safety = crate::Unsafe;
}

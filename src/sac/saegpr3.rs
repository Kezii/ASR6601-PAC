#[doc = "Register `SAEGPR3` reader"]
pub type R = crate::R<Saegpr3Spec>;
#[doc = "Register `SAEGPR3` writer"]
pub type W = crate::W<Saegpr3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "General-purpose register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`saegpr3::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saegpr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Saegpr3Spec;
impl crate::RegisterSpec for Saegpr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saegpr3::R`](R) reader structure"]
impl crate::Readable for Saegpr3Spec {}
#[doc = "`write(|w| ..)` method takes [`saegpr3::W`](W) writer structure"]
impl crate::Writable for Saegpr3Spec {
    type Safety = crate::Unsafe;
}

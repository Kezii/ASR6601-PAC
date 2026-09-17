#[doc = "Register `SAEGPR2` reader"]
pub type R = crate::R<Saegpr2Spec>;
#[doc = "Register `SAEGPR2` writer"]
pub type W = crate::W<Saegpr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "General-purpose register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`saegpr2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saegpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Saegpr2Spec;
impl crate::RegisterSpec for Saegpr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saegpr2::R`](R) reader structure"]
impl crate::Readable for Saegpr2Spec {}
#[doc = "`write(|w| ..)` method takes [`saegpr2::W`](W) writer structure"]
impl crate::Writable for Saegpr2Spec {
    type Safety = crate::Unsafe;
}

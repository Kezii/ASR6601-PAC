#[doc = "Register `SAEGPR1` reader"]
pub type R = crate::R<Saegpr1Spec>;
#[doc = "Register `SAEGPR1` writer"]
pub type W = crate::W<Saegpr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "General-purpose register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`saegpr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saegpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Saegpr1Spec;
impl crate::RegisterSpec for Saegpr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saegpr1::R`](R) reader structure"]
impl crate::Readable for Saegpr1Spec {}
#[doc = "`write(|w| ..)` method takes [`saegpr1::W`](W) writer structure"]
impl crate::Writable for Saegpr1Spec {
    type Safety = crate::Unsafe;
}

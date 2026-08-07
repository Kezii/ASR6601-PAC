#[doc = "Register `CR10` reader"]
pub type R = crate::R<Cr10Spec>;
#[doc = "Register `CR10` writer"]
pub type W = crate::W<Cr10Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "control register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`cr10::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr10Spec;
impl crate::RegisterSpec for Cr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr10::R`](R) reader structure"]
impl crate::Readable for Cr10Spec {}
#[doc = "`write(|w| ..)` method takes [`cr10::W`](W) writer structure"]
impl crate::Writable for Cr10Spec {
    type Safety = crate::Unsafe;
}

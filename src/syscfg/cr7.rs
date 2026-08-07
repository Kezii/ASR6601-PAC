#[doc = "Register `CR7` reader"]
pub type R = crate::R<Cr7Spec>;
#[doc = "Register `CR7` writer"]
pub type W = crate::W<Cr7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "control register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`cr7::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr7Spec;
impl crate::RegisterSpec for Cr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr7::R`](R) reader structure"]
impl crate::Readable for Cr7Spec {}
#[doc = "`write(|w| ..)` method takes [`cr7::W`](W) writer structure"]
impl crate::Writable for Cr7Spec {
    type Safety = crate::Unsafe;
}

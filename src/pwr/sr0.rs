#[doc = "Register `SR0` reader"]
pub type R = crate::R<Sr0Spec>;
#[doc = "Register `SR0` writer"]
pub type W = crate::W<Sr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "status register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sr0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr0Spec;
impl crate::RegisterSpec for Sr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr0::R`](R) reader structure"]
impl crate::Readable for Sr0Spec {}
#[doc = "`write(|w| ..)` method takes [`sr0::W`](W) writer structure"]
impl crate::Writable for Sr0Spec {
    type Safety = crate::Unsafe;
}

#[doc = "Register `SR1` reader"]
pub type R = crate::R<Sr1Spec>;
#[doc = "Register `SR1` writer"]
pub type W = crate::W<Sr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr1Spec;
impl crate::RegisterSpec for Sr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for Sr1Spec {}
#[doc = "`write(|w| ..)` method takes [`sr1::W`](W) writer structure"]
impl crate::Writable for Sr1Spec {
    type Safety = crate::Unsafe;
}

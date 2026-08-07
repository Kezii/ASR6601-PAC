#[doc = "Register `FILTER1` reader"]
pub type R = crate::R<Filter1Spec>;
#[doc = "Register `FILTER1` writer"]
pub type W = crate::W<Filter1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "filter1\n\nYou can [`read`](crate::Reg::read) this register and get [`filter1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Filter1Spec;
impl crate::RegisterSpec for Filter1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter1::R`](R) reader structure"]
impl crate::Readable for Filter1Spec {}
#[doc = "`write(|w| ..)` method takes [`filter1::W`](W) writer structure"]
impl crate::Writable for Filter1Spec {
    type Safety = crate::Unsafe;
}

#[doc = "Register `FILTER0` reader"]
pub type R = crate::R<Filter0Spec>;
#[doc = "Register `FILTER0` writer"]
pub type W = crate::W<Filter0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "filter0\n\nYou can [`read`](crate::Reg::read) this register and get [`filter0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Filter0Spec;
impl crate::RegisterSpec for Filter0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter0::R`](R) reader structure"]
impl crate::Readable for Filter0Spec {}
#[doc = "`write(|w| ..)` method takes [`filter0::W`](W) writer structure"]
impl crate::Writable for Filter0Spec {
    type Safety = crate::Unsafe;
}

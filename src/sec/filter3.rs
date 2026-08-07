#[doc = "Register `FILTER3` reader"]
pub type R = crate::R<Filter3Spec>;
#[doc = "Register `FILTER3` writer"]
pub type W = crate::W<Filter3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "filter3\n\nYou can [`read`](crate::Reg::read) this register and get [`filter3::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Filter3Spec;
impl crate::RegisterSpec for Filter3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter3::R`](R) reader structure"]
impl crate::Readable for Filter3Spec {}
#[doc = "`write(|w| ..)` method takes [`filter3::W`](W) writer structure"]
impl crate::Writable for Filter3Spec {
    type Safety = crate::Unsafe;
}

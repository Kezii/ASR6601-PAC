#[doc = "Register `SGL_REQ_SRC` reader"]
pub type R = crate::R<SglReqSrcSpec>;
#[doc = "Register `SGL_REQ_SRC` writer"]
pub type W = crate::W<SglReqSrcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sgl_req_src::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgl_req_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SglReqSrcSpec;
impl crate::RegisterSpec for SglReqSrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgl_req_src::R`](R) reader structure"]
impl crate::Readable for SglReqSrcSpec {}
#[doc = "`write(|w| ..)` method takes [`sgl_req_src::W`](W) writer structure"]
impl crate::Writable for SglReqSrcSpec {
    type Safety = crate::Unsafe;
}

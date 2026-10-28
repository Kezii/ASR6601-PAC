#[doc = "Register `SGL_REQ_SRC` reader"]
pub type R = crate::R<SglReqSrcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "DMA single source request status\n\nYou can [`read`](crate::Reg::read) this register and get [`sgl_req_src::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SglReqSrcSpec;
impl crate::RegisterSpec for SglReqSrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgl_req_src::R`](R) reader structure"]
impl crate::Readable for SglReqSrcSpec {}

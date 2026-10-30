#[doc = "Register `REQ_SRC` reader"]
pub type R = crate::R<ReqSrcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "DMA source request status\n\nYou can [`read`](crate::Reg::read) this register and get [`req_src::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReqSrcSpec;
impl crate::RegisterSpec for ReqSrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`req_src::R`](R) reader structure"]
impl crate::Readable for ReqSrcSpec {}

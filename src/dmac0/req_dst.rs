#[doc = "Register `REQ_DST` reader"]
pub type R = crate::R<ReqDstSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "DMA destination request status\n\nYou can [`read`](crate::Reg::read) this register and get [`req_dst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReqDstSpec;
impl crate::RegisterSpec for ReqDstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`req_dst::R`](R) reader structure"]
impl crate::Readable for ReqDstSpec {}

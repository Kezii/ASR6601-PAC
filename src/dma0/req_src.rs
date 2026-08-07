#[doc = "Register `REQ_SRC` reader"]
pub type R = crate::R<ReqSrcSpec>;
#[doc = "Register `REQ_SRC` writer"]
pub type W = crate::W<ReqSrcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`req_src::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`req_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReqSrcSpec;
impl crate::RegisterSpec for ReqSrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`req_src::R`](R) reader structure"]
impl crate::Readable for ReqSrcSpec {}
#[doc = "`write(|w| ..)` method takes [`req_src::W`](W) writer structure"]
impl crate::Writable for ReqSrcSpec {
    type Safety = crate::Unsafe;
}

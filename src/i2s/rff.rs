#[doc = "Register `RFF` writer"]
pub type W = crate::W<RffSpec>;
impl core::fmt::Debug for crate::generic::Reg<RffSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "receiver FIFO flush register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rff::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RffSpec;
impl crate::RegisterSpec for RffSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rff::W`](W) writer structure"]
impl crate::Writable for RffSpec {
    type Safety = crate::Unsafe;
}

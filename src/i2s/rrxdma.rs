#[doc = "Register `RRXDMA` writer"]
pub type W = crate::W<RrxdmaSpec>;
impl core::fmt::Debug for crate::generic::Reg<RrxdmaSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "reset receiver block dma register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrxdma::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrxdmaSpec;
impl crate::RegisterSpec for RrxdmaSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rrxdma::W`](W) writer structure"]
impl crate::Writable for RrxdmaSpec {
    type Safety = crate::Unsafe;
}

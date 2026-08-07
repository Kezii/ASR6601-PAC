#[doc = "Register `RTXDMA` writer"]
pub type W = crate::W<RtxdmaSpec>;
impl core::fmt::Debug for crate::generic::Reg<RtxdmaSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "reset transmitter block dma register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtxdma::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtxdmaSpec;
impl crate::RegisterSpec for RtxdmaSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rtxdma::W`](W) writer structure"]
impl crate::Writable for RtxdmaSpec {
    type Safety = crate::Unsafe;
}

#[doc = "Register `RXFFR` writer"]
pub type W = crate::W<RxffrSpec>;
impl core::fmt::Debug for crate::generic::Reg<RxffrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "receiver block FIFO reset register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxffr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxffrSpec;
impl crate::RegisterSpec for RxffrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rxffr::W`](W) writer structure"]
impl crate::Writable for RxffrSpec {
    type Safety = crate::Unsafe;
}

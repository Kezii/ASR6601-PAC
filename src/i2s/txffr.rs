#[doc = "Register `TXFFR` writer"]
pub type W = crate::W<TxffrSpec>;
impl core::fmt::Debug for crate::generic::Reg<TxffrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "transmitter block FIFO reset register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txffr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxffrSpec;
impl crate::RegisterSpec for TxffrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txffr::W`](W) writer structure"]
impl crate::Writable for TxffrSpec {
    type Safety = crate::Unsafe;
}

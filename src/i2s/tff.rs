#[doc = "Register `TFF` writer"]
pub type W = crate::W<TffSpec>;
impl core::fmt::Debug for crate::generic::Reg<TffSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "transmitter FIFO flush register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tff::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TffSpec;
impl crate::RegisterSpec for TffSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tff::W`](W) writer structure"]
impl crate::Writable for TffSpec {
    type Safety = crate::Unsafe;
}

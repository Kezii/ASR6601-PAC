#[doc = "Register `PROTECT_SEQ` writer"]
pub type W = crate::W<ProtectSeqSpec>;
impl core::fmt::Debug for crate::generic::Reg<ProtectSeqSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "protect sequence register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`protect_seq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProtectSeqSpec;
impl crate::RegisterSpec for ProtectSeqSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`protect_seq::W`](W) writer structure"]
impl crate::Writable for ProtectSeqSpec {
    type Safety = crate::Unsafe;
}

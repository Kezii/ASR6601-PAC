#[doc = "Register `BSR` writer"]
pub type W = crate::W<BsrSpec>;
impl core::fmt::Debug for crate::generic::Reg<BsrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "bit set register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsrSpec;
impl crate::RegisterSpec for BsrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bsr::W`](W) writer structure"]
impl crate::Writable for BsrSpec {
    type Safety = crate::Unsafe;
}

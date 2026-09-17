#[doc = "Register `ITOP` writer"]
pub type W = crate::W<ItopSpec>;
impl core::fmt::Debug for crate::generic::Reg<ItopSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "integration test output set register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ItopSpec;
impl crate::RegisterSpec for ItopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`itop::W`](W) writer structure"]
impl crate::Writable for ItopSpec {
    type Safety = crate::Unsafe;
}

#[doc = "Register `RST_CYCL` reader"]
pub type R = crate::R<RstCyclSpec>;
#[doc = "Register `RST_CYCL` writer"]
pub type W = crate::W<RstCyclSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "reset cycle register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_cycl::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_cycl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstCyclSpec;
impl crate::RegisterSpec for RstCyclSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_cycl::R`](R) reader structure"]
impl crate::Readable for RstCyclSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_cycl::W`](W) writer structure"]
impl crate::Writable for RstCyclSpec {
    type Safety = crate::Unsafe;
}

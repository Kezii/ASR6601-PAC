#[doc = "Register `ITCR` reader"]
pub type R = crate::R<ItcrSpec>;
#[doc = "Register `ITCR` writer"]
pub type W = crate::W<ItcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "integration test control register\n\nYou can [`read`](crate::Reg::read) this register and get [`itcr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ItcrSpec;
impl crate::RegisterSpec for ItcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itcr::R`](R) reader structure"]
impl crate::Readable for ItcrSpec {}
#[doc = "`write(|w| ..)` method takes [`itcr::W`](W) writer structure"]
impl crate::Writable for ItcrSpec {
    type Safety = crate::Unsafe;
}

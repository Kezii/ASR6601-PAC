#[doc = "Register `OER` reader"]
pub type R = crate::R<OerSpec>;
#[doc = "Register `OER` writer"]
pub type W = crate::W<OerSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "output enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`oer::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OerSpec;
impl crate::RegisterSpec for OerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oer::R`](R) reader structure"]
impl crate::Readable for OerSpec {}
#[doc = "`write(|w| ..)` method takes [`oer::W`](W) writer structure"]
impl crate::Writable for OerSpec {
    type Safety = crate::Unsafe;
}

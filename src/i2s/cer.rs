#[doc = "Register `CER` reader"]
pub type R = crate::R<CerSpec>;
#[doc = "Register `CER` writer"]
pub type W = crate::W<CerSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cer::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CerSpec;
impl crate::RegisterSpec for CerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cer::R`](R) reader structure"]
impl crate::Readable for CerSpec {}
#[doc = "`write(|w| ..)` method takes [`cer::W`](W) writer structure"]
impl crate::Writable for CerSpec {
    type Safety = crate::Unsafe;
}

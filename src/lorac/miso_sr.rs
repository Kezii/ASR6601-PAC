#[doc = "Register `MISO_SR` reader"]
pub type R = crate::R<MisoSrSpec>;
#[doc = "Register `MISO_SR` writer"]
pub type W = crate::W<MisoSrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "miso control register\n\nYou can [`read`](crate::Reg::read) this register and get [`miso_sr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miso_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisoSrSpec;
impl crate::RegisterSpec for MisoSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miso_sr::R`](R) reader structure"]
impl crate::Readable for MisoSrSpec {}
#[doc = "`write(|w| ..)` method takes [`miso_sr::W`](W) writer structure"]
impl crate::Writable for MisoSrSpec {
    type Safety = crate::Unsafe;
}

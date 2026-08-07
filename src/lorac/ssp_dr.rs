#[doc = "Register `SSP_DR` reader"]
pub type R = crate::R<SspDrSpec>;
#[doc = "Register `SSP_DR` writer"]
pub type W = crate::W<SspDrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ssp data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_dr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspDrSpec;
impl crate::RegisterSpec for SspDrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssp_dr::R`](R) reader structure"]
impl crate::Readable for SspDrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssp_dr::W`](W) writer structure"]
impl crate::Writable for SspDrSpec {
    type Safety = crate::Unsafe;
}

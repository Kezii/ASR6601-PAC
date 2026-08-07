#[doc = "Register `SSP_MIS` reader"]
pub type R = crate::R<SspMisSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ssp masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspMisSpec;
impl crate::RegisterSpec for SspMisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssp_mis::R`](R) reader structure"]
impl crate::Readable for SspMisSpec {}

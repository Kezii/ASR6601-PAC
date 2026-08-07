#[doc = "Register `SSP_RIS` reader"]
pub type R = crate::R<SspRisSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ssp raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspRisSpec;
impl crate::RegisterSpec for SspRisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssp_ris::R`](R) reader structure"]
impl crate::Readable for SspRisSpec {}

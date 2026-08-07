#[doc = "Register `SSP_SR` reader"]
pub type R = crate::R<SspSrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ssp status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspSrSpec;
impl crate::RegisterSpec for SspSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssp_sr::R`](R) reader structure"]
impl crate::Readable for SspSrSpec {}

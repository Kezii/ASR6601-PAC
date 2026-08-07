#[doc = "Register `OPTION_EO_BYTES` reader"]
pub type R = crate::R<OptionEoBytesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "option exe-only bytes register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_eo_bytes::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptionEoBytesSpec;
impl crate::RegisterSpec for OptionEoBytesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`option_eo_bytes::R`](R) reader structure"]
impl crate::Readable for OptionEoBytesSpec {}

#[doc = "Register `OPTION_CSR_BYTES` reader"]
pub type R = crate::R<OptionCsrBytesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "option control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_csr_bytes::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptionCsrBytesSpec;
impl crate::RegisterSpec for OptionCsrBytesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`option_csr_bytes::R`](R) reader structure"]
impl crate::Readable for OptionCsrBytesSpec {}

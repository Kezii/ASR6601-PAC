#[doc = "Register `OPTION_SEC_BYTES0` reader"]
pub type R = crate::R<OptionSecBytes0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "option secure byte 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_sec_bytes0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptionSecBytes0Spec;
impl crate::RegisterSpec for OptionSecBytes0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`option_sec_bytes0::R`](R) reader structure"]
impl crate::Readable for OptionSecBytes0Spec {}

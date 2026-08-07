#[doc = "Register `OPTION_WP_BYTES` reader"]
pub type R = crate::R<OptionWpBytesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "option write-protect bytes register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_wp_bytes::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptionWpBytesSpec;
impl crate::RegisterSpec for OptionWpBytesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`option_wp_bytes::R`](R) reader structure"]
impl crate::Readable for OptionWpBytesSpec {}

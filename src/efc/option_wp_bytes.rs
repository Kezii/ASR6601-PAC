#[doc = "Register `OPTION_WP_BYTES` reader"]
pub type R = crate::R<OptionWpBytesSpec>;
#[doc = "Field `WRPROTECT_START` reader - Write-protected area start offset"]
pub type WrprotectStartR = crate::FieldReader;
#[doc = "Field `WRPROTECT_END` reader - Write-protected area end offset"]
pub type WrprotectEndR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Write-protected area start offset"]
    #[inline(always)]
    pub fn wrprotect_start(&self) -> WrprotectStartR {
        WrprotectStartR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Write-protected area end offset"]
    #[inline(always)]
    pub fn wrprotect_end(&self) -> WrprotectEndR {
        WrprotectEndR::new(((self.bits >> 6) & 0x3f) as u8)
    }
}
#[doc = "option write-protect bytes register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_wp_bytes::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptionWpBytesSpec;
impl crate::RegisterSpec for OptionWpBytesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`option_wp_bytes::R`](R) reader structure"]
impl crate::Readable for OptionWpBytesSpec {}

#[doc = "Register `OPTION_SEC_BYTES0` reader"]
pub type R = crate::R<OptionSecBytes0Spec>;
#[doc = "Field `FLASH_SECURE_START` reader - Flash secure area start"]
pub type FlashSecureStartR = crate::FieldReader;
#[doc = "Field `FLASH_SECURE_END` reader - Flash secure area end"]
pub type FlashSecureEndR = crate::FieldReader;
#[doc = "Field `SYSRAM_SECURE_START` reader - Sysram secure area start"]
pub type SysramSecureStartR = crate::FieldReader;
#[doc = "Field `SYSRAM_SECURE_END` reader - Sysram secure area end"]
pub type SysramSecureEndR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Flash secure area start"]
    #[inline(always)]
    pub fn flash_secure_start(&self) -> FlashSecureStartR {
        FlashSecureStartR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Flash secure area end"]
    #[inline(always)]
    pub fn flash_secure_end(&self) -> FlashSecureEndR {
        FlashSecureEndR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Sysram secure area start"]
    #[inline(always)]
    pub fn sysram_secure_start(&self) -> SysramSecureStartR {
        SysramSecureStartR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Sysram secure area end"]
    #[inline(always)]
    pub fn sysram_secure_end(&self) -> SysramSecureEndR {
        SysramSecureEndR::new(((self.bits >> 18) & 0x3f) as u8)
    }
}
#[doc = "option secure byte 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_sec_bytes0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptionSecBytes0Spec;
impl crate::RegisterSpec for OptionSecBytes0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`option_sec_bytes0::R`](R) reader structure"]
impl crate::Readable for OptionSecBytes0Spec {}

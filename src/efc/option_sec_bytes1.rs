#[doc = "Register `OPTION_SEC_BYTES1` reader"]
pub type R = crate::R<OptionSecBytes1Spec>;
#[doc = "Field `RETRAM_SECURE_START` reader - Retram secure area start"]
pub type RetramSecureStartR = crate::FieldReader;
#[doc = "Field `RETRAM_SECURE_END` reader - Retram secure area end"]
pub type RetramSecureEndR = crate::FieldReader;
#[doc = "Field `FLASH_HIDE_START` reader - Flash hide area start"]
pub type FlashHideStartR = crate::FieldReader;
#[doc = "Field `FLASH_HIDE_ENABLE` reader - Flash hide area enable control"]
pub type FlashHideEnableR = crate::BitReader;
#[doc = "Field `SYSRAM_HIDE_START` reader - Sysram hide area start"]
pub type SysramHideStartR = crate::FieldReader;
#[doc = "Field `SYSRAM_HIDE_ENABLE` reader - Sysram hide area enable control"]
pub type SysramHideEnableR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Retram secure area start"]
    #[inline(always)]
    pub fn retram_secure_start(&self) -> RetramSecureStartR {
        RetramSecureStartR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Retram secure area end"]
    #[inline(always)]
    pub fn retram_secure_end(&self) -> RetramSecureEndR {
        RetramSecureEndR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:15 - Flash hide area start"]
    #[inline(always)]
    pub fn flash_hide_start(&self) -> FlashHideStartR {
        FlashHideStartR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Flash hide area enable control"]
    #[inline(always)]
    pub fn flash_hide_enable(&self) -> FlashHideEnableR {
        FlashHideEnableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Sysram hide area start"]
    #[inline(always)]
    pub fn sysram_hide_start(&self) -> SysramHideStartR {
        SysramHideStartR::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Sysram hide area enable control"]
    #[inline(always)]
    pub fn sysram_hide_enable(&self) -> SysramHideEnableR {
        SysramHideEnableR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "option secure byte 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_sec_bytes1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptionSecBytes1Spec;
impl crate::RegisterSpec for OptionSecBytes1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`option_sec_bytes1::R`](R) reader structure"]
impl crate::Readable for OptionSecBytes1Spec {}

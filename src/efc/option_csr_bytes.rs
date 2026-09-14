#[doc = "Register `OPTION_CSR_BYTES` reader"]
pub type R = crate::R<OptionCsrBytesSpec>;
#[doc = "Field `FLASH_BOOT0` reader - Flash boot 0"]
pub type FlashBoot0R = crate::BitReader;
#[doc = "Field `USE_FLASH_BOOT0` reader - Use flash boot 0"]
pub type UseFlashBoot0R = crate::BitReader;
#[doc = "Field `FLASH_BOOT1` reader - Flash boot 1"]
pub type FlashBoot1R = crate::BitReader;
#[doc = "Field `SYS_SRAM_RST` reader - Clear system sram during system startup"]
pub type SysSramRstR = crate::BitReader;
#[doc = "Field `SECURE_AREA_EN` reader - Flash secure area status flag"]
pub type SecureAreaEnR = crate::BitReader;
#[doc = "Debug level setting"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DebugLevel {
    #[doc = "0: Level 0"]
    Level0 = 0,
    #[doc = "1: Level 1"]
    Level1 = 1,
    #[doc = "2: Level 2"]
    Level2 = 2,
}
impl From<DebugLevel> for u8 {
    #[inline(always)]
    fn from(variant: DebugLevel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DebugLevel {
    type Ux = u8;
}
impl crate::IsEnum for DebugLevel {}
#[doc = "Field `DEBUG_LEVEL` reader - Debug level setting"]
pub type DebugLevelR = crate::FieldReader<DebugLevel>;
impl DebugLevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DebugLevel> {
        match self.bits {
            0 => Some(DebugLevel::Level0),
            1 => Some(DebugLevel::Level1),
            2 => Some(DebugLevel::Level2),
            _ => None,
        }
    }
    #[doc = "Level 0"]
    #[inline(always)]
    pub fn is_level_0(&self) -> bool {
        *self == DebugLevel::Level0
    }
    #[doc = "Level 1"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == DebugLevel::Level1
    }
    #[doc = "Level 2"]
    #[inline(always)]
    pub fn is_level_2(&self) -> bool {
        *self == DebugLevel::Level2
    }
}
impl R {
    #[doc = "Bit 0 - Flash boot 0"]
    #[inline(always)]
    pub fn flash_boot0(&self) -> FlashBoot0R {
        FlashBoot0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Use flash boot 0"]
    #[inline(always)]
    pub fn use_flash_boot0(&self) -> UseFlashBoot0R {
        UseFlashBoot0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash boot 1"]
    #[inline(always)]
    pub fn flash_boot1(&self) -> FlashBoot1R {
        FlashBoot1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear system sram during system startup"]
    #[inline(always)]
    pub fn sys_sram_rst(&self) -> SysSramRstR {
        SysSramRstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash secure area status flag"]
    #[inline(always)]
    pub fn secure_area_en(&self) -> SecureAreaEnR {
        SecureAreaEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Debug level setting"]
    #[inline(always)]
    pub fn debug_level(&self) -> DebugLevelR {
        DebugLevelR::new(((self.bits >> 5) & 3) as u8)
    }
}
#[doc = "option control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_csr_bytes::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptionCsrBytesSpec;
impl crate::RegisterSpec for OptionCsrBytesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`option_csr_bytes::R`](R) reader structure"]
impl crate::Readable for OptionCsrBytesSpec {}

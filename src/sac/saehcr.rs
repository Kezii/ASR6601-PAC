#[doc = "Register `SAEHCR` reader"]
pub type R = crate::R<SaehcrSpec>;
#[doc = "Register `SAEHCR` writer"]
pub type W = crate::W<SaehcrSpec>;
#[doc = "Field `RAM_CLEAR` reader - Ram clear"]
pub type RamClearR = crate::BitReader;
#[doc = "Field `RAM_CLEAR` writer - Ram clear"]
pub type RamClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_UPDATE_ENABLE` reader - Key update enable"]
pub type KeyUpdateEnableR = crate::BitReader;
#[doc = "Field `KEY_UPDATE_ENABLE` writer - Key update enable"]
pub type KeyUpdateEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCRAMBLER_CLOCK_ENABLE` reader - Scrambler clock enable"]
pub type ScramblerClockEnableR = crate::BitReader;
#[doc = "Field `SCRAMBLER_CLOCK_ENABLE` writer - Scrambler clock enable"]
pub type ScramblerClockEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAE_CLOCK_ENABLE` reader - Sae clock enable"]
pub type SaeClockEnableR = crate::BitReader;
#[doc = "Field `SAE_CLOCK_ENABLE` writer - Sae clock enable"]
pub type SaeClockEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCRAMBLER_LEVEL` reader - Scrambler level"]
pub type ScramblerLevelR = crate::FieldReader;
#[doc = "Field `SCRAMBLER_LEVEL` writer - Scrambler level"]
pub type ScramblerLevelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Ram clear"]
    #[inline(always)]
    pub fn ram_clear(&self) -> RamClearR {
        RamClearR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key update enable"]
    #[inline(always)]
    pub fn key_update_enable(&self) -> KeyUpdateEnableR {
        KeyUpdateEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scrambler clock enable"]
    #[inline(always)]
    pub fn scrambler_clock_enable(&self) -> ScramblerClockEnableR {
        ScramblerClockEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sae clock enable"]
    #[inline(always)]
    pub fn sae_clock_enable(&self) -> SaeClockEnableR {
        SaeClockEnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Scrambler level"]
    #[inline(always)]
    pub fn scrambler_level(&self) -> ScramblerLevelR {
        ScramblerLevelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Ram clear"]
    #[inline(always)]
    pub fn ram_clear(&mut self) -> RamClearW<'_, SaehcrSpec> {
        RamClearW::new(self, 0)
    }
    #[doc = "Bit 1 - Key update enable"]
    #[inline(always)]
    pub fn key_update_enable(&mut self) -> KeyUpdateEnableW<'_, SaehcrSpec> {
        KeyUpdateEnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Scrambler clock enable"]
    #[inline(always)]
    pub fn scrambler_clock_enable(&mut self) -> ScramblerClockEnableW<'_, SaehcrSpec> {
        ScramblerClockEnableW::new(self, 2)
    }
    #[doc = "Bit 3 - Sae clock enable"]
    #[inline(always)]
    pub fn sae_clock_enable(&mut self) -> SaeClockEnableW<'_, SaehcrSpec> {
        SaeClockEnableW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Scrambler level"]
    #[inline(always)]
    pub fn scrambler_level(&mut self) -> ScramblerLevelW<'_, SaehcrSpec> {
        ScramblerLevelW::new(self, 4)
    }
}
#[doc = "Security control register\n\nYou can [`read`](crate::Reg::read) this register and get [`saehcr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saehcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaehcrSpec;
impl crate::RegisterSpec for SaehcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saehcr::R`](R) reader structure"]
impl crate::Readable for SaehcrSpec {}
#[doc = "`write(|w| ..)` method takes [`saehcr::W`](W) writer structure"]
impl crate::Writable for SaehcrSpec {
    type Safety = crate::Unsafe;
}

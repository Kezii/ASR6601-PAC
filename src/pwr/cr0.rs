#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `LOW_POWER_MODE` reader - Low-power mode selection"]
pub type LowPowerModeR = crate::FieldReader;
#[doc = "Field `LOW_POWER_MODE` writer - Low-power mode selection"]
pub type LowPowerModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WFE` reader - Wait for event selection, set for WFE, clear for WFI"]
pub type WfeR = crate::BitReader;
#[doc = "Field `WFE` writer - Wait for event selection, set for WFE, clear for WFI"]
pub type WfeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Low-power mode selection"]
    #[inline(always)]
    pub fn low_power_mode(&self) -> LowPowerModeR {
        LowPowerModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 5 - Wait for event selection, set for WFE, clear for WFI"]
    #[inline(always)]
    pub fn wfe(&self) -> WfeR {
        WfeR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low-power mode selection"]
    #[inline(always)]
    pub fn low_power_mode(&mut self) -> LowPowerModeW<'_, Cr0Spec> {
        LowPowerModeW::new(self, 0)
    }
    #[doc = "Bit 5 - Wait for event selection, set for WFE, clear for WFI"]
    #[inline(always)]
    pub fn wfe(&mut self) -> WfeW<'_, Cr0Spec> {
        WfeW::new(self, 5)
    }
}
#[doc = "control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}

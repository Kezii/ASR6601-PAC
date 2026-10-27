#[doc = "Register `ANALOG_12` reader"]
pub type R = crate::R<Analog12Spec>;
#[doc = "Register `ANALOG_12` writer"]
pub type W = crate::W<Analog12Spec>;
#[doc = "Field `ADC_INTERNAL_REF` reader - Select internal ADC reference voltage (vendor tremo_adc.c)"]
pub type AdcInternalRefR = crate::BitReader;
#[doc = "Field `ADC_INTERNAL_REF` writer - Select internal ADC reference voltage (vendor tremo_adc.c)"]
pub type AdcInternalRefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_POWER_DOWN` reader - DAC power-down control (1 = powered down, vendor tremo_dac.c)"]
pub type DacPowerDownR = crate::BitReader;
#[doc = "Field `DAC_POWER_DOWN` writer - DAC power-down control (1 = powered down, vendor tremo_dac.c)"]
pub type DacPowerDownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - Select internal ADC reference voltage (vendor tremo_adc.c)"]
    #[inline(always)]
    pub fn adc_internal_ref(&self) -> AdcInternalRefR {
        AdcInternalRefR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC power-down control (1 = powered down, vendor tremo_dac.c)"]
    #[inline(always)]
    pub fn dac_power_down(&self) -> DacPowerDownR {
        DacPowerDownR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Select internal ADC reference voltage (vendor tremo_adc.c)"]
    #[inline(always)]
    pub fn adc_internal_ref(&mut self) -> AdcInternalRefW<'_, Analog12Spec> {
        AdcInternalRefW::new(self, 6)
    }
    #[doc = "Bit 8 - DAC power-down control (1 = powered down, vendor tremo_dac.c)"]
    #[inline(always)]
    pub fn dac_power_down(&mut self) -> DacPowerDownW<'_, Analog12Spec> {
        DacPowerDownW::new(self, 8)
    }
}
#[doc = "Analog register 0x12 (ADC reference/DAC analog control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_12::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Analog12Spec;
impl crate::RegisterSpec for Analog12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_12::R`](R) reader structure"]
impl crate::Readable for Analog12Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_12::W`](W) writer structure"]
impl crate::Writable for Analog12Spec {
    type Safety = crate::Unsafe;
}

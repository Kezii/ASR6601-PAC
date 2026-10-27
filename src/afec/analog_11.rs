#[doc = "Register `ANALOG_11` reader"]
pub type R = crate::R<Analog11Spec>;
#[doc = "Register `ANALOG_11` writer"]
pub type W = crate::W<Analog11Spec>;
#[doc = "Field `ADC_INIT` reader - ADC initialization bits cleared by vendor adc_init()"]
pub type AdcInitR = crate::FieldReader;
#[doc = "Field `ADC_INIT` writer - ADC initialization bits cleared by vendor adc_init()"]
pub type AdcInitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "DAC power-down control (vendor tremo_dac.c clears both bits to enable)"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DacPowerDown {
    #[doc = "0: DAC enabled"]
    Enabled = 0,
    #[doc = "3: DAC powered down"]
    PoweredDown = 3,
}
impl From<DacPowerDown> for u8 {
    #[inline(always)]
    fn from(variant: DacPowerDown) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DacPowerDown {
    type Ux = u8;
}
impl crate::IsEnum for DacPowerDown {}
#[doc = "Field `DAC_POWER_DOWN` reader - DAC power-down control (vendor tremo_dac.c clears both bits to enable)"]
pub type DacPowerDownR = crate::FieldReader<DacPowerDown>;
impl DacPowerDownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DacPowerDown> {
        match self.bits {
            0 => Some(DacPowerDown::Enabled),
            3 => Some(DacPowerDown::PoweredDown),
            _ => None,
        }
    }
    #[doc = "DAC enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DacPowerDown::Enabled
    }
    #[doc = "DAC powered down"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == DacPowerDown::PoweredDown
    }
}
#[doc = "Field `DAC_POWER_DOWN` writer - DAC power-down control (vendor tremo_dac.c clears both bits to enable)"]
pub type DacPowerDownW<'a, REG> = crate::FieldWriter<'a, REG, 2, DacPowerDown>;
impl<'a, REG> DacPowerDownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DAC enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DacPowerDown::Enabled)
    }
    #[doc = "DAC powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut crate::W<REG> {
        self.variant(DacPowerDown::PoweredDown)
    }
}
impl R {
    #[doc = "Bits 6:9 - ADC initialization bits cleared by vendor adc_init()"]
    #[inline(always)]
    pub fn adc_init(&self) -> AdcInitR {
        AdcInitR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - DAC power-down control (vendor tremo_dac.c clears both bits to enable)"]
    #[inline(always)]
    pub fn dac_power_down(&self) -> DacPowerDownR {
        DacPowerDownR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:9 - ADC initialization bits cleared by vendor adc_init()"]
    #[inline(always)]
    pub fn adc_init(&mut self) -> AdcInitW<'_, Analog11Spec> {
        AdcInitW::new(self, 6)
    }
    #[doc = "Bits 24:25 - DAC power-down control (vendor tremo_dac.c clears both bits to enable)"]
    #[inline(always)]
    pub fn dac_power_down(&mut self) -> DacPowerDownW<'_, Analog11Spec> {
        DacPowerDownW::new(self, 24)
    }
}
#[doc = "Analog register 0x11 (ADC/DAC analog control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_11::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Analog11Spec;
impl crate::RegisterSpec for Analog11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_11::R`](R) reader structure"]
impl crate::Readable for Analog11Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_11::W`](W) writer structure"]
impl crate::Writable for Analog11Spec {
    type Safety = crate::Unsafe;
}

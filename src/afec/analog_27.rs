#[doc = "Register `ANALOG_27` reader"]
pub type R = crate::R<Analog27Spec>;
#[doc = "Register `ANALOG_27` writer"]
pub type W = crate::W<Analog27Spec>;
#[doc = "DAC analog output enable (vendor tremo_dac.c sets both bits to enable)"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DacOutputEnable {
    #[doc = "0: DAC analog output disabled"]
    Disabled = 0,
    #[doc = "3: DAC analog output enabled"]
    Enabled = 3,
}
impl From<DacOutputEnable> for u8 {
    #[inline(always)]
    fn from(variant: DacOutputEnable) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DacOutputEnable {
    type Ux = u8;
}
impl crate::IsEnum for DacOutputEnable {}
#[doc = "Field `DAC_OUTPUT_ENABLE` reader - DAC analog output enable (vendor tremo_dac.c sets both bits to enable)"]
pub type DacOutputEnableR = crate::FieldReader<DacOutputEnable>;
impl DacOutputEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DacOutputEnable> {
        match self.bits {
            0 => Some(DacOutputEnable::Disabled),
            3 => Some(DacOutputEnable::Enabled),
            _ => None,
        }
    }
    #[doc = "DAC analog output disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DacOutputEnable::Disabled
    }
    #[doc = "DAC analog output enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DacOutputEnable::Enabled
    }
}
#[doc = "Field `DAC_OUTPUT_ENABLE` writer - DAC analog output enable (vendor tremo_dac.c sets both bits to enable)"]
pub type DacOutputEnableW<'a, REG> = crate::FieldWriter<'a, REG, 2, DacOutputEnable>;
impl<'a, REG> DacOutputEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DAC analog output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DacOutputEnable::Disabled)
    }
    #[doc = "DAC analog output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DacOutputEnable::Enabled)
    }
}
impl R {
    #[doc = "Bits 11:12 - DAC analog output enable (vendor tremo_dac.c sets both bits to enable)"]
    #[inline(always)]
    pub fn dac_output_enable(&self) -> DacOutputEnableR {
        DacOutputEnableR::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 11:12 - DAC analog output enable (vendor tremo_dac.c sets both bits to enable)"]
    #[inline(always)]
    pub fn dac_output_enable(&mut self) -> DacOutputEnableW<'_, Analog27Spec> {
        DacOutputEnableW::new(self, 11)
    }
}
#[doc = "Analog register 0x27 (DAC analog output control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_27::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Analog27Spec;
impl crate::RegisterSpec for Analog27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_27::R`](R) reader structure"]
impl crate::Readable for Analog27Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_27::W`](W) writer structure"]
impl crate::Writable for Analog27Spec {
    type Safety = crate::Unsafe;
}

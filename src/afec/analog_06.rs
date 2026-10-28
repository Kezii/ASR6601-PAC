#[doc = "Register `ANALOG_06` reader"]
pub type R = crate::R<Analog06Spec>;
#[doc = "Register `ANALOG_06` writer"]
pub type W = crate::W<Analog06Spec>;
#[doc = "Field `XO24M_ENABLE` reader - XO24M enable (1 = enabled, vendor tremo_rcc.c)"]
pub type Xo24mEnableR = crate::BitReader;
#[doc = "Field `XO24M_ENABLE` writer - XO24M enable (1 = enabled, vendor tremo_rcc.c)"]
pub type Xo24mEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XO24M_POWER_DOWN` reader - XO24M power-down control (1 = powered down, vendor tremo_rcc.c)"]
pub type Xo24mPowerDownR = crate::BitReader;
#[doc = "Field `XO24M_POWER_DOWN` writer - XO24M power-down control (1 = powered down, vendor tremo_rcc.c)"]
pub type Xo24mPowerDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCO48M_POWER_DOWN` reader - RCO48M power-down control (1 = powered down, vendor tremo_rcc.c)"]
pub type Rco48mPowerDownR = crate::BitReader;
#[doc = "Field `RCO48M_POWER_DOWN` writer - RCO48M power-down control (1 = powered down, vendor tremo_rcc.c)"]
pub type Rco48mPowerDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCO4M_POWER_DOWN` reader - RCO4M power-down control (1 = powered down, vendor tremo_rcc.c)"]
pub type Rco4mPowerDownR = crate::BitReader;
#[doc = "Field `RCO4M_POWER_DOWN` writer - RCO4M power-down control (1 = powered down, vendor tremo_rcc.c)"]
pub type Rco4mPowerDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Low-power-run control 1 (vendor tremo_pwr.c)"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LowPowerRunEnable1 {
    #[doc = "0: Low-power-run control 1 disabled"]
    Disabled = 0,
    #[doc = "3: Low-power-run control 1 enabled"]
    Enabled = 3,
}
impl From<LowPowerRunEnable1> for u8 {
    #[inline(always)]
    fn from(variant: LowPowerRunEnable1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LowPowerRunEnable1 {
    type Ux = u8;
}
impl crate::IsEnum for LowPowerRunEnable1 {}
#[doc = "Field `LOW_POWER_RUN_ENABLE_1` reader - Low-power-run control 1 (vendor tremo_pwr.c)"]
pub type LowPowerRunEnable1R = crate::FieldReader<LowPowerRunEnable1>;
impl LowPowerRunEnable1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LowPowerRunEnable1> {
        match self.bits {
            0 => Some(LowPowerRunEnable1::Disabled),
            3 => Some(LowPowerRunEnable1::Enabled),
            _ => None,
        }
    }
    #[doc = "Low-power-run control 1 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LowPowerRunEnable1::Disabled
    }
    #[doc = "Low-power-run control 1 enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LowPowerRunEnable1::Enabled
    }
}
#[doc = "Field `LOW_POWER_RUN_ENABLE_1` writer - Low-power-run control 1 (vendor tremo_pwr.c)"]
pub type LowPowerRunEnable1W<'a, REG> = crate::FieldWriter<'a, REG, 2, LowPowerRunEnable1>;
impl<'a, REG> LowPowerRunEnable1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low-power-run control 1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LowPowerRunEnable1::Disabled)
    }
    #[doc = "Low-power-run control 1 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LowPowerRunEnable1::Enabled)
    }
}
impl R {
    #[doc = "Bit 3 - XO24M enable (1 = enabled, vendor tremo_rcc.c)"]
    #[inline(always)]
    pub fn xo24m_enable(&self) -> Xo24mEnableR {
        Xo24mEnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - XO24M power-down control (1 = powered down, vendor tremo_rcc.c)"]
    #[inline(always)]
    pub fn xo24m_power_down(&self) -> Xo24mPowerDownR {
        Xo24mPowerDownR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RCO48M power-down control (1 = powered down, vendor tremo_rcc.c)"]
    #[inline(always)]
    pub fn rco48m_power_down(&self) -> Rco48mPowerDownR {
        Rco48mPowerDownR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RCO4M power-down control (1 = powered down, vendor tremo_rcc.c)"]
    #[inline(always)]
    pub fn rco4m_power_down(&self) -> Rco4mPowerDownR {
        Rco4mPowerDownR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Low-power-run control 1 (vendor tremo_pwr.c)"]
    #[inline(always)]
    pub fn low_power_run_enable_1(&self) -> LowPowerRunEnable1R {
        LowPowerRunEnable1R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - XO24M enable (1 = enabled, vendor tremo_rcc.c)"]
    #[inline(always)]
    pub fn xo24m_enable(&mut self) -> Xo24mEnableW<'_, Analog06Spec> {
        Xo24mEnableW::new(self, 3)
    }
    #[doc = "Bit 4 - XO24M power-down control (1 = powered down, vendor tremo_rcc.c)"]
    #[inline(always)]
    pub fn xo24m_power_down(&mut self) -> Xo24mPowerDownW<'_, Analog06Spec> {
        Xo24mPowerDownW::new(self, 4)
    }
    #[doc = "Bit 5 - RCO48M power-down control (1 = powered down, vendor tremo_rcc.c)"]
    #[inline(always)]
    pub fn rco48m_power_down(&mut self) -> Rco48mPowerDownW<'_, Analog06Spec> {
        Rco48mPowerDownW::new(self, 5)
    }
    #[doc = "Bit 6 - RCO4M power-down control (1 = powered down, vendor tremo_rcc.c)"]
    #[inline(always)]
    pub fn rco4m_power_down(&mut self) -> Rco4mPowerDownW<'_, Analog06Spec> {
        Rco4mPowerDownW::new(self, 6)
    }
    #[doc = "Bits 20:21 - Low-power-run control 1 (vendor tremo_pwr.c)"]
    #[inline(always)]
    pub fn low_power_run_enable_1(&mut self) -> LowPowerRunEnable1W<'_, Analog06Spec> {
        LowPowerRunEnable1W::new(self, 20)
    }
}
#[doc = "Analog register 0x06 (oscillator/low-power-run/LCD analog control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_06::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_06::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Analog06Spec;
impl crate::RegisterSpec for Analog06Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_06::R`](R) reader structure"]
impl crate::Readable for Analog06Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_06::W`](W) writer structure"]
impl crate::Writable for Analog06Spec {
    type Safety = crate::Unsafe;
}

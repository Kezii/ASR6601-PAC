#[doc = "Register `ANALOG_02` reader"]
pub type R = crate::R<Analog02Spec>;
#[doc = "Register `ANALOG_02` writer"]
pub type W = crate::W<Analog02Spec>;
#[doc = "XO32K power-down control (vendor tremo_rcc.c clears both bits to enable)"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xo32kPowerDown {
    #[doc = "0: XO32K enabled"]
    Enabled = 0,
    #[doc = "3: XO32K powered down"]
    PoweredDown = 3,
}
impl From<Xo32kPowerDown> for u8 {
    #[inline(always)]
    fn from(variant: Xo32kPowerDown) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xo32kPowerDown {
    type Ux = u8;
}
impl crate::IsEnum for Xo32kPowerDown {}
#[doc = "Field `XO32K_POWER_DOWN` reader - XO32K power-down control (vendor tremo_rcc.c clears both bits to enable)"]
pub type Xo32kPowerDownR = crate::FieldReader<Xo32kPowerDown>;
impl Xo32kPowerDownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Xo32kPowerDown> {
        match self.bits {
            0 => Some(Xo32kPowerDown::Enabled),
            3 => Some(Xo32kPowerDown::PoweredDown),
            _ => None,
        }
    }
    #[doc = "XO32K enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Xo32kPowerDown::Enabled
    }
    #[doc = "XO32K powered down"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == Xo32kPowerDown::PoweredDown
    }
}
#[doc = "Field `XO32K_POWER_DOWN` writer - XO32K power-down control (vendor tremo_rcc.c clears both bits to enable)"]
pub type Xo32kPowerDownW<'a, REG> = crate::FieldWriter<'a, REG, 2, Xo32kPowerDown>;
impl<'a, REG> Xo32kPowerDownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XO32K enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Xo32kPowerDown::Enabled)
    }
    #[doc = "XO32K powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut crate::W<REG> {
        self.variant(Xo32kPowerDown::PoweredDown)
    }
}
#[doc = "Field `RCO32K_POWER_DOWN` reader - RCO32K power-down control (1 = powered down, vendor tremo_rcc.c)"]
pub type Rco32kPowerDownR = crate::BitReader;
#[doc = "Field `RCO32K_POWER_DOWN` writer - RCO32K power-down control (1 = powered down, vendor tremo_rcc.c)"]
pub type Rco32kPowerDownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 13:14 - XO32K power-down control (vendor tremo_rcc.c clears both bits to enable)"]
    #[inline(always)]
    pub fn xo32k_power_down(&self) -> Xo32kPowerDownR {
        Xo32kPowerDownR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - RCO32K power-down control (1 = powered down, vendor tremo_rcc.c)"]
    #[inline(always)]
    pub fn rco32k_power_down(&self) -> Rco32kPowerDownR {
        Rco32kPowerDownR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 13:14 - XO32K power-down control (vendor tremo_rcc.c clears both bits to enable)"]
    #[inline(always)]
    pub fn xo32k_power_down(&mut self) -> Xo32kPowerDownW<'_, Analog02Spec> {
        Xo32kPowerDownW::new(self, 13)
    }
    #[doc = "Bit 15 - RCO32K power-down control (1 = powered down, vendor tremo_rcc.c)"]
    #[inline(always)]
    pub fn rco32k_power_down(&mut self) -> Rco32kPowerDownW<'_, Analog02Spec> {
        Rco32kPowerDownW::new(self, 15)
    }
}
#[doc = "Analog register 0x02 (RCO32K/XO32K control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_02::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_02::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Analog02Spec;
impl crate::RegisterSpec for Analog02Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_02::R`](R) reader structure"]
impl crate::Readable for Analog02Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_02::W`](W) writer structure"]
impl crate::Writable for Analog02Spec {
    type Safety = crate::Unsafe;
}

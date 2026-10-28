#[doc = "Register `ANALOG_03` reader"]
pub type R = crate::R<Analog03Spec>;
#[doc = "Register `ANALOG_03` writer"]
pub type W = crate::W<Analog03Spec>;
#[doc = "Field `XO32K_LOW_POWER_ENABLE` reader - XO32K low-power mode enable (vendor tremo_pwr.c)"]
pub type Xo32kLowPowerEnableR = crate::BitReader;
#[doc = "Field `XO32K_LOW_POWER_ENABLE` writer - XO32K low-power mode enable (vendor tremo_pwr.c)"]
pub type Xo32kLowPowerEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - XO32K low-power mode enable (vendor tremo_pwr.c)"]
    #[inline(always)]
    pub fn xo32k_low_power_enable(&self) -> Xo32kLowPowerEnableR {
        Xo32kLowPowerEnableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - XO32K low-power mode enable (vendor tremo_pwr.c)"]
    #[inline(always)]
    pub fn xo32k_low_power_enable(&mut self) -> Xo32kLowPowerEnableW<'_, Analog03Spec> {
        Xo32kLowPowerEnableW::new(self, 7)
    }
}
#[doc = "Analog register 0x03 (XO32K low-power control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_03::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_03::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Analog03Spec;
impl crate::RegisterSpec for Analog03Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_03::R`](R) reader structure"]
impl crate::Readable for Analog03Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_03::W`](W) writer structure"]
impl crate::Writable for Analog03Spec {
    type Safety = crate::Unsafe;
}

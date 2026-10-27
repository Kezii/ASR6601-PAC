#[doc = "Register `ANALOG_05` reader"]
pub type R = crate::R<Analog05Spec>;
#[doc = "Register `ANALOG_05` writer"]
pub type W = crate::W<Analog05Spec>;
#[doc = "Field `LOW_POWER_RUN_ENABLE` reader - Low-power-run control 0 (vendor tremo_pwr.c)"]
pub type LowPowerRunEnableR = crate::BitReader;
#[doc = "Field `LOW_POWER_RUN_ENABLE` writer - Low-power-run control 0 (vendor tremo_pwr.c)"]
pub type LowPowerRunEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Low-power-run control 0 (vendor tremo_pwr.c)"]
    #[inline(always)]
    pub fn low_power_run_enable(&self) -> LowPowerRunEnableR {
        LowPowerRunEnableR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Low-power-run control 0 (vendor tremo_pwr.c)"]
    #[inline(always)]
    pub fn low_power_run_enable(&mut self) -> LowPowerRunEnableW<'_, Analog05Spec> {
        LowPowerRunEnableW::new(self, 3)
    }
}
#[doc = "Analog register 0x05 (low-power-run control 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_05::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_05::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Analog05Spec;
impl crate::RegisterSpec for Analog05Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_05::R`](R) reader structure"]
impl crate::Readable for Analog05Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_05::W`](W) writer structure"]
impl crate::Writable for Analog05Spec {
    type Safety = crate::Unsafe;
}

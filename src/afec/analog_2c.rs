#[doc = "Register `ANALOG_2C` reader"]
pub type R = crate::R<Analog2cSpec>;
#[doc = "Register `ANALOG_2C` writer"]
pub type W = crate::W<Analog2cSpec>;
#[doc = "Field `VBAT_MEASURE_ENABLE` reader - VBAT/3 measurement enable (vendor tremo_adc.c)"]
pub type VbatMeasureEnableR = crate::BitReader;
#[doc = "Field `VBAT_MEASURE_ENABLE` writer - VBAT/3 measurement enable (vendor tremo_adc.c)"]
pub type VbatMeasureEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 25 - VBAT/3 measurement enable (vendor tremo_adc.c)"]
    #[inline(always)]
    pub fn vbat_measure_enable(&self) -> VbatMeasureEnableR {
        VbatMeasureEnableR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - VBAT/3 measurement enable (vendor tremo_adc.c)"]
    #[inline(always)]
    pub fn vbat_measure_enable(&mut self) -> VbatMeasureEnableW<'_, Analog2cSpec> {
        VbatMeasureEnableW::new(self, 25)
    }
}
#[doc = "Analog register 0x2C (VBAT/3 measurement-path control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_2c::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Analog2cSpec;
impl crate::RegisterSpec for Analog2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_2c::R`](R) reader structure"]
impl crate::Readable for Analog2cSpec {}
#[doc = "`write(|w| ..)` method takes [`analog_2c::W`](W) writer structure"]
impl crate::Writable for Analog2cSpec {
    type Safety = crate::Unsafe;
}

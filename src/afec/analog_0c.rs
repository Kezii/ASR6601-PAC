#[doc = "Register `ANALOG_0C` reader"]
pub type R = crate::R<Analog0cSpec>;
#[doc = "Register `ANALOG_0C` writer"]
pub type W = crate::W<Analog0cSpec>;
#[doc = "Field `STOP3_ANALOG_ENABLE` reader - STOP3 analog enable (vendor tremo_pwr.c)"]
pub type Stop3AnalogEnableR = crate::BitReader;
#[doc = "Field `STOP3_ANALOG_ENABLE` writer - STOP3 analog enable (vendor tremo_pwr.c)"]
pub type Stop3AnalogEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - STOP3 analog enable (vendor tremo_pwr.c)"]
    #[inline(always)]
    pub fn stop3_analog_enable(&self) -> Stop3AnalogEnableR {
        Stop3AnalogEnableR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - STOP3 analog enable (vendor tremo_pwr.c)"]
    #[inline(always)]
    pub fn stop3_analog_enable(&mut self) -> Stop3AnalogEnableW<'_, Analog0cSpec> {
        Stop3AnalogEnableW::new(self, 14)
    }
}
#[doc = "Analog register 0x0C (STOP3 analog control)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_0c::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Analog0cSpec;
impl crate::RegisterSpec for Analog0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_0c::R`](R) reader structure"]
impl crate::Readable for Analog0cSpec {}
#[doc = "`write(|w| ..)` method takes [`analog_0c::W`](W) writer structure"]
impl crate::Writable for Analog0cSpec {
    type Safety = crate::Unsafe;
}

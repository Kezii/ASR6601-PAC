#[doc = "Register `RNGCLK` reader"]
pub type R = crate::R<RngclkSpec>;
#[doc = "Register `RNGCLK` writer"]
pub type W = crate::W<RngclkSpec>;
#[doc = "Field `CLOCK_ENABLE` reader - Clock enable"]
pub type ClockEnableR = crate::BitReader;
#[doc = "Field `CLOCK_ENABLE` writer - Clock enable"]
pub type ClockEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - Clock enable"]
    #[inline(always)]
    pub fn clock_enable(&self) -> ClockEnableR {
        ClockEnableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Clock enable"]
    #[inline(always)]
    pub fn clock_enable(&mut self) -> ClockEnableW<'_, RngclkSpec> {
        ClockEnableW::new(self, 7)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngclk::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngclkSpec;
impl crate::RegisterSpec for RngclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngclk::R`](R) reader structure"]
impl crate::Readable for RngclkSpec {}
#[doc = "`write(|w| ..)` method takes [`rngclk::W`](W) writer structure"]
impl crate::Writable for RngclkSpec {
    type Safety = crate::Unsafe;
}

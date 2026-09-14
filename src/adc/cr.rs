#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `EN` reader - adc enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - adc enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS` reader - adc disable"]
pub type DisR = crate::BitReader;
#[doc = "Field `DIS` writer - adc disable"]
pub type DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - adc start conversion command"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - adc start conversion command"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - adc stop conversion command"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - adc stop conversion command"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - adc enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - adc disable"]
    #[inline(always)]
    pub fn dis(&self) -> DisR {
        DisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - adc start conversion command"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - adc stop conversion command"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - adc enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - adc disable"]
    #[inline(always)]
    pub fn dis(&mut self) -> DisW<'_, CrSpec> {
        DisW::new(self, 1)
    }
    #[doc = "Bit 2 - adc start conversion command"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, CrSpec> {
        StartW::new(self, 2)
    }
    #[doc = "Bit 3 - adc stop conversion command"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, CrSpec> {
        StopW::new(self, 3)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}

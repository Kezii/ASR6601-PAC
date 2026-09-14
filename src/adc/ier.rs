#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `EOC_INT_EN` reader - end of conversion interrupt enable"]
pub type EocIntEnR = crate::BitReader;
#[doc = "Field `EOC_INT_EN` writer - end of conversion interrupt enable"]
pub type EocIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS_INT_EN` reader - end of sequence interrupt enable"]
pub type EosIntEnR = crate::BitReader;
#[doc = "Field `EOS_INT_EN` writer - end of sequence interrupt enable"]
pub type EosIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERRUN_INT_EN` reader - overrun interrupt enable"]
pub type OverrunIntEnR = crate::BitReader;
#[doc = "Field `OVERRUN_INT_EN` writer - overrun interrupt enable"]
pub type OverrunIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - end of conversion interrupt enable"]
    #[inline(always)]
    pub fn eoc_int_en(&self) -> EocIntEnR {
        EocIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - end of sequence interrupt enable"]
    #[inline(always)]
    pub fn eos_int_en(&self) -> EosIntEnR {
        EosIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - overrun interrupt enable"]
    #[inline(always)]
    pub fn overrun_int_en(&self) -> OverrunIntEnR {
        OverrunIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - end of conversion interrupt enable"]
    #[inline(always)]
    pub fn eoc_int_en(&mut self) -> EocIntEnW<'_, IerSpec> {
        EocIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - end of sequence interrupt enable"]
    #[inline(always)]
    pub fn eos_int_en(&mut self) -> EosIntEnW<'_, IerSpec> {
        EosIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - overrun interrupt enable"]
    #[inline(always)]
    pub fn overrun_int_en(&mut self) -> OverrunIntEnW<'_, IerSpec> {
        OverrunIntEnW::new(self, 2)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}

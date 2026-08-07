#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `EOC` reader - Eoc"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOC` writer - Eoc"]
pub type EocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS` reader - Eos"]
pub type EosR = crate::BitReader;
#[doc = "Field `EOS` writer - Eos"]
pub type EosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERRUN` reader - Overrun"]
pub type OverrunR = crate::BitReader;
#[doc = "Field `OVERRUN` writer - Overrun"]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Eoc"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Eos"]
    #[inline(always)]
    pub fn eos(&self) -> EosR {
        EosR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overrun"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Eoc"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EocW<'_, IerSpec> {
        EocW::new(self, 0)
    }
    #[doc = "Bit 1 - Eos"]
    #[inline(always)]
    pub fn eos(&mut self) -> EosW<'_, IerSpec> {
        EosW::new(self, 1)
    }
    #[doc = "Bit 2 - Overrun"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OverrunW<'_, IerSpec> {
        OverrunW::new(self, 2)
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

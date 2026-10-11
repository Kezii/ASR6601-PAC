#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `UNDERFLOW` reader - DAC buffer underflow flag"]
pub type UnderflowR = crate::BitReader;
#[doc = "Field `UNDERFLOW` writer - DAC buffer underflow flag"]
pub type UnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTY` reader - DAC buffer empty flag"]
pub type EmptyR = crate::BitReader;
#[doc = "Field `EMPTY` writer - DAC buffer empty flag"]
pub type EmptyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DAC buffer underflow flag"]
    #[inline(always)]
    pub fn underflow(&self) -> UnderflowR {
        UnderflowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC buffer empty flag"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC buffer underflow flag"]
    #[inline(always)]
    pub fn underflow(&mut self) -> UnderflowW<'_, SrSpec> {
        UnderflowW::new(self, 0)
    }
    #[doc = "Bit 1 - DAC buffer empty flag"]
    #[inline(always)]
    pub fn empty(&mut self) -> EmptyW<'_, SrSpec> {
        EmptyW::new(self, 1)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}

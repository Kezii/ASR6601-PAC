#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `RXIC` writer - receive interrupt clear"]
pub type RxicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIC` writer - transmit interrupt clear"]
pub type TxicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIC` writer - receive timeout interrupt clear"]
pub type RticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIC` writer - framing error interrupt clear"]
pub type FeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIC` writer - parity error interrupt clear"]
pub type PeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIC` writer - break error interrupt clear"]
pub type BeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIC` writer - overrun error interrupt clear"]
pub type OeicW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 4 - receive interrupt clear"]
    #[inline(always)]
    pub fn rxic(&mut self) -> RxicW<'_, IcrSpec> {
        RxicW::new(self, 4)
    }
    #[doc = "Bit 5 - transmit interrupt clear"]
    #[inline(always)]
    pub fn txic(&mut self) -> TxicW<'_, IcrSpec> {
        TxicW::new(self, 5)
    }
    #[doc = "Bit 6 - receive timeout interrupt clear"]
    #[inline(always)]
    pub fn rtic(&mut self) -> RticW<'_, IcrSpec> {
        RticW::new(self, 6)
    }
    #[doc = "Bit 7 - framing error interrupt clear"]
    #[inline(always)]
    pub fn feic(&mut self) -> FeicW<'_, IcrSpec> {
        FeicW::new(self, 7)
    }
    #[doc = "Bit 8 - parity error interrupt clear"]
    #[inline(always)]
    pub fn peic(&mut self) -> PeicW<'_, IcrSpec> {
        PeicW::new(self, 8)
    }
    #[doc = "Bit 9 - break error interrupt clear"]
    #[inline(always)]
    pub fn beic(&mut self) -> BeicW<'_, IcrSpec> {
        BeicW::new(self, 9)
    }
    #[doc = "Bit 10 - overrun error interrupt clear"]
    #[inline(always)]
    pub fn oeic(&mut self) -> OeicW<'_, IcrSpec> {
        OeicW::new(self, 10)
    }
}
#[doc = "interrupt clear register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}

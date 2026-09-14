#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `UART_EN` reader - Uart en"]
pub type UartEnR = crate::BitReader;
#[doc = "Field `UART_EN` writer - Uart en"]
pub type UartEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIR_EN` reader - Sir en"]
pub type SirEnR = crate::BitReader;
#[doc = "Field `SIR_EN` writer - Sir en"]
pub type SirEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIR_LPIRDA_EN` reader - Sir lpirda en"]
pub type SirLpirdaEnR = crate::BitReader;
#[doc = "Field `SIR_LPIRDA_EN` writer - Sir lpirda en"]
pub type SirLpirdaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - transmit enable"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - transmit enable"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXE` reader - receive enable"]
pub type RxeR = crate::BitReader;
#[doc = "Field `RXE` writer - receive enable"]
pub type RxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEn` reader - rts hardware flow control enable"]
pub type RtsenR = crate::BitReader;
#[doc = "Field `RTSEn` writer - rts hardware flow control enable"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSEn` reader - cts hardware flow control enable"]
pub type CtsenR = crate::BitReader;
#[doc = "Field `CTSEn` writer - cts hardware flow control enable"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Uart en"]
    #[inline(always)]
    pub fn uart_en(&self) -> UartEnR {
        UartEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sir en"]
    #[inline(always)]
    pub fn sir_en(&self) -> SirEnR {
        SirEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sir lpirda en"]
    #[inline(always)]
    pub fn sir_lpirda_en(&self) -> SirLpirdaEnR {
        SirLpirdaEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - transmit enable"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - receive enable"]
    #[inline(always)]
    pub fn rxe(&self) -> RxeR {
        RxeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - rts hardware flow control enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - cts hardware flow control enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Uart en"]
    #[inline(always)]
    pub fn uart_en(&mut self) -> UartEnW<'_, CrSpec> {
        UartEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Sir en"]
    #[inline(always)]
    pub fn sir_en(&mut self) -> SirEnW<'_, CrSpec> {
        SirEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Sir lpirda en"]
    #[inline(always)]
    pub fn sir_lpirda_en(&mut self) -> SirLpirdaEnW<'_, CrSpec> {
        SirLpirdaEnW::new(self, 2)
    }
    #[doc = "Bit 8 - transmit enable"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, CrSpec> {
        TxeW::new(self, 8)
    }
    #[doc = "Bit 9 - receive enable"]
    #[inline(always)]
    pub fn rxe(&mut self) -> RxeW<'_, CrSpec> {
        RxeW::new(self, 9)
    }
    #[doc = "Bit 14 - rts hardware flow control enable"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RtsenW<'_, CrSpec> {
        RtsenW::new(self, 14)
    }
    #[doc = "Bit 15 - cts hardware flow control enable"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CtsenW<'_, CrSpec> {
        CtsenW::new(self, 15)
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

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
#[doc = "Uart mode"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UartMode {
    #[doc = "0: None"]
    None = 0,
    #[doc = "2: Rx"]
    Rx = 2,
    #[doc = "1: Tx"]
    Tx = 1,
    #[doc = "3: Txrx"]
    Txrx = 3,
}
impl From<UartMode> for u8 {
    #[inline(always)]
    fn from(variant: UartMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UartMode {
    type Ux = u8;
}
impl crate::IsEnum for UartMode {}
#[doc = "Field `UART_MODE` reader - Uart mode"]
pub type UartModeR = crate::FieldReader<UartMode>;
impl UartModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UartMode {
        match self.bits {
            0 => UartMode::None,
            2 => UartMode::Rx,
            1 => UartMode::Tx,
            3 => UartMode::Txrx,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == UartMode::None
    }
    #[doc = "Rx"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == UartMode::Rx
    }
    #[doc = "Tx"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == UartMode::Tx
    }
    #[doc = "Txrx"]
    #[inline(always)]
    pub fn is_txrx(&self) -> bool {
        *self == UartMode::Txrx
    }
}
#[doc = "Field `UART_MODE` writer - Uart mode"]
pub type UartModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, UartMode, crate::Safe>;
impl<'a, REG> UartModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(UartMode::None)
    }
    #[doc = "Rx"]
    #[inline(always)]
    pub fn rx(self) -> &'a mut crate::W<REG> {
        self.variant(UartMode::Rx)
    }
    #[doc = "Tx"]
    #[inline(always)]
    pub fn tx(self) -> &'a mut crate::W<REG> {
        self.variant(UartMode::Tx)
    }
    #[doc = "Txrx"]
    #[inline(always)]
    pub fn txrx(self) -> &'a mut crate::W<REG> {
        self.variant(UartMode::Txrx)
    }
}
#[doc = "Flow ctrl"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FlowCtrl {
    #[doc = "2: Cts"]
    Cts = 2,
    #[doc = "3: Cts rts"]
    CtsRts = 3,
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Rts"]
    Rts = 1,
}
impl From<FlowCtrl> for u8 {
    #[inline(always)]
    fn from(variant: FlowCtrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FlowCtrl {
    type Ux = u8;
}
impl crate::IsEnum for FlowCtrl {}
#[doc = "Field `FLOW_CTRL` reader - Flow ctrl"]
pub type FlowCtrlR = crate::FieldReader<FlowCtrl>;
impl FlowCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlowCtrl {
        match self.bits {
            2 => FlowCtrl::Cts,
            3 => FlowCtrl::CtsRts,
            0 => FlowCtrl::None,
            1 => FlowCtrl::Rts,
            _ => unreachable!(),
        }
    }
    #[doc = "Cts"]
    #[inline(always)]
    pub fn is_cts(&self) -> bool {
        *self == FlowCtrl::Cts
    }
    #[doc = "Cts rts"]
    #[inline(always)]
    pub fn is_cts_rts(&self) -> bool {
        *self == FlowCtrl::CtsRts
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == FlowCtrl::None
    }
    #[doc = "Rts"]
    #[inline(always)]
    pub fn is_rts(&self) -> bool {
        *self == FlowCtrl::Rts
    }
}
#[doc = "Field `FLOW_CTRL` writer - Flow ctrl"]
pub type FlowCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2, FlowCtrl, crate::Safe>;
impl<'a, REG> FlowCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Cts"]
    #[inline(always)]
    pub fn cts(self) -> &'a mut crate::W<REG> {
        self.variant(FlowCtrl::Cts)
    }
    #[doc = "Cts rts"]
    #[inline(always)]
    pub fn cts_rts(self) -> &'a mut crate::W<REG> {
        self.variant(FlowCtrl::CtsRts)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(FlowCtrl::None)
    }
    #[doc = "Rts"]
    #[inline(always)]
    pub fn rts(self) -> &'a mut crate::W<REG> {
        self.variant(FlowCtrl::Rts)
    }
}
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
    #[doc = "Bits 8:9 - Uart mode"]
    #[inline(always)]
    pub fn uart_mode(&self) -> UartModeR {
        UartModeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Flow ctrl"]
    #[inline(always)]
    pub fn flow_ctrl(&self) -> FlowCtrlR {
        FlowCtrlR::new(((self.bits >> 14) & 3) as u8)
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
    #[doc = "Bits 8:9 - Uart mode"]
    #[inline(always)]
    pub fn uart_mode(&mut self) -> UartModeW<'_, CrSpec> {
        UartModeW::new(self, 8)
    }
    #[doc = "Bits 14:15 - Flow ctrl"]
    #[inline(always)]
    pub fn flow_ctrl(&mut self) -> FlowCtrlW<'_, CrSpec> {
        FlowCtrlW::new(self, 14)
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

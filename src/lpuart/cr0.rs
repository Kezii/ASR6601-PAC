#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `LPUART_DATA_LEN` reader - Lpuart data length, data width = value + 5"]
pub type LpuartDataLenR = crate::FieldReader;
#[doc = "Field `LPUART_DATA_LEN` writer - Lpuart data length, data width = value + 5"]
pub type LpuartDataLenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Lpuart parity bit configuration"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LpuartParityCfg {
    #[doc = "0: even parity"]
    Even = 0,
    #[doc = "1: odd parity"]
    Odd = 1,
    #[doc = "2: parity bit is 0"]
    Bit0 = 2,
    #[doc = "3: parity bit is 1"]
    Bit1 = 3,
    #[doc = "4: no parity"]
    None = 4,
}
impl From<LpuartParityCfg> for u8 {
    #[inline(always)]
    fn from(variant: LpuartParityCfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LpuartParityCfg {
    type Ux = u8;
}
impl crate::IsEnum for LpuartParityCfg {}
#[doc = "Field `LPUART_PARITY_CFG` reader - Lpuart parity bit configuration"]
pub type LpuartParityCfgR = crate::FieldReader<LpuartParityCfg>;
impl LpuartParityCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LpuartParityCfg> {
        match self.bits {
            0 => Some(LpuartParityCfg::Even),
            1 => Some(LpuartParityCfg::Odd),
            2 => Some(LpuartParityCfg::Bit0),
            3 => Some(LpuartParityCfg::Bit1),
            4 => Some(LpuartParityCfg::None),
            _ => None,
        }
    }
    #[doc = "even parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == LpuartParityCfg::Even
    }
    #[doc = "odd parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == LpuartParityCfg::Odd
    }
    #[doc = "parity bit is 0"]
    #[inline(always)]
    pub fn is_bit0(&self) -> bool {
        *self == LpuartParityCfg::Bit0
    }
    #[doc = "parity bit is 1"]
    #[inline(always)]
    pub fn is_bit1(&self) -> bool {
        *self == LpuartParityCfg::Bit1
    }
    #[doc = "no parity"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LpuartParityCfg::None
    }
}
#[doc = "Field `LPUART_PARITY_CFG` writer - Lpuart parity bit configuration"]
pub type LpuartParityCfgW<'a, REG> = crate::FieldWriter<'a, REG, 3, LpuartParityCfg>;
impl<'a, REG> LpuartParityCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(LpuartParityCfg::Even)
    }
    #[doc = "odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(LpuartParityCfg::Odd)
    }
    #[doc = "parity bit is 0"]
    #[inline(always)]
    pub fn bit0(self) -> &'a mut crate::W<REG> {
        self.variant(LpuartParityCfg::Bit0)
    }
    #[doc = "parity bit is 1"]
    #[inline(always)]
    pub fn bit1(self) -> &'a mut crate::W<REG> {
        self.variant(LpuartParityCfg::Bit1)
    }
    #[doc = "no parity"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(LpuartParityCfg::None)
    }
}
#[doc = "Lpuart stop bits configuration"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpuartStopLen {
    #[doc = "0: 1 stop bit"]
    Value1 = 0,
    #[doc = "1: 2 stop bits"]
    Value2 = 1,
}
impl From<LpuartStopLen> for bool {
    #[inline(always)]
    fn from(variant: LpuartStopLen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART_STOP_LEN` reader - Lpuart stop bits configuration"]
pub type LpuartStopLenR = crate::BitReader<LpuartStopLen>;
impl LpuartStopLenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpuartStopLen {
        match self.bits {
            false => LpuartStopLen::Value1,
            true => LpuartStopLen::Value2,
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == LpuartStopLen::Value1
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == LpuartStopLen::Value2
    }
}
#[doc = "Field `LPUART_STOP_LEN` writer - Lpuart stop bits configuration"]
pub type LpuartStopLenW<'a, REG> = crate::BitWriter<'a, REG, LpuartStopLen>;
impl<'a, REG> LpuartStopLenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(LpuartStopLen::Value1)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(LpuartStopLen::Value2)
    }
}
#[doc = "Field `LPUART_BAUD_RATE_FRA` reader - Lpuart baud rate divisor fractional part"]
pub type LpuartBaudRateFraR = crate::FieldReader;
#[doc = "Field `LPUART_BAUD_RATE_FRA` writer - Lpuart baud rate divisor fractional part"]
pub type LpuartBaudRateFraW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPUART_BAUD_RATE_INT` reader - Lpuart baud rate divisor integer part"]
pub type LpuartBaudRateIntR = crate::FieldReader<u16>;
#[doc = "Field `LPUART_BAUD_RATE_INT` writer - Lpuart baud rate divisor integer part"]
pub type LpuartBaudRateIntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `LOW_LEVEL_WAKEUP` reader - Low level wakeup"]
pub type LowLevelWakeupR = crate::BitReader;
#[doc = "Field `LOW_LEVEL_WAKEUP` writer - Low level wakeup"]
pub type LowLevelWakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START_WAKEUP` reader - Start wakeup"]
pub type StartWakeupR = crate::BitReader;
#[doc = "Field `START_WAKEUP` writer - Start wakeup"]
pub type StartWakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DONE_WAKEUP` reader - Rx done wakeup"]
pub type RxDoneWakeupR = crate::BitReader;
#[doc = "Field `RX_DONE_WAKEUP` writer - Rx done wakeup"]
pub type RxDoneWakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ENABLE` reader - Rx enable"]
pub type RxEnableR = crate::BitReader;
#[doc = "Field `RX_ENABLE` writer - Rx enable"]
pub type RxEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS_ENABLE` reader - Rts enable"]
pub type RtsEnableR = crate::BitReader;
#[doc = "Field `RTS_ENABLE` writer - Rts enable"]
pub type RtsEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Lpuart data length, data width = value + 5"]
    #[inline(always)]
    pub fn lpuart_data_len(&self) -> LpuartDataLenR {
        LpuartDataLenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Lpuart parity bit configuration"]
    #[inline(always)]
    pub fn lpuart_parity_cfg(&self) -> LpuartParityCfgR {
        LpuartParityCfgR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Lpuart stop bits configuration"]
    #[inline(always)]
    pub fn lpuart_stop_len(&self) -> LpuartStopLenR {
        LpuartStopLenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - Lpuart baud rate divisor fractional part"]
    #[inline(always)]
    pub fn lpuart_baud_rate_fra(&self) -> LpuartBaudRateFraR {
        LpuartBaudRateFraR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:21 - Lpuart baud rate divisor integer part"]
    #[inline(always)]
    pub fn lpuart_baud_rate_int(&self) -> LpuartBaudRateIntR {
        LpuartBaudRateIntR::new(((self.bits >> 10) & 0x0fff) as u16)
    }
    #[doc = "Bit 22 - Low level wakeup"]
    #[inline(always)]
    pub fn low_level_wakeup(&self) -> LowLevelWakeupR {
        LowLevelWakeupR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Start wakeup"]
    #[inline(always)]
    pub fn start_wakeup(&self) -> StartWakeupR {
        StartWakeupR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Rx done wakeup"]
    #[inline(always)]
    pub fn rx_done_wakeup(&self) -> RxDoneWakeupR {
        RxDoneWakeupR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx enable"]
    #[inline(always)]
    pub fn rx_enable(&self) -> RxEnableR {
        RxEnableR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Rts enable"]
    #[inline(always)]
    pub fn rts_enable(&self) -> RtsEnableR {
        RtsEnableR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Lpuart data length, data width = value + 5"]
    #[inline(always)]
    pub fn lpuart_data_len(&mut self) -> LpuartDataLenW<'_, Cr0Spec> {
        LpuartDataLenW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Lpuart parity bit configuration"]
    #[inline(always)]
    pub fn lpuart_parity_cfg(&mut self) -> LpuartParityCfgW<'_, Cr0Spec> {
        LpuartParityCfgW::new(self, 2)
    }
    #[doc = "Bit 5 - Lpuart stop bits configuration"]
    #[inline(always)]
    pub fn lpuart_stop_len(&mut self) -> LpuartStopLenW<'_, Cr0Spec> {
        LpuartStopLenW::new(self, 5)
    }
    #[doc = "Bits 6:9 - Lpuart baud rate divisor fractional part"]
    #[inline(always)]
    pub fn lpuart_baud_rate_fra(&mut self) -> LpuartBaudRateFraW<'_, Cr0Spec> {
        LpuartBaudRateFraW::new(self, 6)
    }
    #[doc = "Bits 10:21 - Lpuart baud rate divisor integer part"]
    #[inline(always)]
    pub fn lpuart_baud_rate_int(&mut self) -> LpuartBaudRateIntW<'_, Cr0Spec> {
        LpuartBaudRateIntW::new(self, 10)
    }
    #[doc = "Bit 22 - Low level wakeup"]
    #[inline(always)]
    pub fn low_level_wakeup(&mut self) -> LowLevelWakeupW<'_, Cr0Spec> {
        LowLevelWakeupW::new(self, 22)
    }
    #[doc = "Bit 23 - Start wakeup"]
    #[inline(always)]
    pub fn start_wakeup(&mut self) -> StartWakeupW<'_, Cr0Spec> {
        StartWakeupW::new(self, 23)
    }
    #[doc = "Bit 24 - Rx done wakeup"]
    #[inline(always)]
    pub fn rx_done_wakeup(&mut self) -> RxDoneWakeupW<'_, Cr0Spec> {
        RxDoneWakeupW::new(self, 24)
    }
    #[doc = "Bit 25 - Rx enable"]
    #[inline(always)]
    pub fn rx_enable(&mut self) -> RxEnableW<'_, Cr0Spec> {
        RxEnableW::new(self, 25)
    }
    #[doc = "Bit 26 - Rts enable"]
    #[inline(always)]
    pub fn rts_enable(&mut self) -> RtsEnableW<'_, Cr0Spec> {
        RtsEnableW::new(self, 26)
    }
}
#[doc = "control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}

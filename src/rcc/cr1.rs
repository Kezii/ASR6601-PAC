#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Iwdg clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IwdgClkSel {
    #[doc = "1: Rco32k"]
    Rco32k = 1,
    #[doc = "0: Xo32k"]
    Xo32k = 0,
}
impl From<IwdgClkSel> for bool {
    #[inline(always)]
    fn from(variant: IwdgClkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDG_CLK_SEL` reader - Iwdg clk sel"]
pub type IwdgClkSelR = crate::BitReader<IwdgClkSel>;
impl IwdgClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IwdgClkSel {
        match self.bits {
            true => IwdgClkSel::Rco32k,
            false => IwdgClkSel::Xo32k,
        }
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn is_rco32k(&self) -> bool {
        *self == IwdgClkSel::Rco32k
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == IwdgClkSel::Xo32k
    }
}
#[doc = "Field `IWDG_CLK_SEL` writer - Iwdg clk sel"]
pub type IwdgClkSelW<'a, REG> = crate::BitWriter<'a, REG, IwdgClkSel>;
impl<'a, REG> IwdgClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn rco32k(self) -> &'a mut crate::W<REG> {
        self.variant(IwdgClkSel::Rco32k)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(IwdgClkSel::Xo32k)
    }
}
#[doc = "Rtc clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcClkSel {
    #[doc = "1: Rco32k"]
    Rco32k = 1,
    #[doc = "0: Xo32k"]
    Xo32k = 0,
}
impl From<RtcClkSel> for bool {
    #[inline(always)]
    fn from(variant: RtcClkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_CLK_SEL` reader - Rtc clk sel"]
pub type RtcClkSelR = crate::BitReader<RtcClkSel>;
impl RtcClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcClkSel {
        match self.bits {
            true => RtcClkSel::Rco32k,
            false => RtcClkSel::Xo32k,
        }
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn is_rco32k(&self) -> bool {
        *self == RtcClkSel::Rco32k
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == RtcClkSel::Xo32k
    }
}
#[doc = "Field `RTC_CLK_SEL` writer - Rtc clk sel"]
pub type RtcClkSelW<'a, REG> = crate::BitWriter<'a, REG, RtcClkSel>;
impl<'a, REG> RtcClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn rco32k(self) -> &'a mut crate::W<REG> {
        self.variant(RtcClkSel::Rco32k)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(RtcClkSel::Xo32k)
    }
}
#[doc = "Lpuart clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LpuartClkSel {
    #[doc = "1: Rco32k"]
    Rco32k = 1,
    #[doc = "2: Rco4m"]
    Rco4m = 2,
    #[doc = "0: Xo32k"]
    Xo32k = 0,
}
impl From<LpuartClkSel> for u8 {
    #[inline(always)]
    fn from(variant: LpuartClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LpuartClkSel {
    type Ux = u8;
}
impl crate::IsEnum for LpuartClkSel {}
#[doc = "Field `LPUART_CLK_SEL` reader - Lpuart clk sel"]
pub type LpuartClkSelR = crate::FieldReader<LpuartClkSel>;
impl LpuartClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LpuartClkSel> {
        match self.bits {
            1 => Some(LpuartClkSel::Rco32k),
            2 => Some(LpuartClkSel::Rco4m),
            0 => Some(LpuartClkSel::Xo32k),
            _ => None,
        }
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn is_rco32k(&self) -> bool {
        *self == LpuartClkSel::Rco32k
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn is_rco4m(&self) -> bool {
        *self == LpuartClkSel::Rco4m
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == LpuartClkSel::Xo32k
    }
}
#[doc = "Field `LPUART_CLK_SEL` writer - Lpuart clk sel"]
pub type LpuartClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, LpuartClkSel>;
impl<'a, REG> LpuartClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn rco32k(self) -> &'a mut crate::W<REG> {
        self.variant(LpuartClkSel::Rco32k)
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn rco4m(self) -> &'a mut crate::W<REG> {
        self.variant(LpuartClkSel::Rco4m)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(LpuartClkSel::Xo32k)
    }
}
#[doc = "Lcd clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LcdClkSel {
    #[doc = "1: Rco32k"]
    Rco32k = 1,
    #[doc = "2: Rco4m"]
    Rco4m = 2,
    #[doc = "0: Xo32k"]
    Xo32k = 0,
}
impl From<LcdClkSel> for u8 {
    #[inline(always)]
    fn from(variant: LcdClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LcdClkSel {
    type Ux = u8;
}
impl crate::IsEnum for LcdClkSel {}
#[doc = "Field `LCD_CLK_SEL` reader - Lcd clk sel"]
pub type LcdClkSelR = crate::FieldReader<LcdClkSel>;
impl LcdClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LcdClkSel> {
        match self.bits {
            1 => Some(LcdClkSel::Rco32k),
            2 => Some(LcdClkSel::Rco4m),
            0 => Some(LcdClkSel::Xo32k),
            _ => None,
        }
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn is_rco32k(&self) -> bool {
        *self == LcdClkSel::Rco32k
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn is_rco4m(&self) -> bool {
        *self == LcdClkSel::Rco4m
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == LcdClkSel::Xo32k
    }
}
#[doc = "Field `LCD_CLK_SEL` writer - Lcd clk sel"]
pub type LcdClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, LcdClkSel>;
impl<'a, REG> LcdClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn rco32k(self) -> &'a mut crate::W<REG> {
        self.variant(LcdClkSel::Rco32k)
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn rco4m(self) -> &'a mut crate::W<REG> {
        self.variant(LcdClkSel::Rco4m)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(LcdClkSel::Xo32k)
    }
}
#[doc = "Lptimer0 clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lptimer0ClkSel {
    #[doc = "0: Pclk0"]
    Pclk0 = 0,
    #[doc = "3: Rco32k"]
    Rco32k = 3,
    #[doc = "1: Rco4m"]
    Rco4m = 1,
    #[doc = "2: Xo32k"]
    Xo32k = 2,
}
impl From<Lptimer0ClkSel> for u8 {
    #[inline(always)]
    fn from(variant: Lptimer0ClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lptimer0ClkSel {
    type Ux = u8;
}
impl crate::IsEnum for Lptimer0ClkSel {}
#[doc = "Field `LPTIMER0_CLK_SEL` reader - Lptimer0 clk sel"]
pub type Lptimer0ClkSelR = crate::FieldReader<Lptimer0ClkSel>;
impl Lptimer0ClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptimer0ClkSel {
        match self.bits {
            0 => Lptimer0ClkSel::Pclk0,
            3 => Lptimer0ClkSel::Rco32k,
            1 => Lptimer0ClkSel::Rco4m,
            2 => Lptimer0ClkSel::Xo32k,
            _ => unreachable!(),
        }
    }
    #[doc = "Pclk0"]
    #[inline(always)]
    pub fn is_pclk0(&self) -> bool {
        *self == Lptimer0ClkSel::Pclk0
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn is_rco32k(&self) -> bool {
        *self == Lptimer0ClkSel::Rco32k
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn is_rco4m(&self) -> bool {
        *self == Lptimer0ClkSel::Rco4m
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == Lptimer0ClkSel::Xo32k
    }
}
#[doc = "Field `LPTIMER0_CLK_SEL` writer - Lptimer0 clk sel"]
pub type Lptimer0ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lptimer0ClkSel, crate::Safe>;
impl<'a, REG> Lptimer0ClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pclk0"]
    #[inline(always)]
    pub fn pclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptimer0ClkSel::Pclk0)
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn rco32k(self) -> &'a mut crate::W<REG> {
        self.variant(Lptimer0ClkSel::Rco32k)
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn rco4m(self) -> &'a mut crate::W<REG> {
        self.variant(Lptimer0ClkSel::Rco4m)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(Lptimer0ClkSel::Xo32k)
    }
}
#[doc = "Lptimer1 clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lptimer1ClkSel {
    #[doc = "0: Pclk0"]
    Pclk0 = 0,
    #[doc = "3: Rco32k"]
    Rco32k = 3,
    #[doc = "1: Rco4m"]
    Rco4m = 1,
    #[doc = "2: Xo32k"]
    Xo32k = 2,
}
impl From<Lptimer1ClkSel> for u8 {
    #[inline(always)]
    fn from(variant: Lptimer1ClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lptimer1ClkSel {
    type Ux = u8;
}
impl crate::IsEnum for Lptimer1ClkSel {}
#[doc = "Field `LPTIMER1_CLK_SEL` reader - Lptimer1 clk sel"]
pub type Lptimer1ClkSelR = crate::FieldReader<Lptimer1ClkSel>;
impl Lptimer1ClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptimer1ClkSel {
        match self.bits {
            0 => Lptimer1ClkSel::Pclk0,
            3 => Lptimer1ClkSel::Rco32k,
            1 => Lptimer1ClkSel::Rco4m,
            2 => Lptimer1ClkSel::Xo32k,
            _ => unreachable!(),
        }
    }
    #[doc = "Pclk0"]
    #[inline(always)]
    pub fn is_pclk0(&self) -> bool {
        *self == Lptimer1ClkSel::Pclk0
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn is_rco32k(&self) -> bool {
        *self == Lptimer1ClkSel::Rco32k
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn is_rco4m(&self) -> bool {
        *self == Lptimer1ClkSel::Rco4m
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == Lptimer1ClkSel::Xo32k
    }
}
#[doc = "Field `LPTIMER1_CLK_SEL` writer - Lptimer1 clk sel"]
pub type Lptimer1ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lptimer1ClkSel, crate::Safe>;
impl<'a, REG> Lptimer1ClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pclk0"]
    #[inline(always)]
    pub fn pclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptimer1ClkSel::Pclk0)
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn rco32k(self) -> &'a mut crate::W<REG> {
        self.variant(Lptimer1ClkSel::Rco32k)
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn rco4m(self) -> &'a mut crate::W<REG> {
        self.variant(Lptimer1ClkSel::Rco4m)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(Lptimer1ClkSel::Xo32k)
    }
}
#[doc = "Field `LPTIMER0_EXTCLK_SEL` reader - Lptimer0 extclk sel"]
pub type Lptimer0ExtclkSelR = crate::BitReader;
#[doc = "Field `LPTIMER0_EXTCLK_SEL` writer - Lptimer0 extclk sel"]
pub type Lptimer0ExtclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIMER1_EXTCLK_SEL` reader - Lptimer1 extclk sel"]
pub type Lptimer1ExtclkSelR = crate::BitReader;
#[doc = "Field `LPTIMER1_EXTCLK_SEL` writer - Lptimer1 extclk sel"]
pub type Lptimer1ExtclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Iwdg clk sel"]
    #[inline(always)]
    pub fn iwdg_clk_sel(&self) -> IwdgClkSelR {
        IwdgClkSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rtc clk sel"]
    #[inline(always)]
    pub fn rtc_clk_sel(&self) -> RtcClkSelR {
        RtcClkSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Lpuart clk sel"]
    #[inline(always)]
    pub fn lpuart_clk_sel(&self) -> LpuartClkSelR {
        LpuartClkSelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Lcd clk sel"]
    #[inline(always)]
    pub fn lcd_clk_sel(&self) -> LcdClkSelR {
        LcdClkSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Lptimer0 clk sel"]
    #[inline(always)]
    pub fn lptimer0_clk_sel(&self) -> Lptimer0ClkSelR {
        Lptimer0ClkSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Lptimer1 clk sel"]
    #[inline(always)]
    pub fn lptimer1_clk_sel(&self) -> Lptimer1ClkSelR {
        Lptimer1ClkSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Lptimer0 extclk sel"]
    #[inline(always)]
    pub fn lptimer0_extclk_sel(&self) -> Lptimer0ExtclkSelR {
        Lptimer0ExtclkSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Lptimer1 extclk sel"]
    #[inline(always)]
    pub fn lptimer1_extclk_sel(&self) -> Lptimer1ExtclkSelR {
        Lptimer1ExtclkSelR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Iwdg clk sel"]
    #[inline(always)]
    pub fn iwdg_clk_sel(&mut self) -> IwdgClkSelW<'_, Cr1Spec> {
        IwdgClkSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Rtc clk sel"]
    #[inline(always)]
    pub fn rtc_clk_sel(&mut self) -> RtcClkSelW<'_, Cr1Spec> {
        RtcClkSelW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Lpuart clk sel"]
    #[inline(always)]
    pub fn lpuart_clk_sel(&mut self) -> LpuartClkSelW<'_, Cr1Spec> {
        LpuartClkSelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Lcd clk sel"]
    #[inline(always)]
    pub fn lcd_clk_sel(&mut self) -> LcdClkSelW<'_, Cr1Spec> {
        LcdClkSelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Lptimer0 clk sel"]
    #[inline(always)]
    pub fn lptimer0_clk_sel(&mut self) -> Lptimer0ClkSelW<'_, Cr1Spec> {
        Lptimer0ClkSelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Lptimer1 clk sel"]
    #[inline(always)]
    pub fn lptimer1_clk_sel(&mut self) -> Lptimer1ClkSelW<'_, Cr1Spec> {
        Lptimer1ClkSelW::new(self, 8)
    }
    #[doc = "Bit 10 - Lptimer0 extclk sel"]
    #[inline(always)]
    pub fn lptimer0_extclk_sel(&mut self) -> Lptimer0ExtclkSelW<'_, Cr1Spec> {
        Lptimer0ExtclkSelW::new(self, 10)
    }
    #[doc = "Bit 11 - Lptimer1 extclk sel"]
    #[inline(always)]
    pub fn lptimer1_extclk_sel(&mut self) -> Lptimer1ExtclkSelW<'_, Cr1Spec> {
        Lptimer1ExtclkSelW::new(self, 11)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}

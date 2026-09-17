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
#[doc = "Lcdctrl clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LcdctrlClkSel {
    #[doc = "1: Rco32k"]
    Rco32k = 1,
    #[doc = "2: Rco4m"]
    Rco4m = 2,
    #[doc = "0: Xo32k"]
    Xo32k = 0,
}
impl From<LcdctrlClkSel> for u8 {
    #[inline(always)]
    fn from(variant: LcdctrlClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LcdctrlClkSel {
    type Ux = u8;
}
impl crate::IsEnum for LcdctrlClkSel {}
#[doc = "Field `LCDCTRL_CLK_SEL` reader - Lcdctrl clk sel"]
pub type LcdctrlClkSelR = crate::FieldReader<LcdctrlClkSel>;
impl LcdctrlClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LcdctrlClkSel> {
        match self.bits {
            1 => Some(LcdctrlClkSel::Rco32k),
            2 => Some(LcdctrlClkSel::Rco4m),
            0 => Some(LcdctrlClkSel::Xo32k),
            _ => None,
        }
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn is_rco32k(&self) -> bool {
        *self == LcdctrlClkSel::Rco32k
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn is_rco4m(&self) -> bool {
        *self == LcdctrlClkSel::Rco4m
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == LcdctrlClkSel::Xo32k
    }
}
#[doc = "Field `LCDCTRL_CLK_SEL` writer - Lcdctrl clk sel"]
pub type LcdctrlClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, LcdctrlClkSel>;
impl<'a, REG> LcdctrlClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn rco32k(self) -> &'a mut crate::W<REG> {
        self.variant(LcdctrlClkSel::Rco32k)
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn rco4m(self) -> &'a mut crate::W<REG> {
        self.variant(LcdctrlClkSel::Rco4m)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(LcdctrlClkSel::Xo32k)
    }
}
#[doc = "Lptim0 clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lptim0ClkSel {
    #[doc = "0: Pclk0"]
    Pclk0 = 0,
    #[doc = "3: Rco32k"]
    Rco32k = 3,
    #[doc = "1: Rco4m"]
    Rco4m = 1,
    #[doc = "2: Xo32k"]
    Xo32k = 2,
}
impl From<Lptim0ClkSel> for u8 {
    #[inline(always)]
    fn from(variant: Lptim0ClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lptim0ClkSel {
    type Ux = u8;
}
impl crate::IsEnum for Lptim0ClkSel {}
#[doc = "Field `LPTIM0_CLK_SEL` reader - Lptim0 clk sel"]
pub type Lptim0ClkSelR = crate::FieldReader<Lptim0ClkSel>;
impl Lptim0ClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim0ClkSel {
        match self.bits {
            0 => Lptim0ClkSel::Pclk0,
            3 => Lptim0ClkSel::Rco32k,
            1 => Lptim0ClkSel::Rco4m,
            2 => Lptim0ClkSel::Xo32k,
            _ => unreachable!(),
        }
    }
    #[doc = "Pclk0"]
    #[inline(always)]
    pub fn is_pclk0(&self) -> bool {
        *self == Lptim0ClkSel::Pclk0
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn is_rco32k(&self) -> bool {
        *self == Lptim0ClkSel::Rco32k
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn is_rco4m(&self) -> bool {
        *self == Lptim0ClkSel::Rco4m
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == Lptim0ClkSel::Xo32k
    }
}
#[doc = "Field `LPTIM0_CLK_SEL` writer - Lptim0 clk sel"]
pub type Lptim0ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lptim0ClkSel, crate::Safe>;
impl<'a, REG> Lptim0ClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pclk0"]
    #[inline(always)]
    pub fn pclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim0ClkSel::Pclk0)
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn rco32k(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim0ClkSel::Rco32k)
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn rco4m(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim0ClkSel::Rco4m)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim0ClkSel::Xo32k)
    }
}
#[doc = "Lptim1 clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lptim1ClkSel {
    #[doc = "0: Pclk0"]
    Pclk0 = 0,
    #[doc = "3: Rco32k"]
    Rco32k = 3,
    #[doc = "1: Rco4m"]
    Rco4m = 1,
    #[doc = "2: Xo32k"]
    Xo32k = 2,
}
impl From<Lptim1ClkSel> for u8 {
    #[inline(always)]
    fn from(variant: Lptim1ClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lptim1ClkSel {
    type Ux = u8;
}
impl crate::IsEnum for Lptim1ClkSel {}
#[doc = "Field `LPTIM1_CLK_SEL` reader - Lptim1 clk sel"]
pub type Lptim1ClkSelR = crate::FieldReader<Lptim1ClkSel>;
impl Lptim1ClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim1ClkSel {
        match self.bits {
            0 => Lptim1ClkSel::Pclk0,
            3 => Lptim1ClkSel::Rco32k,
            1 => Lptim1ClkSel::Rco4m,
            2 => Lptim1ClkSel::Xo32k,
            _ => unreachable!(),
        }
    }
    #[doc = "Pclk0"]
    #[inline(always)]
    pub fn is_pclk0(&self) -> bool {
        *self == Lptim1ClkSel::Pclk0
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn is_rco32k(&self) -> bool {
        *self == Lptim1ClkSel::Rco32k
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn is_rco4m(&self) -> bool {
        *self == Lptim1ClkSel::Rco4m
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == Lptim1ClkSel::Xo32k
    }
}
#[doc = "Field `LPTIM1_CLK_SEL` writer - Lptim1 clk sel"]
pub type Lptim1ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lptim1ClkSel, crate::Safe>;
impl<'a, REG> Lptim1ClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pclk0"]
    #[inline(always)]
    pub fn pclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1ClkSel::Pclk0)
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn rco32k(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1ClkSel::Rco32k)
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn rco4m(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1ClkSel::Rco4m)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1ClkSel::Xo32k)
    }
}
#[doc = "Field `LPTIM0_EXT_CLK_SEL` reader - Lptim0 extclk sel"]
pub type Lptim0ExtClkSelR = crate::BitReader;
#[doc = "Field `LPTIM0_EXT_CLK_SEL` writer - Lptim0 extclk sel"]
pub type Lptim0ExtClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1_EXT_CLK_SEL` reader - Lptim1 extclk sel"]
pub type Lptim1ExtClkSelR = crate::BitReader;
#[doc = "Field `LPTIM1_EXT_CLK_SEL` writer - Lptim1 extclk sel"]
pub type Lptim1ExtClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bits 4:5 - Lcdctrl clk sel"]
    #[inline(always)]
    pub fn lcdctrl_clk_sel(&self) -> LcdctrlClkSelR {
        LcdctrlClkSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Lptim0 clk sel"]
    #[inline(always)]
    pub fn lptim0_clk_sel(&self) -> Lptim0ClkSelR {
        Lptim0ClkSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Lptim1 clk sel"]
    #[inline(always)]
    pub fn lptim1_clk_sel(&self) -> Lptim1ClkSelR {
        Lptim1ClkSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Lptim0 extclk sel"]
    #[inline(always)]
    pub fn lptim0_ext_clk_sel(&self) -> Lptim0ExtClkSelR {
        Lptim0ExtClkSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Lptim1 extclk sel"]
    #[inline(always)]
    pub fn lptim1_ext_clk_sel(&self) -> Lptim1ExtClkSelR {
        Lptim1ExtClkSelR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bits 4:5 - Lcdctrl clk sel"]
    #[inline(always)]
    pub fn lcdctrl_clk_sel(&mut self) -> LcdctrlClkSelW<'_, Cr1Spec> {
        LcdctrlClkSelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Lptim0 clk sel"]
    #[inline(always)]
    pub fn lptim0_clk_sel(&mut self) -> Lptim0ClkSelW<'_, Cr1Spec> {
        Lptim0ClkSelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Lptim1 clk sel"]
    #[inline(always)]
    pub fn lptim1_clk_sel(&mut self) -> Lptim1ClkSelW<'_, Cr1Spec> {
        Lptim1ClkSelW::new(self, 8)
    }
    #[doc = "Bit 10 - Lptim0 extclk sel"]
    #[inline(always)]
    pub fn lptim0_ext_clk_sel(&mut self) -> Lptim0ExtClkSelW<'_, Cr1Spec> {
        Lptim0ExtClkSelW::new(self, 10)
    }
    #[doc = "Bit 11 - Lptim1 extclk sel"]
    #[inline(always)]
    pub fn lptim1_ext_clk_sel(&mut self) -> Lptim1ExtClkSelW<'_, Cr1Spec> {
        Lptim1ExtClkSelW::new(self, 11)
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

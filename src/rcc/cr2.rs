#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Qspi clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QspiClkSel {
    #[doc = "0: Hclk"]
    Hclk = 0,
    #[doc = "2: Pll"]
    Pll = 2,
    #[doc = "1: Sysclk"]
    Sysclk = 1,
}
impl From<QspiClkSel> for u8 {
    #[inline(always)]
    fn from(variant: QspiClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QspiClkSel {
    type Ux = u8;
}
impl crate::IsEnum for QspiClkSel {}
#[doc = "Field `QSPI_CLK_SEL` reader - Qspi clk sel"]
pub type QspiClkSelR = crate::FieldReader<QspiClkSel>;
impl QspiClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<QspiClkSel> {
        match self.bits {
            0 => Some(QspiClkSel::Hclk),
            2 => Some(QspiClkSel::Pll),
            1 => Some(QspiClkSel::Sysclk),
            _ => None,
        }
    }
    #[doc = "Hclk"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == QspiClkSel::Hclk
    }
    #[doc = "Pll"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == QspiClkSel::Pll
    }
    #[doc = "Sysclk"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == QspiClkSel::Sysclk
    }
}
#[doc = "Field `QSPI_CLK_SEL` writer - Qspi clk sel"]
pub type QspiClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, QspiClkSel>;
impl<'a, REG> QspiClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Hclk"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(QspiClkSel::Hclk)
    }
    #[doc = "Pll"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(QspiClkSel::Pll)
    }
    #[doc = "Sysclk"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(QspiClkSel::Sysclk)
    }
}
#[doc = "I2s clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2sClkSel {
    #[doc = "4: Ext clk"]
    ExtClk = 4,
    #[doc = "0: Pclk0"]
    Pclk0 = 0,
    #[doc = "2: Pll"]
    Pll = 2,
    #[doc = "1: Xo24m"]
    Xo24m = 1,
    #[doc = "3: Xo32m"]
    Xo32m = 3,
}
impl From<I2sClkSel> for u8 {
    #[inline(always)]
    fn from(variant: I2sClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2sClkSel {
    type Ux = u8;
}
impl crate::IsEnum for I2sClkSel {}
#[doc = "Field `I2S_CLK_SEL` reader - I2s clk sel"]
pub type I2sClkSelR = crate::FieldReader<I2sClkSel>;
impl I2sClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2sClkSel> {
        match self.bits {
            4 => Some(I2sClkSel::ExtClk),
            0 => Some(I2sClkSel::Pclk0),
            2 => Some(I2sClkSel::Pll),
            1 => Some(I2sClkSel::Xo24m),
            3 => Some(I2sClkSel::Xo32m),
            _ => None,
        }
    }
    #[doc = "Ext clk"]
    #[inline(always)]
    pub fn is_ext_clk(&self) -> bool {
        *self == I2sClkSel::ExtClk
    }
    #[doc = "Pclk0"]
    #[inline(always)]
    pub fn is_pclk0(&self) -> bool {
        *self == I2sClkSel::Pclk0
    }
    #[doc = "Pll"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == I2sClkSel::Pll
    }
    #[doc = "Xo24m"]
    #[inline(always)]
    pub fn is_xo24m(&self) -> bool {
        *self == I2sClkSel::Xo24m
    }
    #[doc = "Xo32m"]
    #[inline(always)]
    pub fn is_xo32m(&self) -> bool {
        *self == I2sClkSel::Xo32m
    }
}
#[doc = "Field `I2S_CLK_SEL` writer - I2s clk sel"]
pub type I2sClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, I2sClkSel>;
impl<'a, REG> I2sClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ext clk"]
    #[inline(always)]
    pub fn ext_clk(self) -> &'a mut crate::W<REG> {
        self.variant(I2sClkSel::ExtClk)
    }
    #[doc = "Pclk0"]
    #[inline(always)]
    pub fn pclk0(self) -> &'a mut crate::W<REG> {
        self.variant(I2sClkSel::Pclk0)
    }
    #[doc = "Pll"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(I2sClkSel::Pll)
    }
    #[doc = "Xo24m"]
    #[inline(always)]
    pub fn xo24m(self) -> &'a mut crate::W<REG> {
        self.variant(I2sClkSel::Xo24m)
    }
    #[doc = "Xo32m"]
    #[inline(always)]
    pub fn xo32m(self) -> &'a mut crate::W<REG> {
        self.variant(I2sClkSel::Xo32m)
    }
}
#[doc = "Adc clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcClkSel {
    #[doc = "0: Pclk1"]
    Pclk1 = 0,
    #[doc = "2: Pll"]
    Pll = 2,
    #[doc = "3: Rco48m"]
    Rco48m = 3,
    #[doc = "1: Sysclk"]
    Sysclk = 1,
}
impl From<AdcClkSel> for u8 {
    #[inline(always)]
    fn from(variant: AdcClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AdcClkSel {
    type Ux = u8;
}
impl crate::IsEnum for AdcClkSel {}
#[doc = "Field `ADC_CLK_SEL` reader - Adc clk sel"]
pub type AdcClkSelR = crate::FieldReader<AdcClkSel>;
impl AdcClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcClkSel {
        match self.bits {
            0 => AdcClkSel::Pclk1,
            2 => AdcClkSel::Pll,
            3 => AdcClkSel::Rco48m,
            1 => AdcClkSel::Sysclk,
            _ => unreachable!(),
        }
    }
    #[doc = "Pclk1"]
    #[inline(always)]
    pub fn is_pclk1(&self) -> bool {
        *self == AdcClkSel::Pclk1
    }
    #[doc = "Pll"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == AdcClkSel::Pll
    }
    #[doc = "Rco48m"]
    #[inline(always)]
    pub fn is_rco48m(&self) -> bool {
        *self == AdcClkSel::Rco48m
    }
    #[doc = "Sysclk"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == AdcClkSel::Sysclk
    }
}
#[doc = "Field `ADC_CLK_SEL` writer - Adc clk sel"]
pub type AdcClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, AdcClkSel, crate::Safe>;
impl<'a, REG> AdcClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pclk1"]
    #[inline(always)]
    pub fn pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(AdcClkSel::Pclk1)
    }
    #[doc = "Pll"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(AdcClkSel::Pll)
    }
    #[doc = "Rco48m"]
    #[inline(always)]
    pub fn rco48m(self) -> &'a mut crate::W<REG> {
        self.variant(AdcClkSel::Rco48m)
    }
    #[doc = "Sysclk"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(AdcClkSel::Sysclk)
    }
}
#[doc = "Uart3 clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart3ClkSel {
    #[doc = "0: Pclk1"]
    Pclk1 = 0,
    #[doc = "1: Rco4m"]
    Rco4m = 1,
    #[doc = "3: Xo24m"]
    Xo24m = 3,
    #[doc = "2: Xo32k"]
    Xo32k = 2,
}
impl From<Uart3ClkSel> for u8 {
    #[inline(always)]
    fn from(variant: Uart3ClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart3ClkSel {
    type Ux = u8;
}
impl crate::IsEnum for Uart3ClkSel {}
#[doc = "Field `UART3_CLK_SEL` reader - Uart3 clk sel"]
pub type Uart3ClkSelR = crate::FieldReader<Uart3ClkSel>;
impl Uart3ClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart3ClkSel {
        match self.bits {
            0 => Uart3ClkSel::Pclk1,
            1 => Uart3ClkSel::Rco4m,
            3 => Uart3ClkSel::Xo24m,
            2 => Uart3ClkSel::Xo32k,
            _ => unreachable!(),
        }
    }
    #[doc = "Pclk1"]
    #[inline(always)]
    pub fn is_pclk1(&self) -> bool {
        *self == Uart3ClkSel::Pclk1
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn is_rco4m(&self) -> bool {
        *self == Uart3ClkSel::Rco4m
    }
    #[doc = "Xo24m"]
    #[inline(always)]
    pub fn is_xo24m(&self) -> bool {
        *self == Uart3ClkSel::Xo24m
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == Uart3ClkSel::Xo32k
    }
}
#[doc = "Field `UART3_CLK_SEL` writer - Uart3 clk sel"]
pub type Uart3ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart3ClkSel, crate::Safe>;
impl<'a, REG> Uart3ClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pclk1"]
    #[inline(always)]
    pub fn pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart3ClkSel::Pclk1)
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn rco4m(self) -> &'a mut crate::W<REG> {
        self.variant(Uart3ClkSel::Rco4m)
    }
    #[doc = "Xo24m"]
    #[inline(always)]
    pub fn xo24m(self) -> &'a mut crate::W<REG> {
        self.variant(Uart3ClkSel::Xo24m)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(Uart3ClkSel::Xo32k)
    }
}
#[doc = "Uart2 clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart2ClkSel {
    #[doc = "0: Pclk1"]
    Pclk1 = 0,
    #[doc = "1: Rco4m"]
    Rco4m = 1,
    #[doc = "3: Xo24m"]
    Xo24m = 3,
    #[doc = "2: Xo32k"]
    Xo32k = 2,
}
impl From<Uart2ClkSel> for u8 {
    #[inline(always)]
    fn from(variant: Uart2ClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart2ClkSel {
    type Ux = u8;
}
impl crate::IsEnum for Uart2ClkSel {}
#[doc = "Field `UART2_CLK_SEL` reader - Uart2 clk sel"]
pub type Uart2ClkSelR = crate::FieldReader<Uart2ClkSel>;
impl Uart2ClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart2ClkSel {
        match self.bits {
            0 => Uart2ClkSel::Pclk1,
            1 => Uart2ClkSel::Rco4m,
            3 => Uart2ClkSel::Xo24m,
            2 => Uart2ClkSel::Xo32k,
            _ => unreachable!(),
        }
    }
    #[doc = "Pclk1"]
    #[inline(always)]
    pub fn is_pclk1(&self) -> bool {
        *self == Uart2ClkSel::Pclk1
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn is_rco4m(&self) -> bool {
        *self == Uart2ClkSel::Rco4m
    }
    #[doc = "Xo24m"]
    #[inline(always)]
    pub fn is_xo24m(&self) -> bool {
        *self == Uart2ClkSel::Xo24m
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == Uart2ClkSel::Xo32k
    }
}
#[doc = "Field `UART2_CLK_SEL` writer - Uart2 clk sel"]
pub type Uart2ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart2ClkSel, crate::Safe>;
impl<'a, REG> Uart2ClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pclk1"]
    #[inline(always)]
    pub fn pclk1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart2ClkSel::Pclk1)
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn rco4m(self) -> &'a mut crate::W<REG> {
        self.variant(Uart2ClkSel::Rco4m)
    }
    #[doc = "Xo24m"]
    #[inline(always)]
    pub fn xo24m(self) -> &'a mut crate::W<REG> {
        self.variant(Uart2ClkSel::Xo24m)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(Uart2ClkSel::Xo32k)
    }
}
#[doc = "Uart1 clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart1ClkSel {
    #[doc = "0: Pclk0"]
    Pclk0 = 0,
    #[doc = "1: Rco4m"]
    Rco4m = 1,
    #[doc = "3: Xo24m"]
    Xo24m = 3,
    #[doc = "2: Xo32k"]
    Xo32k = 2,
}
impl From<Uart1ClkSel> for u8 {
    #[inline(always)]
    fn from(variant: Uart1ClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart1ClkSel {
    type Ux = u8;
}
impl crate::IsEnum for Uart1ClkSel {}
#[doc = "Field `UART1_CLK_SEL` reader - Uart1 clk sel"]
pub type Uart1ClkSelR = crate::FieldReader<Uart1ClkSel>;
impl Uart1ClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1ClkSel {
        match self.bits {
            0 => Uart1ClkSel::Pclk0,
            1 => Uart1ClkSel::Rco4m,
            3 => Uart1ClkSel::Xo24m,
            2 => Uart1ClkSel::Xo32k,
            _ => unreachable!(),
        }
    }
    #[doc = "Pclk0"]
    #[inline(always)]
    pub fn is_pclk0(&self) -> bool {
        *self == Uart1ClkSel::Pclk0
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn is_rco4m(&self) -> bool {
        *self == Uart1ClkSel::Rco4m
    }
    #[doc = "Xo24m"]
    #[inline(always)]
    pub fn is_xo24m(&self) -> bool {
        *self == Uart1ClkSel::Xo24m
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == Uart1ClkSel::Xo32k
    }
}
#[doc = "Field `UART1_CLK_SEL` writer - Uart1 clk sel"]
pub type Uart1ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart1ClkSel, crate::Safe>;
impl<'a, REG> Uart1ClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pclk0"]
    #[inline(always)]
    pub fn pclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1ClkSel::Pclk0)
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn rco4m(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1ClkSel::Rco4m)
    }
    #[doc = "Xo24m"]
    #[inline(always)]
    pub fn xo24m(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1ClkSel::Xo24m)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1ClkSel::Xo32k)
    }
}
#[doc = "Uart0 clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart0ClkSel {
    #[doc = "0: Pclk0"]
    Pclk0 = 0,
    #[doc = "1: Rco4m"]
    Rco4m = 1,
    #[doc = "3: Xo24m"]
    Xo24m = 3,
    #[doc = "2: Xo32k"]
    Xo32k = 2,
}
impl From<Uart0ClkSel> for u8 {
    #[inline(always)]
    fn from(variant: Uart0ClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart0ClkSel {
    type Ux = u8;
}
impl crate::IsEnum for Uart0ClkSel {}
#[doc = "Field `UART0_CLK_SEL` reader - Uart0 clk sel"]
pub type Uart0ClkSelR = crate::FieldReader<Uart0ClkSel>;
impl Uart0ClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart0ClkSel {
        match self.bits {
            0 => Uart0ClkSel::Pclk0,
            1 => Uart0ClkSel::Rco4m,
            3 => Uart0ClkSel::Xo24m,
            2 => Uart0ClkSel::Xo32k,
            _ => unreachable!(),
        }
    }
    #[doc = "Pclk0"]
    #[inline(always)]
    pub fn is_pclk0(&self) -> bool {
        *self == Uart0ClkSel::Pclk0
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn is_rco4m(&self) -> bool {
        *self == Uart0ClkSel::Rco4m
    }
    #[doc = "Xo24m"]
    #[inline(always)]
    pub fn is_xo24m(&self) -> bool {
        *self == Uart0ClkSel::Xo24m
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == Uart0ClkSel::Xo32k
    }
}
#[doc = "Field `UART0_CLK_SEL` writer - Uart0 clk sel"]
pub type Uart0ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart0ClkSel, crate::Safe>;
impl<'a, REG> Uart0ClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pclk0"]
    #[inline(always)]
    pub fn pclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0ClkSel::Pclk0)
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn rco4m(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0ClkSel::Rco4m)
    }
    #[doc = "Xo24m"]
    #[inline(always)]
    pub fn xo24m(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0ClkSel::Xo24m)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0ClkSel::Xo32k)
    }
}
impl R {
    #[doc = "Bits 0:1 - Qspi clk sel"]
    #[inline(always)]
    pub fn qspi_clk_sel(&self) -> QspiClkSelR {
        QspiClkSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - I2s clk sel"]
    #[inline(always)]
    pub fn i2s_clk_sel(&self) -> I2sClkSelR {
        I2sClkSelR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - Adc clk sel"]
    #[inline(always)]
    pub fn adc_clk_sel(&self) -> AdcClkSelR {
        AdcClkSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Uart3 clk sel"]
    #[inline(always)]
    pub fn uart3_clk_sel(&self) -> Uart3ClkSelR {
        Uart3ClkSelR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Uart2 clk sel"]
    #[inline(always)]
    pub fn uart2_clk_sel(&self) -> Uart2ClkSelR {
        Uart2ClkSelR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Uart1 clk sel"]
    #[inline(always)]
    pub fn uart1_clk_sel(&self) -> Uart1ClkSelR {
        Uart1ClkSelR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:16 - Uart0 clk sel"]
    #[inline(always)]
    pub fn uart0_clk_sel(&self) -> Uart0ClkSelR {
        Uart0ClkSelR::new(((self.bits >> 15) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Qspi clk sel"]
    #[inline(always)]
    pub fn qspi_clk_sel(&mut self) -> QspiClkSelW<'_, Cr2Spec> {
        QspiClkSelW::new(self, 0)
    }
    #[doc = "Bits 2:4 - I2s clk sel"]
    #[inline(always)]
    pub fn i2s_clk_sel(&mut self) -> I2sClkSelW<'_, Cr2Spec> {
        I2sClkSelW::new(self, 2)
    }
    #[doc = "Bits 5:6 - Adc clk sel"]
    #[inline(always)]
    pub fn adc_clk_sel(&mut self) -> AdcClkSelW<'_, Cr2Spec> {
        AdcClkSelW::new(self, 5)
    }
    #[doc = "Bits 9:10 - Uart3 clk sel"]
    #[inline(always)]
    pub fn uart3_clk_sel(&mut self) -> Uart3ClkSelW<'_, Cr2Spec> {
        Uart3ClkSelW::new(self, 9)
    }
    #[doc = "Bits 11:12 - Uart2 clk sel"]
    #[inline(always)]
    pub fn uart2_clk_sel(&mut self) -> Uart2ClkSelW<'_, Cr2Spec> {
        Uart2ClkSelW::new(self, 11)
    }
    #[doc = "Bits 13:14 - Uart1 clk sel"]
    #[inline(always)]
    pub fn uart1_clk_sel(&mut self) -> Uart1ClkSelW<'_, Cr2Spec> {
        Uart1ClkSelW::new(self, 13)
    }
    #[doc = "Bits 15:16 - Uart0 clk sel"]
    #[inline(always)]
    pub fn uart0_clk_sel(&mut self) -> Uart0ClkSelW<'_, Cr2Spec> {
        Uart0ClkSelW::new(self, 15)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}

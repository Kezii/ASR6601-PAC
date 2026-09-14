#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Pclk0 div"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pclk0Div {
    #[doc = "0: Value 1"]
    Value1 = 0,
    #[doc = "4: Value 16"]
    Value16 = 4,
    #[doc = "1: Value 2"]
    Value2 = 1,
    #[doc = "2: Value 4"]
    Value4 = 2,
    #[doc = "3: Value 8"]
    Value8 = 3,
}
impl From<Pclk0Div> for u8 {
    #[inline(always)]
    fn from(variant: Pclk0Div) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pclk0Div {
    type Ux = u8;
}
impl crate::IsEnum for Pclk0Div {}
#[doc = "Field `PCLK0_DIV` reader - Pclk0 div"]
pub type Pclk0DivR = crate::FieldReader<Pclk0Div>;
impl Pclk0DivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pclk0Div> {
        match self.bits {
            0 => Some(Pclk0Div::Value1),
            4 => Some(Pclk0Div::Value16),
            1 => Some(Pclk0Div::Value2),
            2 => Some(Pclk0Div::Value4),
            3 => Some(Pclk0Div::Value8),
            _ => None,
        }
    }
    #[doc = "Value 1"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Pclk0Div::Value1
    }
    #[doc = "Value 16"]
    #[inline(always)]
    pub fn is_value_16(&self) -> bool {
        *self == Pclk0Div::Value16
    }
    #[doc = "Value 2"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == Pclk0Div::Value2
    }
    #[doc = "Value 4"]
    #[inline(always)]
    pub fn is_value_4(&self) -> bool {
        *self == Pclk0Div::Value4
    }
    #[doc = "Value 8"]
    #[inline(always)]
    pub fn is_value_8(&self) -> bool {
        *self == Pclk0Div::Value8
    }
}
#[doc = "Field `PCLK0_DIV` writer - Pclk0 div"]
pub type Pclk0DivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pclk0Div>;
impl<'a, REG> Pclk0DivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value 1"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pclk0Div::Value1)
    }
    #[doc = "Value 16"]
    #[inline(always)]
    pub fn value_16(self) -> &'a mut crate::W<REG> {
        self.variant(Pclk0Div::Value16)
    }
    #[doc = "Value 2"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(Pclk0Div::Value2)
    }
    #[doc = "Value 4"]
    #[inline(always)]
    pub fn value_4(self) -> &'a mut crate::W<REG> {
        self.variant(Pclk0Div::Value4)
    }
    #[doc = "Value 8"]
    #[inline(always)]
    pub fn value_8(self) -> &'a mut crate::W<REG> {
        self.variant(Pclk0Div::Value8)
    }
}
#[doc = "Hclk div"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HclkDiv {
    #[doc = "0: Value 1"]
    Value1 = 0,
    #[doc = "7: Value 128"]
    Value128 = 7,
    #[doc = "4: Value 16"]
    Value16 = 4,
    #[doc = "1: Value 2"]
    Value2 = 1,
    #[doc = "8: Value 256"]
    Value256 = 8,
    #[doc = "5: Value 32"]
    Value32 = 5,
    #[doc = "2: Value 4"]
    Value4 = 2,
    #[doc = "9: Value 512"]
    Value512 = 9,
    #[doc = "6: Value 64"]
    Value64 = 6,
    #[doc = "3: Value 8"]
    Value8 = 3,
}
impl From<HclkDiv> for u8 {
    #[inline(always)]
    fn from(variant: HclkDiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HclkDiv {
    type Ux = u8;
}
impl crate::IsEnum for HclkDiv {}
#[doc = "Field `HCLK_DIV` reader - Hclk div"]
pub type HclkDivR = crate::FieldReader<HclkDiv>;
impl HclkDivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HclkDiv> {
        match self.bits {
            0 => Some(HclkDiv::Value1),
            7 => Some(HclkDiv::Value128),
            4 => Some(HclkDiv::Value16),
            1 => Some(HclkDiv::Value2),
            8 => Some(HclkDiv::Value256),
            5 => Some(HclkDiv::Value32),
            2 => Some(HclkDiv::Value4),
            9 => Some(HclkDiv::Value512),
            6 => Some(HclkDiv::Value64),
            3 => Some(HclkDiv::Value8),
            _ => None,
        }
    }
    #[doc = "Value 1"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == HclkDiv::Value1
    }
    #[doc = "Value 128"]
    #[inline(always)]
    pub fn is_value_128(&self) -> bool {
        *self == HclkDiv::Value128
    }
    #[doc = "Value 16"]
    #[inline(always)]
    pub fn is_value_16(&self) -> bool {
        *self == HclkDiv::Value16
    }
    #[doc = "Value 2"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == HclkDiv::Value2
    }
    #[doc = "Value 256"]
    #[inline(always)]
    pub fn is_value_256(&self) -> bool {
        *self == HclkDiv::Value256
    }
    #[doc = "Value 32"]
    #[inline(always)]
    pub fn is_value_32(&self) -> bool {
        *self == HclkDiv::Value32
    }
    #[doc = "Value 4"]
    #[inline(always)]
    pub fn is_value_4(&self) -> bool {
        *self == HclkDiv::Value4
    }
    #[doc = "Value 512"]
    #[inline(always)]
    pub fn is_value_512(&self) -> bool {
        *self == HclkDiv::Value512
    }
    #[doc = "Value 64"]
    #[inline(always)]
    pub fn is_value_64(&self) -> bool {
        *self == HclkDiv::Value64
    }
    #[doc = "Value 8"]
    #[inline(always)]
    pub fn is_value_8(&self) -> bool {
        *self == HclkDiv::Value8
    }
}
#[doc = "Field `HCLK_DIV` writer - Hclk div"]
pub type HclkDivW<'a, REG> = crate::FieldWriter<'a, REG, 4, HclkDiv>;
impl<'a, REG> HclkDivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value 1"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(HclkDiv::Value1)
    }
    #[doc = "Value 128"]
    #[inline(always)]
    pub fn value_128(self) -> &'a mut crate::W<REG> {
        self.variant(HclkDiv::Value128)
    }
    #[doc = "Value 16"]
    #[inline(always)]
    pub fn value_16(self) -> &'a mut crate::W<REG> {
        self.variant(HclkDiv::Value16)
    }
    #[doc = "Value 2"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(HclkDiv::Value2)
    }
    #[doc = "Value 256"]
    #[inline(always)]
    pub fn value_256(self) -> &'a mut crate::W<REG> {
        self.variant(HclkDiv::Value256)
    }
    #[doc = "Value 32"]
    #[inline(always)]
    pub fn value_32(self) -> &'a mut crate::W<REG> {
        self.variant(HclkDiv::Value32)
    }
    #[doc = "Value 4"]
    #[inline(always)]
    pub fn value_4(self) -> &'a mut crate::W<REG> {
        self.variant(HclkDiv::Value4)
    }
    #[doc = "Value 512"]
    #[inline(always)]
    pub fn value_512(self) -> &'a mut crate::W<REG> {
        self.variant(HclkDiv::Value512)
    }
    #[doc = "Value 64"]
    #[inline(always)]
    pub fn value_64(self) -> &'a mut crate::W<REG> {
        self.variant(HclkDiv::Value64)
    }
    #[doc = "Value 8"]
    #[inline(always)]
    pub fn value_8(self) -> &'a mut crate::W<REG> {
        self.variant(HclkDiv::Value8)
    }
}
#[doc = "Sysclk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysclkSel {
    #[doc = "3: Pll"]
    Pll = 3,
    #[doc = "1: Rco32k"]
    Rco32k = 1,
    #[doc = "7: Rco48m"]
    Rco48m = 7,
    #[doc = "0: Rco48m div2"]
    Rco48mDiv2 = 0,
    #[doc = "6: Rco4m"]
    Rco4m = 6,
    #[doc = "4: Xo24m"]
    Xo24m = 4,
    #[doc = "2: Xo32k"]
    Xo32k = 2,
    #[doc = "5: Xo32m"]
    Xo32m = 5,
}
impl From<SysclkSel> for u8 {
    #[inline(always)]
    fn from(variant: SysclkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysclkSel {
    type Ux = u8;
}
impl crate::IsEnum for SysclkSel {}
#[doc = "Field `SYSCLK_SEL` reader - Sysclk sel"]
pub type SysclkSelR = crate::FieldReader<SysclkSel>;
impl SysclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysclkSel {
        match self.bits {
            3 => SysclkSel::Pll,
            1 => SysclkSel::Rco32k,
            7 => SysclkSel::Rco48m,
            0 => SysclkSel::Rco48mDiv2,
            6 => SysclkSel::Rco4m,
            4 => SysclkSel::Xo24m,
            2 => SysclkSel::Xo32k,
            5 => SysclkSel::Xo32m,
            _ => unreachable!(),
        }
    }
    #[doc = "Pll"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SysclkSel::Pll
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn is_rco32k(&self) -> bool {
        *self == SysclkSel::Rco32k
    }
    #[doc = "Rco48m"]
    #[inline(always)]
    pub fn is_rco48m(&self) -> bool {
        *self == SysclkSel::Rco48m
    }
    #[doc = "Rco48m div2"]
    #[inline(always)]
    pub fn is_rco48m_div2(&self) -> bool {
        *self == SysclkSel::Rco48mDiv2
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn is_rco4m(&self) -> bool {
        *self == SysclkSel::Rco4m
    }
    #[doc = "Xo24m"]
    #[inline(always)]
    pub fn is_xo24m(&self) -> bool {
        *self == SysclkSel::Xo24m
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == SysclkSel::Xo32k
    }
    #[doc = "Xo32m"]
    #[inline(always)]
    pub fn is_xo32m(&self) -> bool {
        *self == SysclkSel::Xo32m
    }
}
#[doc = "Field `SYSCLK_SEL` writer - Sysclk sel"]
pub type SysclkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, SysclkSel, crate::Safe>;
impl<'a, REG> SysclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pll"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Pll)
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn rco32k(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Rco32k)
    }
    #[doc = "Rco48m"]
    #[inline(always)]
    pub fn rco48m(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Rco48m)
    }
    #[doc = "Rco48m div2"]
    #[inline(always)]
    pub fn rco48m_div2(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Rco48mDiv2)
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn rco4m(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Rco4m)
    }
    #[doc = "Xo24m"]
    #[inline(always)]
    pub fn xo24m(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Xo24m)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Xo32k)
    }
    #[doc = "Xo32m"]
    #[inline(always)]
    pub fn xo32m(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Xo32m)
    }
}
#[doc = "Pclk1 div"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pclk1Div {
    #[doc = "0: Value 1"]
    Value1 = 0,
    #[doc = "4: Value 16"]
    Value16 = 4,
    #[doc = "1: Value 2"]
    Value2 = 1,
    #[doc = "2: Value 4"]
    Value4 = 2,
    #[doc = "3: Value 8"]
    Value8 = 3,
}
impl From<Pclk1Div> for u8 {
    #[inline(always)]
    fn from(variant: Pclk1Div) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pclk1Div {
    type Ux = u8;
}
impl crate::IsEnum for Pclk1Div {}
#[doc = "Field `PCLK1_DIV` reader - Pclk1 div"]
pub type Pclk1DivR = crate::FieldReader<Pclk1Div>;
impl Pclk1DivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pclk1Div> {
        match self.bits {
            0 => Some(Pclk1Div::Value1),
            4 => Some(Pclk1Div::Value16),
            1 => Some(Pclk1Div::Value2),
            2 => Some(Pclk1Div::Value4),
            3 => Some(Pclk1Div::Value8),
            _ => None,
        }
    }
    #[doc = "Value 1"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Pclk1Div::Value1
    }
    #[doc = "Value 16"]
    #[inline(always)]
    pub fn is_value_16(&self) -> bool {
        *self == Pclk1Div::Value16
    }
    #[doc = "Value 2"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == Pclk1Div::Value2
    }
    #[doc = "Value 4"]
    #[inline(always)]
    pub fn is_value_4(&self) -> bool {
        *self == Pclk1Div::Value4
    }
    #[doc = "Value 8"]
    #[inline(always)]
    pub fn is_value_8(&self) -> bool {
        *self == Pclk1Div::Value8
    }
}
#[doc = "Field `PCLK1_DIV` writer - Pclk1 div"]
pub type Pclk1DivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pclk1Div>;
impl<'a, REG> Pclk1DivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value 1"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pclk1Div::Value1)
    }
    #[doc = "Value 16"]
    #[inline(always)]
    pub fn value_16(self) -> &'a mut crate::W<REG> {
        self.variant(Pclk1Div::Value16)
    }
    #[doc = "Value 2"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(Pclk1Div::Value2)
    }
    #[doc = "Value 4"]
    #[inline(always)]
    pub fn value_4(self) -> &'a mut crate::W<REG> {
        self.variant(Pclk1Div::Value4)
    }
    #[doc = "Value 8"]
    #[inline(always)]
    pub fn value_8(self) -> &'a mut crate::W<REG> {
        self.variant(Pclk1Div::Value8)
    }
}
#[doc = "Field `MCO_CLK_OUT_EN` reader - Mco clk out en"]
pub type McoClkOutEnR = crate::BitReader;
#[doc = "Field `MCO_CLK_OUT_EN` writer - Mco clk out en"]
pub type McoClkOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Mco clk sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum McoClkSel {
    #[doc = "6: Pll"]
    Pll = 6,
    #[doc = "0: Rco32k"]
    Rco32k = 0,
    #[doc = "5: Rco48m"]
    Rco48m = 5,
    #[doc = "2: Rco4m"]
    Rco4m = 2,
    #[doc = "7: Sysclk"]
    Sysclk = 7,
    #[doc = "3: Xo24m"]
    Xo24m = 3,
    #[doc = "1: Xo32k"]
    Xo32k = 1,
    #[doc = "4: Xo32m"]
    Xo32m = 4,
}
impl From<McoClkSel> for u8 {
    #[inline(always)]
    fn from(variant: McoClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for McoClkSel {
    type Ux = u8;
}
impl crate::IsEnum for McoClkSel {}
#[doc = "Field `MCO_CLK_SEL` reader - Mco clk sel"]
pub type McoClkSelR = crate::FieldReader<McoClkSel>;
impl McoClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McoClkSel {
        match self.bits {
            6 => McoClkSel::Pll,
            0 => McoClkSel::Rco32k,
            5 => McoClkSel::Rco48m,
            2 => McoClkSel::Rco4m,
            7 => McoClkSel::Sysclk,
            3 => McoClkSel::Xo24m,
            1 => McoClkSel::Xo32k,
            4 => McoClkSel::Xo32m,
            _ => unreachable!(),
        }
    }
    #[doc = "Pll"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == McoClkSel::Pll
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn is_rco32k(&self) -> bool {
        *self == McoClkSel::Rco32k
    }
    #[doc = "Rco48m"]
    #[inline(always)]
    pub fn is_rco48m(&self) -> bool {
        *self == McoClkSel::Rco48m
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn is_rco4m(&self) -> bool {
        *self == McoClkSel::Rco4m
    }
    #[doc = "Sysclk"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == McoClkSel::Sysclk
    }
    #[doc = "Xo24m"]
    #[inline(always)]
    pub fn is_xo24m(&self) -> bool {
        *self == McoClkSel::Xo24m
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == McoClkSel::Xo32k
    }
    #[doc = "Xo32m"]
    #[inline(always)]
    pub fn is_xo32m(&self) -> bool {
        *self == McoClkSel::Xo32m
    }
}
#[doc = "Field `MCO_CLK_SEL` writer - Mco clk sel"]
pub type McoClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, McoClkSel, crate::Safe>;
impl<'a, REG> McoClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pll"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(McoClkSel::Pll)
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn rco32k(self) -> &'a mut crate::W<REG> {
        self.variant(McoClkSel::Rco32k)
    }
    #[doc = "Rco48m"]
    #[inline(always)]
    pub fn rco48m(self) -> &'a mut crate::W<REG> {
        self.variant(McoClkSel::Rco48m)
    }
    #[doc = "Rco4m"]
    #[inline(always)]
    pub fn rco4m(self) -> &'a mut crate::W<REG> {
        self.variant(McoClkSel::Rco4m)
    }
    #[doc = "Sysclk"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(McoClkSel::Sysclk)
    }
    #[doc = "Xo24m"]
    #[inline(always)]
    pub fn xo24m(self) -> &'a mut crate::W<REG> {
        self.variant(McoClkSel::Xo24m)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(McoClkSel::Xo32k)
    }
    #[doc = "Xo32m"]
    #[inline(always)]
    pub fn xo32m(self) -> &'a mut crate::W<REG> {
        self.variant(McoClkSel::Xo32m)
    }
}
#[doc = "Mco clk div"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum McoClkDiv {
    #[doc = "0: Value 1"]
    Value1 = 0,
    #[doc = "7: Value 16"]
    Value16 = 7,
    #[doc = "4: Value 2"]
    Value2 = 4,
    #[doc = "5: Value 4"]
    Value4 = 5,
    #[doc = "6: Value 8"]
    Value8 = 6,
}
impl From<McoClkDiv> for u8 {
    #[inline(always)]
    fn from(variant: McoClkDiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for McoClkDiv {
    type Ux = u8;
}
impl crate::IsEnum for McoClkDiv {}
#[doc = "Field `MCO_CLK_DIV` reader - Mco clk div"]
pub type McoClkDivR = crate::FieldReader<McoClkDiv>;
impl McoClkDivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<McoClkDiv> {
        match self.bits {
            0 => Some(McoClkDiv::Value1),
            7 => Some(McoClkDiv::Value16),
            4 => Some(McoClkDiv::Value2),
            5 => Some(McoClkDiv::Value4),
            6 => Some(McoClkDiv::Value8),
            _ => None,
        }
    }
    #[doc = "Value 1"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == McoClkDiv::Value1
    }
    #[doc = "Value 16"]
    #[inline(always)]
    pub fn is_value_16(&self) -> bool {
        *self == McoClkDiv::Value16
    }
    #[doc = "Value 2"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == McoClkDiv::Value2
    }
    #[doc = "Value 4"]
    #[inline(always)]
    pub fn is_value_4(&self) -> bool {
        *self == McoClkDiv::Value4
    }
    #[doc = "Value 8"]
    #[inline(always)]
    pub fn is_value_8(&self) -> bool {
        *self == McoClkDiv::Value8
    }
}
#[doc = "Field `MCO_CLK_DIV` writer - Mco clk div"]
pub type McoClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 3, McoClkDiv>;
impl<'a, REG> McoClkDivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value 1"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(McoClkDiv::Value1)
    }
    #[doc = "Value 16"]
    #[inline(always)]
    pub fn value_16(self) -> &'a mut crate::W<REG> {
        self.variant(McoClkDiv::Value16)
    }
    #[doc = "Value 2"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(McoClkDiv::Value2)
    }
    #[doc = "Value 4"]
    #[inline(always)]
    pub fn value_4(self) -> &'a mut crate::W<REG> {
        self.variant(McoClkDiv::Value4)
    }
    #[doc = "Value 8"]
    #[inline(always)]
    pub fn value_8(self) -> &'a mut crate::W<REG> {
        self.variant(McoClkDiv::Value8)
    }
}
#[doc = "Stclken sel"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StclkenSel {
    #[doc = "1: Rco32k"]
    Rco32k = 1,
    #[doc = "0: Xo32k"]
    Xo32k = 0,
}
impl From<StclkenSel> for bool {
    #[inline(always)]
    fn from(variant: StclkenSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STCLKEN_SEL` reader - Stclken sel"]
pub type StclkenSelR = crate::BitReader<StclkenSel>;
impl StclkenSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StclkenSel {
        match self.bits {
            true => StclkenSel::Rco32k,
            false => StclkenSel::Xo32k,
        }
    }
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn is_rco32k(&self) -> bool {
        *self == StclkenSel::Rco32k
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == StclkenSel::Xo32k
    }
}
#[doc = "Field `STCLKEN_SEL` writer - Stclken sel"]
pub type StclkenSelW<'a, REG> = crate::BitWriter<'a, REG, StclkenSel>;
impl<'a, REG> StclkenSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rco32k"]
    #[inline(always)]
    pub fn rco32k(self) -> &'a mut crate::W<REG> {
        self.variant(StclkenSel::Rco32k)
    }
    #[doc = "Xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(StclkenSel::Xo32k)
    }
}
impl R {
    #[doc = "Bits 5:7 - Pclk0 div"]
    #[inline(always)]
    pub fn pclk0_div(&self) -> Pclk0DivR {
        Pclk0DivR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Hclk div"]
    #[inline(always)]
    pub fn hclk_div(&self) -> HclkDivR {
        HclkDivR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Sysclk sel"]
    #[inline(always)]
    pub fn sysclk_sel(&self) -> SysclkSelR {
        SysclkSelR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Pclk1 div"]
    #[inline(always)]
    pub fn pclk1_div(&self) -> Pclk1DivR {
        Pclk1DivR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bit 18 - Mco clk out en"]
    #[inline(always)]
    pub fn mco_clk_out_en(&self) -> McoClkOutEnR {
        McoClkOutEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - Mco clk sel"]
    #[inline(always)]
    pub fn mco_clk_sel(&self) -> McoClkSelR {
        McoClkSelR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - Mco clk div"]
    #[inline(always)]
    pub fn mco_clk_div(&self) -> McoClkDivR {
        McoClkDivR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - Stclken sel"]
    #[inline(always)]
    pub fn stclken_sel(&self) -> StclkenSelR {
        StclkenSelR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 5:7 - Pclk0 div"]
    #[inline(always)]
    pub fn pclk0_div(&mut self) -> Pclk0DivW<'_, Cr0Spec> {
        Pclk0DivW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Hclk div"]
    #[inline(always)]
    pub fn hclk_div(&mut self) -> HclkDivW<'_, Cr0Spec> {
        HclkDivW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Sysclk sel"]
    #[inline(always)]
    pub fn sysclk_sel(&mut self) -> SysclkSelW<'_, Cr0Spec> {
        SysclkSelW::new(self, 12)
    }
    #[doc = "Bits 15:17 - Pclk1 div"]
    #[inline(always)]
    pub fn pclk1_div(&mut self) -> Pclk1DivW<'_, Cr0Spec> {
        Pclk1DivW::new(self, 15)
    }
    #[doc = "Bit 18 - Mco clk out en"]
    #[inline(always)]
    pub fn mco_clk_out_en(&mut self) -> McoClkOutEnW<'_, Cr0Spec> {
        McoClkOutEnW::new(self, 18)
    }
    #[doc = "Bits 19:21 - Mco clk sel"]
    #[inline(always)]
    pub fn mco_clk_sel(&mut self) -> McoClkSelW<'_, Cr0Spec> {
        McoClkSelW::new(self, 19)
    }
    #[doc = "Bits 22:24 - Mco clk div"]
    #[inline(always)]
    pub fn mco_clk_div(&mut self) -> McoClkDivW<'_, Cr0Spec> {
        McoClkDivW::new(self, 22)
    }
    #[doc = "Bit 25 - Stclken sel"]
    #[inline(always)]
    pub fn stclken_sel(&mut self) -> StclkenSelW<'_, Cr0Spec> {
        StclkenSelW::new(self, 25)
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

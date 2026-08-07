#[doc = "Register `IFLS` reader"]
pub type R = crate::R<IflsSpec>;
#[doc = "Register `IFLS` writer"]
pub type W = crate::W<IflsSpec>;
#[doc = "Tx"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tx {
    #[doc = "2: Value 1 2"]
    Value1_2 = 2,
    #[doc = "1: Value 1 4"]
    Value1_4 = 1,
    #[doc = "0: Value 1 8"]
    Value1_8 = 0,
    #[doc = "3: Value 3 4"]
    Value3_4 = 3,
    #[doc = "4: Value 7 8"]
    Value7_8 = 4,
}
impl From<Tx> for u8 {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tx {
    type Ux = u8;
}
impl crate::IsEnum for Tx {}
#[doc = "Field `TX` reader - Tx"]
pub type TxR = crate::FieldReader<Tx>;
impl TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tx> {
        match self.bits {
            2 => Some(Tx::Value1_2),
            1 => Some(Tx::Value1_4),
            0 => Some(Tx::Value1_8),
            3 => Some(Tx::Value3_4),
            4 => Some(Tx::Value7_8),
            _ => None,
        }
    }
    #[doc = "Value 1 2"]
    #[inline(always)]
    pub fn is_value_1_2(&self) -> bool {
        *self == Tx::Value1_2
    }
    #[doc = "Value 1 4"]
    #[inline(always)]
    pub fn is_value_1_4(&self) -> bool {
        *self == Tx::Value1_4
    }
    #[doc = "Value 1 8"]
    #[inline(always)]
    pub fn is_value_1_8(&self) -> bool {
        *self == Tx::Value1_8
    }
    #[doc = "Value 3 4"]
    #[inline(always)]
    pub fn is_value_3_4(&self) -> bool {
        *self == Tx::Value3_4
    }
    #[doc = "Value 7 8"]
    #[inline(always)]
    pub fn is_value_7_8(&self) -> bool {
        *self == Tx::Value7_8
    }
}
#[doc = "Field `TX` writer - Tx"]
pub type TxW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tx>;
impl<'a, REG> TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value 1 2"]
    #[inline(always)]
    pub fn value_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Value1_2)
    }
    #[doc = "Value 1 4"]
    #[inline(always)]
    pub fn value_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Value1_4)
    }
    #[doc = "Value 1 8"]
    #[inline(always)]
    pub fn value_1_8(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Value1_8)
    }
    #[doc = "Value 3 4"]
    #[inline(always)]
    pub fn value_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Value3_4)
    }
    #[doc = "Value 7 8"]
    #[inline(always)]
    pub fn value_7_8(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Value7_8)
    }
}
#[doc = "Rx"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rx {
    #[doc = "2: Value 1 2"]
    Value1_2 = 2,
    #[doc = "1: Value 1 4"]
    Value1_4 = 1,
    #[doc = "0: Value 1 8"]
    Value1_8 = 0,
    #[doc = "3: Value 3 4"]
    Value3_4 = 3,
    #[doc = "4: Value 7 8"]
    Value7_8 = 4,
}
impl From<Rx> for u8 {
    #[inline(always)]
    fn from(variant: Rx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rx {
    type Ux = u8;
}
impl crate::IsEnum for Rx {}
#[doc = "Field `RX` reader - Rx"]
pub type RxR = crate::FieldReader<Rx>;
impl RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rx> {
        match self.bits {
            2 => Some(Rx::Value1_2),
            1 => Some(Rx::Value1_4),
            0 => Some(Rx::Value1_8),
            3 => Some(Rx::Value3_4),
            4 => Some(Rx::Value7_8),
            _ => None,
        }
    }
    #[doc = "Value 1 2"]
    #[inline(always)]
    pub fn is_value_1_2(&self) -> bool {
        *self == Rx::Value1_2
    }
    #[doc = "Value 1 4"]
    #[inline(always)]
    pub fn is_value_1_4(&self) -> bool {
        *self == Rx::Value1_4
    }
    #[doc = "Value 1 8"]
    #[inline(always)]
    pub fn is_value_1_8(&self) -> bool {
        *self == Rx::Value1_8
    }
    #[doc = "Value 3 4"]
    #[inline(always)]
    pub fn is_value_3_4(&self) -> bool {
        *self == Rx::Value3_4
    }
    #[doc = "Value 7 8"]
    #[inline(always)]
    pub fn is_value_7_8(&self) -> bool {
        *self == Rx::Value7_8
    }
}
#[doc = "Field `RX` writer - Rx"]
pub type RxW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rx>;
impl<'a, REG> RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value 1 2"]
    #[inline(always)]
    pub fn value_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Value1_2)
    }
    #[doc = "Value 1 4"]
    #[inline(always)]
    pub fn value_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Value1_4)
    }
    #[doc = "Value 1 8"]
    #[inline(always)]
    pub fn value_1_8(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Value1_8)
    }
    #[doc = "Value 3 4"]
    #[inline(always)]
    pub fn value_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Value3_4)
    }
    #[doc = "Value 7 8"]
    #[inline(always)]
    pub fn value_7_8(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Value7_8)
    }
}
impl R {
    #[doc = "Bits 0:2 - Tx"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Rx"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tx"]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, IflsSpec> {
        TxW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Rx"]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<'_, IflsSpec> {
        RxW::new(self, 3)
    }
}
#[doc = "interrupt fifo level select register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifls::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IflsSpec;
impl crate::RegisterSpec for IflsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifls::R`](R) reader structure"]
impl crate::Readable for IflsSpec {}
#[doc = "`write(|w| ..)` method takes [`ifls::W`](W) writer structure"]
impl crate::Writable for IflsSpec {
    type Safety = crate::Unsafe;
}

#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `INT_EN` reader - DMA interrupt enable"]
pub type IntEnR = crate::BitReader;
#[doc = "Field `INT_EN` writer - DMA interrupt enable"]
pub type IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMA destination data width configuration"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DstTrWidth {
    #[doc = "0: 8 bits"]
    Bits8 = 0,
    #[doc = "1: 16 bits"]
    Bits16 = 1,
    #[doc = "2: 32 bits"]
    Bits32 = 2,
}
impl From<DstTrWidth> for u8 {
    #[inline(always)]
    fn from(variant: DstTrWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DstTrWidth {
    type Ux = u8;
}
impl crate::IsEnum for DstTrWidth {}
#[doc = "Field `DST_TR_WIDTH` reader - DMA destination data width configuration"]
pub type DstTrWidthR = crate::FieldReader<DstTrWidth>;
impl DstTrWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DstTrWidth> {
        match self.bits {
            0 => Some(DstTrWidth::Bits8),
            1 => Some(DstTrWidth::Bits16),
            2 => Some(DstTrWidth::Bits32),
            _ => None,
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_bits_8(&self) -> bool {
        *self == DstTrWidth::Bits8
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_bits_16(&self) -> bool {
        *self == DstTrWidth::Bits16
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_bits_32(&self) -> bool {
        *self == DstTrWidth::Bits32
    }
}
#[doc = "Field `DST_TR_WIDTH` writer - DMA destination data width configuration"]
pub type DstTrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3, DstTrWidth>;
impl<'a, REG> DstTrWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn bits_8(self) -> &'a mut crate::W<REG> {
        self.variant(DstTrWidth::Bits8)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn bits_16(self) -> &'a mut crate::W<REG> {
        self.variant(DstTrWidth::Bits16)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn bits_32(self) -> &'a mut crate::W<REG> {
        self.variant(DstTrWidth::Bits32)
    }
}
#[doc = "DMA source data width configuration"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SrcTrWidth {
    #[doc = "0: 8 bits"]
    Bits8 = 0,
    #[doc = "1: 16 bits"]
    Bits16 = 1,
    #[doc = "2: 32 bits"]
    Bits32 = 2,
}
impl From<SrcTrWidth> for u8 {
    #[inline(always)]
    fn from(variant: SrcTrWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SrcTrWidth {
    type Ux = u8;
}
impl crate::IsEnum for SrcTrWidth {}
#[doc = "Field `SRC_TR_WIDTH` reader - DMA source data width configuration"]
pub type SrcTrWidthR = crate::FieldReader<SrcTrWidth>;
impl SrcTrWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SrcTrWidth> {
        match self.bits {
            0 => Some(SrcTrWidth::Bits8),
            1 => Some(SrcTrWidth::Bits16),
            2 => Some(SrcTrWidth::Bits32),
            _ => None,
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_bits_8(&self) -> bool {
        *self == SrcTrWidth::Bits8
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_bits_16(&self) -> bool {
        *self == SrcTrWidth::Bits16
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_bits_32(&self) -> bool {
        *self == SrcTrWidth::Bits32
    }
}
#[doc = "Field `SRC_TR_WIDTH` writer - DMA source data width configuration"]
pub type SrcTrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3, SrcTrWidth>;
impl<'a, REG> SrcTrWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn bits_8(self) -> &'a mut crate::W<REG> {
        self.variant(SrcTrWidth::Bits8)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn bits_16(self) -> &'a mut crate::W<REG> {
        self.variant(SrcTrWidth::Bits16)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn bits_32(self) -> &'a mut crate::W<REG> {
        self.variant(SrcTrWidth::Bits32)
    }
}
#[doc = "DMA destination address control"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dinc {
    #[doc = "0: increment"]
    Increment = 0,
    #[doc = "1: decrement"]
    Decrement = 1,
    #[doc = "2: no change"]
    NoChange10 = 2,
    #[doc = "3: no change"]
    NoChange11 = 3,
}
impl From<Dinc> for u8 {
    #[inline(always)]
    fn from(variant: Dinc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dinc {
    type Ux = u8;
}
impl crate::IsEnum for Dinc {}
#[doc = "Field `DINC` reader - DMA destination address control"]
pub type DincR = crate::FieldReader<Dinc>;
impl DincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dinc {
        match self.bits {
            0 => Dinc::Increment,
            1 => Dinc::Decrement,
            2 => Dinc::NoChange10,
            3 => Dinc::NoChange11,
            _ => unreachable!(),
        }
    }
    #[doc = "increment"]
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        *self == Dinc::Increment
    }
    #[doc = "decrement"]
    #[inline(always)]
    pub fn is_decrement(&self) -> bool {
        *self == Dinc::Decrement
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn is_no_change_10(&self) -> bool {
        *self == Dinc::NoChange10
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn is_no_change_11(&self) -> bool {
        *self == Dinc::NoChange11
    }
}
#[doc = "Field `DINC` writer - DMA destination address control"]
pub type DincW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dinc, crate::Safe>;
impl<'a, REG> DincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "increment"]
    #[inline(always)]
    pub fn increment(self) -> &'a mut crate::W<REG> {
        self.variant(Dinc::Increment)
    }
    #[doc = "decrement"]
    #[inline(always)]
    pub fn decrement(self) -> &'a mut crate::W<REG> {
        self.variant(Dinc::Decrement)
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn no_change_10(self) -> &'a mut crate::W<REG> {
        self.variant(Dinc::NoChange10)
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn no_change_11(self) -> &'a mut crate::W<REG> {
        self.variant(Dinc::NoChange11)
    }
}
#[doc = "DMA source address control"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sinc {
    #[doc = "0: increment"]
    Increment = 0,
    #[doc = "1: decrement"]
    Decrement = 1,
    #[doc = "2: no change"]
    NoChange10 = 2,
    #[doc = "3: no change"]
    NoChange11 = 3,
}
impl From<Sinc> for u8 {
    #[inline(always)]
    fn from(variant: Sinc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sinc {
    type Ux = u8;
}
impl crate::IsEnum for Sinc {}
#[doc = "Field `SINC` reader - DMA source address control"]
pub type SincR = crate::FieldReader<Sinc>;
impl SincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sinc {
        match self.bits {
            0 => Sinc::Increment,
            1 => Sinc::Decrement,
            2 => Sinc::NoChange10,
            3 => Sinc::NoChange11,
            _ => unreachable!(),
        }
    }
    #[doc = "increment"]
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        *self == Sinc::Increment
    }
    #[doc = "decrement"]
    #[inline(always)]
    pub fn is_decrement(&self) -> bool {
        *self == Sinc::Decrement
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn is_no_change_10(&self) -> bool {
        *self == Sinc::NoChange10
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn is_no_change_11(&self) -> bool {
        *self == Sinc::NoChange11
    }
}
#[doc = "Field `SINC` writer - DMA source address control"]
pub type SincW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sinc, crate::Safe>;
impl<'a, REG> SincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "increment"]
    #[inline(always)]
    pub fn increment(self) -> &'a mut crate::W<REG> {
        self.variant(Sinc::Increment)
    }
    #[doc = "decrement"]
    #[inline(always)]
    pub fn decrement(self) -> &'a mut crate::W<REG> {
        self.variant(Sinc::Decrement)
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn no_change_10(self) -> &'a mut crate::W<REG> {
        self.variant(Sinc::NoChange10)
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn no_change_11(self) -> &'a mut crate::W<REG> {
        self.variant(Sinc::NoChange11)
    }
}
#[doc = "DMA destination burst length"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DestMsize {
    #[doc = "0: burst length 1"]
    Value1 = 0,
    #[doc = "1: burst length 4"]
    Value4 = 1,
    #[doc = "2: burst length 8"]
    Value8 = 2,
}
impl From<DestMsize> for u8 {
    #[inline(always)]
    fn from(variant: DestMsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DestMsize {
    type Ux = u8;
}
impl crate::IsEnum for DestMsize {}
#[doc = "Field `DEST_MSIZE` reader - DMA destination burst length"]
pub type DestMsizeR = crate::FieldReader<DestMsize>;
impl DestMsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DestMsize> {
        match self.bits {
            0 => Some(DestMsize::Value1),
            1 => Some(DestMsize::Value4),
            2 => Some(DestMsize::Value8),
            _ => None,
        }
    }
    #[doc = "burst length 1"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == DestMsize::Value1
    }
    #[doc = "burst length 4"]
    #[inline(always)]
    pub fn is_value_4(&self) -> bool {
        *self == DestMsize::Value4
    }
    #[doc = "burst length 8"]
    #[inline(always)]
    pub fn is_value_8(&self) -> bool {
        *self == DestMsize::Value8
    }
}
#[doc = "Field `DEST_MSIZE` writer - DMA destination burst length"]
pub type DestMsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, DestMsize>;
impl<'a, REG> DestMsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "burst length 1"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(DestMsize::Value1)
    }
    #[doc = "burst length 4"]
    #[inline(always)]
    pub fn value_4(self) -> &'a mut crate::W<REG> {
        self.variant(DestMsize::Value4)
    }
    #[doc = "burst length 8"]
    #[inline(always)]
    pub fn value_8(self) -> &'a mut crate::W<REG> {
        self.variant(DestMsize::Value8)
    }
}
#[doc = "DMA source burst length"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SrcMsize {
    #[doc = "0: burst length 1"]
    Value1 = 0,
    #[doc = "1: burst length 4"]
    Value4 = 1,
    #[doc = "2: burst length 8"]
    Value8 = 2,
}
impl From<SrcMsize> for u8 {
    #[inline(always)]
    fn from(variant: SrcMsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SrcMsize {
    type Ux = u8;
}
impl crate::IsEnum for SrcMsize {}
#[doc = "Field `SRC_MSIZE` reader - DMA source burst length"]
pub type SrcMsizeR = crate::FieldReader<SrcMsize>;
impl SrcMsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SrcMsize> {
        match self.bits {
            0 => Some(SrcMsize::Value1),
            1 => Some(SrcMsize::Value4),
            2 => Some(SrcMsize::Value8),
            _ => None,
        }
    }
    #[doc = "burst length 1"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == SrcMsize::Value1
    }
    #[doc = "burst length 4"]
    #[inline(always)]
    pub fn is_value_4(&self) -> bool {
        *self == SrcMsize::Value4
    }
    #[doc = "burst length 8"]
    #[inline(always)]
    pub fn is_value_8(&self) -> bool {
        *self == SrcMsize::Value8
    }
}
#[doc = "Field `SRC_MSIZE` writer - DMA source burst length"]
pub type SrcMsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, SrcMsize>;
impl<'a, REG> SrcMsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "burst length 1"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(SrcMsize::Value1)
    }
    #[doc = "burst length 4"]
    #[inline(always)]
    pub fn value_4(self) -> &'a mut crate::W<REG> {
        self.variant(SrcMsize::Value4)
    }
    #[doc = "burst length 8"]
    #[inline(always)]
    pub fn value_8(self) -> &'a mut crate::W<REG> {
        self.variant(SrcMsize::Value8)
    }
}
#[doc = "Field `SRC_GATHER_EN` reader - DMA source gather enable"]
pub type SrcGatherEnR = crate::BitReader;
#[doc = "Field `SRC_GATHER_EN` writer - DMA source gather enable"]
pub type SrcGatherEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_SCATTER_EN` reader - DMA destination scatter enable"]
pub type DstScatterEnR = crate::BitReader;
#[doc = "Field `DST_SCATTER_EN` writer - DMA destination scatter enable"]
pub type DstScatterEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMA data transfer mode selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TtFc {
    #[doc = "0: memory to memory"]
    MemToMem = 0,
    #[doc = "1: memory to peripheral"]
    MemToPeriph = 1,
    #[doc = "2: peripheral to memory"]
    PeriphToMem = 2,
    #[doc = "3: peripheral to peripheral"]
    PeriphToPeriph = 3,
}
impl From<TtFc> for u8 {
    #[inline(always)]
    fn from(variant: TtFc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TtFc {
    type Ux = u8;
}
impl crate::IsEnum for TtFc {}
#[doc = "Field `TT_FC` reader - DMA data transfer mode selection"]
pub type TtFcR = crate::FieldReader<TtFc>;
impl TtFcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TtFc> {
        match self.bits {
            0 => Some(TtFc::MemToMem),
            1 => Some(TtFc::MemToPeriph),
            2 => Some(TtFc::PeriphToMem),
            3 => Some(TtFc::PeriphToPeriph),
            _ => None,
        }
    }
    #[doc = "memory to memory"]
    #[inline(always)]
    pub fn is_mem_to_mem(&self) -> bool {
        *self == TtFc::MemToMem
    }
    #[doc = "memory to peripheral"]
    #[inline(always)]
    pub fn is_mem_to_periph(&self) -> bool {
        *self == TtFc::MemToPeriph
    }
    #[doc = "peripheral to memory"]
    #[inline(always)]
    pub fn is_periph_to_mem(&self) -> bool {
        *self == TtFc::PeriphToMem
    }
    #[doc = "peripheral to peripheral"]
    #[inline(always)]
    pub fn is_periph_to_periph(&self) -> bool {
        *self == TtFc::PeriphToPeriph
    }
}
#[doc = "Field `TT_FC` writer - DMA data transfer mode selection"]
pub type TtFcW<'a, REG> = crate::FieldWriter<'a, REG, 3, TtFc>;
impl<'a, REG> TtFcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "memory to memory"]
    #[inline(always)]
    pub fn mem_to_mem(self) -> &'a mut crate::W<REG> {
        self.variant(TtFc::MemToMem)
    }
    #[doc = "memory to peripheral"]
    #[inline(always)]
    pub fn mem_to_periph(self) -> &'a mut crate::W<REG> {
        self.variant(TtFc::MemToPeriph)
    }
    #[doc = "peripheral to memory"]
    #[inline(always)]
    pub fn periph_to_mem(self) -> &'a mut crate::W<REG> {
        self.variant(TtFc::PeriphToMem)
    }
    #[doc = "peripheral to peripheral"]
    #[inline(always)]
    pub fn periph_to_periph(self) -> &'a mut crate::W<REG> {
        self.variant(TtFc::PeriphToPeriph)
    }
}
#[doc = "DMA destination AHB master selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dms {
    #[doc = "0: AHB master 1"]
    Master1 = 0,
    #[doc = "1: AHB master 2"]
    Master2 = 1,
    #[doc = "2: AHB master 3"]
    Master3 = 2,
    #[doc = "3: AHB master 4"]
    Master4 = 3,
}
impl From<Dms> for u8 {
    #[inline(always)]
    fn from(variant: Dms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dms {
    type Ux = u8;
}
impl crate::IsEnum for Dms {}
#[doc = "Field `DMS` reader - DMA destination AHB master selection"]
pub type DmsR = crate::FieldReader<Dms>;
impl DmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dms {
        match self.bits {
            0 => Dms::Master1,
            1 => Dms::Master2,
            2 => Dms::Master3,
            3 => Dms::Master4,
            _ => unreachable!(),
        }
    }
    #[doc = "AHB master 1"]
    #[inline(always)]
    pub fn is_master_1(&self) -> bool {
        *self == Dms::Master1
    }
    #[doc = "AHB master 2"]
    #[inline(always)]
    pub fn is_master_2(&self) -> bool {
        *self == Dms::Master2
    }
    #[doc = "AHB master 3"]
    #[inline(always)]
    pub fn is_master_3(&self) -> bool {
        *self == Dms::Master3
    }
    #[doc = "AHB master 4"]
    #[inline(always)]
    pub fn is_master_4(&self) -> bool {
        *self == Dms::Master4
    }
}
#[doc = "Field `DMS` writer - DMA destination AHB master selection"]
pub type DmsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dms, crate::Safe>;
impl<'a, REG> DmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AHB master 1"]
    #[inline(always)]
    pub fn master_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dms::Master1)
    }
    #[doc = "AHB master 2"]
    #[inline(always)]
    pub fn master_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dms::Master2)
    }
    #[doc = "AHB master 3"]
    #[inline(always)]
    pub fn master_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dms::Master3)
    }
    #[doc = "AHB master 4"]
    #[inline(always)]
    pub fn master_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dms::Master4)
    }
}
#[doc = "DMA source AHB master selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sms {
    #[doc = "0: AHB master 1"]
    Master1 = 0,
    #[doc = "1: AHB master 2"]
    Master2 = 1,
    #[doc = "2: AHB master 3"]
    Master3 = 2,
    #[doc = "3: AHB master 4"]
    Master4 = 3,
}
impl From<Sms> for u8 {
    #[inline(always)]
    fn from(variant: Sms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sms {
    type Ux = u8;
}
impl crate::IsEnum for Sms {}
#[doc = "Field `SMS` reader - DMA source AHB master selection"]
pub type SmsR = crate::FieldReader<Sms>;
impl SmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sms {
        match self.bits {
            0 => Sms::Master1,
            1 => Sms::Master2,
            2 => Sms::Master3,
            3 => Sms::Master4,
            _ => unreachable!(),
        }
    }
    #[doc = "AHB master 1"]
    #[inline(always)]
    pub fn is_master_1(&self) -> bool {
        *self == Sms::Master1
    }
    #[doc = "AHB master 2"]
    #[inline(always)]
    pub fn is_master_2(&self) -> bool {
        *self == Sms::Master2
    }
    #[doc = "AHB master 3"]
    #[inline(always)]
    pub fn is_master_3(&self) -> bool {
        *self == Sms::Master3
    }
    #[doc = "AHB master 4"]
    #[inline(always)]
    pub fn is_master_4(&self) -> bool {
        *self == Sms::Master4
    }
}
#[doc = "Field `SMS` writer - DMA source AHB master selection"]
pub type SmsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sms, crate::Safe>;
impl<'a, REG> SmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AHB master 1"]
    #[inline(always)]
    pub fn master_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sms::Master1)
    }
    #[doc = "AHB master 2"]
    #[inline(always)]
    pub fn master_2(self) -> &'a mut crate::W<REG> {
        self.variant(Sms::Master2)
    }
    #[doc = "AHB master 3"]
    #[inline(always)]
    pub fn master_3(self) -> &'a mut crate::W<REG> {
        self.variant(Sms::Master3)
    }
    #[doc = "AHB master 4"]
    #[inline(always)]
    pub fn master_4(self) -> &'a mut crate::W<REG> {
        self.variant(Sms::Master4)
    }
}
#[doc = "Field `LLP_DST_EN` reader - DMA destination LLI chain table enable"]
pub type LlpDstEnR = crate::BitReader;
#[doc = "Field `LLP_DST_EN` writer - DMA destination LLI chain table enable"]
pub type LlpDstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLP_SRC_EN` reader - DMA source LLI chain table enable"]
pub type LlpSrcEnR = crate::BitReader;
#[doc = "Field `LLP_SRC_EN` writer - DMA source LLI chain table enable"]
pub type LlpSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCK_TS` reader - block length"]
pub type BlockTsR = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_TS` writer - block length"]
pub type BlockTsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DONE` reader - LLI chain table block transfer status"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - LLI chain table block transfer status"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA interrupt enable"]
    #[inline(always)]
    pub fn int_en(&self) -> IntEnR {
        IntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - DMA destination data width configuration"]
    #[inline(always)]
    pub fn dst_tr_width(&self) -> DstTrWidthR {
        DstTrWidthR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - DMA source data width configuration"]
    #[inline(always)]
    pub fn src_tr_width(&self) -> SrcTrWidthR {
        SrcTrWidthR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - DMA destination address control"]
    #[inline(always)]
    pub fn dinc(&self) -> DincR {
        DincR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - DMA source address control"]
    #[inline(always)]
    pub fn sinc(&self) -> SincR {
        SincR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:13 - DMA destination burst length"]
    #[inline(always)]
    pub fn dest_msize(&self) -> DestMsizeR {
        DestMsizeR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - DMA source burst length"]
    #[inline(always)]
    pub fn src_msize(&self) -> SrcMsizeR {
        SrcMsizeR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17 - DMA source gather enable"]
    #[inline(always)]
    pub fn src_gather_en(&self) -> SrcGatherEnR {
        SrcGatherEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMA destination scatter enable"]
    #[inline(always)]
    pub fn dst_scatter_en(&self) -> DstScatterEnR {
        DstScatterEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:22 - DMA data transfer mode selection"]
    #[inline(always)]
    pub fn tt_fc(&self) -> TtFcR {
        TtFcR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:24 - DMA destination AHB master selection"]
    #[inline(always)]
    pub fn dms(&self) -> DmsR {
        DmsR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - DMA source AHB master selection"]
    #[inline(always)]
    pub fn sms(&self) -> SmsR {
        SmsR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - DMA destination LLI chain table enable"]
    #[inline(always)]
    pub fn llp_dst_en(&self) -> LlpDstEnR {
        LlpDstEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA source LLI chain table enable"]
    #[inline(always)]
    pub fn llp_src_en(&self) -> LlpSrcEnR {
        LlpSrcEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 32:43 - block length"]
    #[inline(always)]
    pub fn block_ts(&self) -> BlockTsR {
        BlockTsR::new(((self.bits >> 32) & 0x0fff) as u16)
    }
    #[doc = "Bit 44 - LLI chain table block transfer status"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 44) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA interrupt enable"]
    #[inline(always)]
    pub fn int_en(&mut self) -> IntEnW<'_, Ctl1Spec> {
        IntEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - DMA destination data width configuration"]
    #[inline(always)]
    pub fn dst_tr_width(&mut self) -> DstTrWidthW<'_, Ctl1Spec> {
        DstTrWidthW::new(self, 1)
    }
    #[doc = "Bits 4:6 - DMA source data width configuration"]
    #[inline(always)]
    pub fn src_tr_width(&mut self) -> SrcTrWidthW<'_, Ctl1Spec> {
        SrcTrWidthW::new(self, 4)
    }
    #[doc = "Bits 7:8 - DMA destination address control"]
    #[inline(always)]
    pub fn dinc(&mut self) -> DincW<'_, Ctl1Spec> {
        DincW::new(self, 7)
    }
    #[doc = "Bits 9:10 - DMA source address control"]
    #[inline(always)]
    pub fn sinc(&mut self) -> SincW<'_, Ctl1Spec> {
        SincW::new(self, 9)
    }
    #[doc = "Bits 11:13 - DMA destination burst length"]
    #[inline(always)]
    pub fn dest_msize(&mut self) -> DestMsizeW<'_, Ctl1Spec> {
        DestMsizeW::new(self, 11)
    }
    #[doc = "Bits 14:16 - DMA source burst length"]
    #[inline(always)]
    pub fn src_msize(&mut self) -> SrcMsizeW<'_, Ctl1Spec> {
        SrcMsizeW::new(self, 14)
    }
    #[doc = "Bit 17 - DMA source gather enable"]
    #[inline(always)]
    pub fn src_gather_en(&mut self) -> SrcGatherEnW<'_, Ctl1Spec> {
        SrcGatherEnW::new(self, 17)
    }
    #[doc = "Bit 18 - DMA destination scatter enable"]
    #[inline(always)]
    pub fn dst_scatter_en(&mut self) -> DstScatterEnW<'_, Ctl1Spec> {
        DstScatterEnW::new(self, 18)
    }
    #[doc = "Bits 20:22 - DMA data transfer mode selection"]
    #[inline(always)]
    pub fn tt_fc(&mut self) -> TtFcW<'_, Ctl1Spec> {
        TtFcW::new(self, 20)
    }
    #[doc = "Bits 23:24 - DMA destination AHB master selection"]
    #[inline(always)]
    pub fn dms(&mut self) -> DmsW<'_, Ctl1Spec> {
        DmsW::new(self, 23)
    }
    #[doc = "Bits 25:26 - DMA source AHB master selection"]
    #[inline(always)]
    pub fn sms(&mut self) -> SmsW<'_, Ctl1Spec> {
        SmsW::new(self, 25)
    }
    #[doc = "Bit 27 - DMA destination LLI chain table enable"]
    #[inline(always)]
    pub fn llp_dst_en(&mut self) -> LlpDstEnW<'_, Ctl1Spec> {
        LlpDstEnW::new(self, 27)
    }
    #[doc = "Bit 28 - DMA source LLI chain table enable"]
    #[inline(always)]
    pub fn llp_src_en(&mut self) -> LlpSrcEnW<'_, Ctl1Spec> {
        LlpSrcEnW::new(self, 28)
    }
    #[doc = "Bits 32:43 - block length"]
    #[inline(always)]
    pub fn block_ts(&mut self) -> BlockTsW<'_, Ctl1Spec> {
        BlockTsW::new(self, 32)
    }
    #[doc = "Bit 44 - LLI chain table block transfer status"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, Ctl1Spec> {
        DoneW::new(self, 44)
    }
}
#[doc = "channel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}

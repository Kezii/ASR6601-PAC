#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `START` reader - Start"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Prediv"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prediv {
    #[doc = "5: Value 128"]
    Value128 = 5,
    #[doc = "2: Value 16"]
    Value16 = 2,
    #[doc = "6: Value 256"]
    Value256 = 6,
    #[doc = "3: Value 32"]
    Value32 = 3,
    #[doc = "0: Value 4"]
    Value4 = 0,
    #[doc = "4: Value 64"]
    Value64 = 4,
    #[doc = "1: Value 8"]
    Value8 = 1,
}
impl From<Prediv> for u8 {
    #[inline(always)]
    fn from(variant: Prediv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prediv {
    type Ux = u8;
}
impl crate::IsEnum for Prediv {}
#[doc = "Field `PREDIV` reader - Prediv"]
pub type PredivR = crate::FieldReader<Prediv>;
impl PredivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prediv> {
        match self.bits {
            5 => Some(Prediv::Value128),
            2 => Some(Prediv::Value16),
            6 => Some(Prediv::Value256),
            3 => Some(Prediv::Value32),
            0 => Some(Prediv::Value4),
            4 => Some(Prediv::Value64),
            1 => Some(Prediv::Value8),
            _ => None,
        }
    }
    #[doc = "Value 128"]
    #[inline(always)]
    pub fn is_value_128(&self) -> bool {
        *self == Prediv::Value128
    }
    #[doc = "Value 16"]
    #[inline(always)]
    pub fn is_value_16(&self) -> bool {
        *self == Prediv::Value16
    }
    #[doc = "Value 256"]
    #[inline(always)]
    pub fn is_value_256(&self) -> bool {
        *self == Prediv::Value256
    }
    #[doc = "Value 32"]
    #[inline(always)]
    pub fn is_value_32(&self) -> bool {
        *self == Prediv::Value32
    }
    #[doc = "Value 4"]
    #[inline(always)]
    pub fn is_value_4(&self) -> bool {
        *self == Prediv::Value4
    }
    #[doc = "Value 64"]
    #[inline(always)]
    pub fn is_value_64(&self) -> bool {
        *self == Prediv::Value64
    }
    #[doc = "Value 8"]
    #[inline(always)]
    pub fn is_value_8(&self) -> bool {
        *self == Prediv::Value8
    }
}
#[doc = "Field `PREDIV` writer - Prediv"]
pub type PredivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prediv>;
impl<'a, REG> PredivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value 128"]
    #[inline(always)]
    pub fn value_128(self) -> &'a mut crate::W<REG> {
        self.variant(Prediv::Value128)
    }
    #[doc = "Value 16"]
    #[inline(always)]
    pub fn value_16(self) -> &'a mut crate::W<REG> {
        self.variant(Prediv::Value16)
    }
    #[doc = "Value 256"]
    #[inline(always)]
    pub fn value_256(self) -> &'a mut crate::W<REG> {
        self.variant(Prediv::Value256)
    }
    #[doc = "Value 32"]
    #[inline(always)]
    pub fn value_32(self) -> &'a mut crate::W<REG> {
        self.variant(Prediv::Value32)
    }
    #[doc = "Value 4"]
    #[inline(always)]
    pub fn value_4(self) -> &'a mut crate::W<REG> {
        self.variant(Prediv::Value4)
    }
    #[doc = "Value 64"]
    #[inline(always)]
    pub fn value_64(self) -> &'a mut crate::W<REG> {
        self.variant(Prediv::Value64)
    }
    #[doc = "Value 8"]
    #[inline(always)]
    pub fn value_8(self) -> &'a mut crate::W<REG> {
        self.variant(Prediv::Value8)
    }
}
#[doc = "Field `WKEN` reader - Wken"]
pub type WkenR = crate::BitReader;
#[doc = "Field `WKEN` writer - Wken"]
pub type WkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTEN` reader - Rsten"]
pub type RstenR = crate::BitReader;
#[doc = "Field `RSTEN` writer - Rsten"]
pub type RstenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Prediv"]
    #[inline(always)]
    pub fn prediv(&self) -> PredivR {
        PredivR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Wken"]
    #[inline(always)]
    pub fn wken(&self) -> WkenR {
        WkenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rsten"]
    #[inline(always)]
    pub fn rsten(&self) -> RstenR {
        RstenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, CrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Prediv"]
    #[inline(always)]
    pub fn prediv(&mut self) -> PredivW<'_, CrSpec> {
        PredivW::new(self, 1)
    }
    #[doc = "Bit 4 - Wken"]
    #[inline(always)]
    pub fn wken(&mut self) -> WkenW<'_, CrSpec> {
        WkenW::new(self, 4)
    }
    #[doc = "Bit 5 - Rsten"]
    #[inline(always)]
    pub fn rsten(&mut self) -> RstenW<'_, CrSpec> {
        RstenW::new(self, 5)
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

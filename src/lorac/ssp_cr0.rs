#[doc = "Register `SSP_CR0` reader"]
pub type R = crate::R<SspCr0Spec>;
#[doc = "Register `SSP_CR0` writer"]
pub type W = crate::W<SspCr0Spec>;
#[doc = "data width setting"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dss {
    #[doc = "3: 4-bit data"]
    Value4 = 3,
    #[doc = "4: 5-bit data"]
    Value5 = 4,
    #[doc = "5: 6-bit data"]
    Value6 = 5,
    #[doc = "6: 7-bit data"]
    Value7 = 6,
    #[doc = "7: 8-bit data"]
    Value8 = 7,
    #[doc = "8: 9-bit data"]
    Value9 = 8,
    #[doc = "9: 10-bit data"]
    Value10 = 9,
    #[doc = "10: 11-bit data"]
    Value11 = 10,
    #[doc = "11: 12-bit data"]
    Value12 = 11,
    #[doc = "12: 13-bit data"]
    Value13 = 12,
    #[doc = "13: 14-bit data"]
    Value14 = 13,
    #[doc = "14: 15-bit data"]
    Value15 = 14,
    #[doc = "15: 16-bit data"]
    Value16 = 15,
}
impl From<Dss> for u8 {
    #[inline(always)]
    fn from(variant: Dss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dss {
    type Ux = u8;
}
impl crate::IsEnum for Dss {}
#[doc = "Field `DSS` reader - data width setting"]
pub type DssR = crate::FieldReader<Dss>;
impl DssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dss> {
        match self.bits {
            3 => Some(Dss::Value4),
            4 => Some(Dss::Value5),
            5 => Some(Dss::Value6),
            6 => Some(Dss::Value7),
            7 => Some(Dss::Value8),
            8 => Some(Dss::Value9),
            9 => Some(Dss::Value10),
            10 => Some(Dss::Value11),
            11 => Some(Dss::Value12),
            12 => Some(Dss::Value13),
            13 => Some(Dss::Value14),
            14 => Some(Dss::Value15),
            15 => Some(Dss::Value16),
            _ => None,
        }
    }
    #[doc = "4-bit data"]
    #[inline(always)]
    pub fn is_value_4(&self) -> bool {
        *self == Dss::Value4
    }
    #[doc = "5-bit data"]
    #[inline(always)]
    pub fn is_value_5(&self) -> bool {
        *self == Dss::Value5
    }
    #[doc = "6-bit data"]
    #[inline(always)]
    pub fn is_value_6(&self) -> bool {
        *self == Dss::Value6
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn is_value_7(&self) -> bool {
        *self == Dss::Value7
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn is_value_8(&self) -> bool {
        *self == Dss::Value8
    }
    #[doc = "9-bit data"]
    #[inline(always)]
    pub fn is_value_9(&self) -> bool {
        *self == Dss::Value9
    }
    #[doc = "10-bit data"]
    #[inline(always)]
    pub fn is_value_10(&self) -> bool {
        *self == Dss::Value10
    }
    #[doc = "11-bit data"]
    #[inline(always)]
    pub fn is_value_11(&self) -> bool {
        *self == Dss::Value11
    }
    #[doc = "12-bit data"]
    #[inline(always)]
    pub fn is_value_12(&self) -> bool {
        *self == Dss::Value12
    }
    #[doc = "13-bit data"]
    #[inline(always)]
    pub fn is_value_13(&self) -> bool {
        *self == Dss::Value13
    }
    #[doc = "14-bit data"]
    #[inline(always)]
    pub fn is_value_14(&self) -> bool {
        *self == Dss::Value14
    }
    #[doc = "15-bit data"]
    #[inline(always)]
    pub fn is_value_15(&self) -> bool {
        *self == Dss::Value15
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn is_value_16(&self) -> bool {
        *self == Dss::Value16
    }
}
#[doc = "Field `DSS` writer - data width setting"]
pub type DssW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dss>;
impl<'a, REG> DssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4-bit data"]
    #[inline(always)]
    pub fn value_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Value4)
    }
    #[doc = "5-bit data"]
    #[inline(always)]
    pub fn value_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Value5)
    }
    #[doc = "6-bit data"]
    #[inline(always)]
    pub fn value_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Value6)
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn value_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Value7)
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn value_8(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Value8)
    }
    #[doc = "9-bit data"]
    #[inline(always)]
    pub fn value_9(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Value9)
    }
    #[doc = "10-bit data"]
    #[inline(always)]
    pub fn value_10(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Value10)
    }
    #[doc = "11-bit data"]
    #[inline(always)]
    pub fn value_11(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Value11)
    }
    #[doc = "12-bit data"]
    #[inline(always)]
    pub fn value_12(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Value12)
    }
    #[doc = "13-bit data"]
    #[inline(always)]
    pub fn value_13(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Value13)
    }
    #[doc = "14-bit data"]
    #[inline(always)]
    pub fn value_14(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Value14)
    }
    #[doc = "15-bit data"]
    #[inline(always)]
    pub fn value_15(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Value15)
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn value_16(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Value16)
    }
}
#[doc = "frame format setting"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frf {
    #[doc = "0: Motorola SPI"]
    Motorola = 0,
    #[doc = "1: Texas Instruments SPI"]
    Ti = 1,
    #[doc = "2: National Semiconductor Microwire"]
    Microwire = 2,
}
impl From<Frf> for u8 {
    #[inline(always)]
    fn from(variant: Frf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frf {
    type Ux = u8;
}
impl crate::IsEnum for Frf {}
#[doc = "Field `FRF` reader - frame format setting"]
pub type FrfR = crate::FieldReader<Frf>;
impl FrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frf> {
        match self.bits {
            0 => Some(Frf::Motorola),
            1 => Some(Frf::Ti),
            2 => Some(Frf::Microwire),
            _ => None,
        }
    }
    #[doc = "Motorola SPI"]
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        *self == Frf::Motorola
    }
    #[doc = "Texas Instruments SPI"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == Frf::Ti
    }
    #[doc = "National Semiconductor Microwire"]
    #[inline(always)]
    pub fn is_microwire(&self) -> bool {
        *self == Frf::Microwire
    }
}
#[doc = "Field `FRF` writer - frame format setting"]
pub type FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Frf>;
impl<'a, REG> FrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Motorola SPI"]
    #[inline(always)]
    pub fn motorola(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Motorola)
    }
    #[doc = "Texas Instruments SPI"]
    #[inline(always)]
    pub fn ti(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Ti)
    }
    #[doc = "National Semiconductor Microwire"]
    #[inline(always)]
    pub fn microwire(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Microwire)
    }
}
#[doc = "Field `SPO` reader - ssp polarity setting"]
pub type SpoR = crate::BitReader;
#[doc = "Field `SPO` writer - ssp polarity setting"]
pub type SpoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPH` reader - ssp phase setting"]
pub type SphR = crate::BitReader;
#[doc = "Field `SPH` writer - ssp phase setting"]
pub type SphW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCR` reader - serial clock rate"]
pub type ScrR = crate::FieldReader;
#[doc = "Field `SCR` writer - serial clock rate"]
pub type ScrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - data width setting"]
    #[inline(always)]
    pub fn dss(&self) -> DssR {
        DssR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - frame format setting"]
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - ssp polarity setting"]
    #[inline(always)]
    pub fn spo(&self) -> SpoR {
        SpoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ssp phase setting"]
    #[inline(always)]
    pub fn sph(&self) -> SphR {
        SphR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - serial clock rate"]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - data width setting"]
    #[inline(always)]
    pub fn dss(&mut self) -> DssW<'_, SspCr0Spec> {
        DssW::new(self, 0)
    }
    #[doc = "Bits 4:5 - frame format setting"]
    #[inline(always)]
    pub fn frf(&mut self) -> FrfW<'_, SspCr0Spec> {
        FrfW::new(self, 4)
    }
    #[doc = "Bit 6 - ssp polarity setting"]
    #[inline(always)]
    pub fn spo(&mut self) -> SpoW<'_, SspCr0Spec> {
        SpoW::new(self, 6)
    }
    #[doc = "Bit 7 - ssp phase setting"]
    #[inline(always)]
    pub fn sph(&mut self) -> SphW<'_, SspCr0Spec> {
        SphW::new(self, 7)
    }
    #[doc = "Bits 8:15 - serial clock rate"]
    #[inline(always)]
    pub fn scr(&mut self) -> ScrW<'_, SspCr0Spec> {
        ScrW::new(self, 8)
    }
}
#[doc = "ssp control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_cr0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspCr0Spec;
impl crate::RegisterSpec for SspCr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssp_cr0::R`](R) reader structure"]
impl crate::Readable for SspCr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ssp_cr0::W`](W) writer structure"]
impl crate::Writable for SspCr0Spec {
    type Safety = crate::Unsafe;
}

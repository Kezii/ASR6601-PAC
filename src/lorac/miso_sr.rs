#[doc = "Register `MISO_SR` reader"]
pub type R = crate::R<MisoSrSpec>;
#[doc = "miso status flag"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegMiso {
    #[doc = "0: low"]
    Low = 0,
    #[doc = "1: high"]
    High = 1,
}
impl From<RegMiso> for bool {
    #[inline(always)]
    fn from(variant: RegMiso) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REG_MISO` reader - miso status flag"]
pub type RegMisoR = crate::BitReader<RegMiso>;
impl RegMisoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RegMiso {
        match self.bits {
            false => RegMiso::Low,
            true => RegMiso::High,
        }
    }
    #[doc = "low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == RegMiso::Low
    }
    #[doc = "high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == RegMiso::High
    }
}
impl R {
    #[doc = "Bit 0 - miso status flag"]
    #[inline(always)]
    pub fn reg_miso(&self) -> RegMisoR {
        RegMisoR::new((self.bits & 1) != 0)
    }
}
#[doc = "miso status register\n\nYou can [`read`](crate::Reg::read) this register and get [`miso_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisoSrSpec;
impl crate::RegisterSpec for MisoSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miso_sr::R`](R) reader structure"]
impl crate::Readable for MisoSrSpec {}

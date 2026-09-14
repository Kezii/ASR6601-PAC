#[doc = "Register `BMR` reader"]
pub type R = crate::R<BmrSpec>;
#[doc = "Field `SDA` reader - sda pin state"]
pub type SdaR = crate::BitReader;
#[doc = "Field `SCL` reader - scl pin state"]
pub type SclR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - sda pin state"]
    #[inline(always)]
    pub fn sda(&self) -> SdaR {
        SdaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - scl pin state"]
    #[inline(always)]
    pub fn scl(&self) -> SclR {
        SclR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "bus monitor register\n\nYou can [`read`](crate::Reg::read) this register and get [`bmr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmrSpec;
impl crate::RegisterSpec for BmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmr::R`](R) reader structure"]
impl crate::Readable for BmrSpec {}

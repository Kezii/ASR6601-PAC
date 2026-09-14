#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `TFE` reader - transmit fifo empty"]
pub type TfeR = crate::BitReader;
#[doc = "Field `TNF` reader - transmit fifo not full"]
pub type TnfR = crate::BitReader;
#[doc = "Field `RNE` reader - receive fifo not empty"]
pub type RneR = crate::BitReader;
#[doc = "Field `RFF` reader - receive fifo full"]
pub type RffR = crate::BitReader;
#[doc = "Field `BSY` reader - ssp busy"]
pub type BsyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - transmit fifo empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - transmit fifo not full"]
    #[inline(always)]
    pub fn tnf(&self) -> TnfR {
        TnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - receive fifo not empty"]
    #[inline(always)]
    pub fn rne(&self) -> RneR {
        RneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - receive fifo full"]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ssp busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}

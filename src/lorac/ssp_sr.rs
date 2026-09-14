#[doc = "Register `SSP_SR` reader"]
pub type R = crate::R<SspSrSpec>;
#[doc = "Field `TFE` reader - tx fifo empty flag"]
pub type TfeR = crate::BitReader;
#[doc = "Field `TNF` reader - tx fifo not full flag"]
pub type TnfR = crate::BitReader;
#[doc = "Field `RNE` reader - rx fifo not empty flag"]
pub type RneR = crate::BitReader;
#[doc = "Field `RFF` reader - rx fifo full flag"]
pub type RffR = crate::BitReader;
#[doc = "Field `BSY` reader - ssp busy flag"]
pub type BsyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - tx fifo empty flag"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tx fifo not full flag"]
    #[inline(always)]
    pub fn tnf(&self) -> TnfR {
        TnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - rx fifo not empty flag"]
    #[inline(always)]
    pub fn rne(&self) -> RneR {
        RneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - rx fifo full flag"]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ssp busy flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "ssp status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspSrSpec;
impl crate::RegisterSpec for SspSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssp_sr::R`](R) reader structure"]
impl crate::Readable for SspSrSpec {}

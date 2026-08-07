#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Field `CMPM` reader - Cmpm"]
pub type CmpmR = crate::BitReader;
#[doc = "Field `ARRM` reader - Arrm"]
pub type ArrmR = crate::BitReader;
#[doc = "Field `EXTTRIG` reader - Exttrig"]
pub type ExttrigR = crate::BitReader;
#[doc = "Field `UP` reader - Up"]
pub type UpR = crate::BitReader;
#[doc = "Field `DOWN` reader - Down"]
pub type DownR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Cmpm"]
    #[inline(always)]
    pub fn cmpm(&self) -> CmpmR {
        CmpmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Arrm"]
    #[inline(always)]
    pub fn arrm(&self) -> ArrmR {
        ArrmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Exttrig"]
    #[inline(always)]
    pub fn exttrig(&self) -> ExttrigR {
        ExttrigR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Up"]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Down"]
    #[inline(always)]
    pub fn down(&self) -> DownR {
        DownR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "LPTIMER CSR register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}

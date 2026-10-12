#[doc = "Register `SR1` reader"]
pub type R = crate::R<Sr1Spec>;
#[doc = "Field `CMPM` reader - Counter value matching CMP value detection"]
pub type CmpmR = crate::BitReader;
#[doc = "Field `ARRM` reader - Counter value reaching ARR value detection"]
pub type ArrmR = crate::BitReader;
#[doc = "Field `EXTTRIG` reader - Valid trigger edge detection"]
pub type ExttrigR = crate::BitReader;
#[doc = "Field `CMPOK` reader - CMP value loading status"]
pub type CmpokR = crate::BitReader;
#[doc = "Field `ARROK` reader - ARR value loading status"]
pub type ArrokR = crate::BitReader;
#[doc = "Field `UP` reader - Counting direction change to up"]
pub type UpR = crate::BitReader;
#[doc = "Field `DOWN` reader - Counting direction change to down"]
pub type DownR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Counter value matching CMP value detection"]
    #[inline(always)]
    pub fn cmpm(&self) -> CmpmR {
        CmpmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter value reaching ARR value detection"]
    #[inline(always)]
    pub fn arrm(&self) -> ArrmR {
        ArrmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Valid trigger edge detection"]
    #[inline(always)]
    pub fn exttrig(&self) -> ExttrigR {
        ExttrigR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMP value loading status"]
    #[inline(always)]
    pub fn cmpok(&self) -> CmpokR {
        CmpokR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARR value loading status"]
    #[inline(always)]
    pub fn arrok(&self) -> ArrokR {
        ArrokR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counting direction change to up"]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counting direction change to down"]
    #[inline(always)]
    pub fn down(&self) -> DownR {
        DownR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "LPTIM SR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr1Spec;
impl crate::RegisterSpec for Sr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for Sr1Spec {}

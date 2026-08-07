#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `CMPM` reader - Cmpm"]
pub type CmpmR = crate::BitReader;
#[doc = "Field `ARRM` reader - Arrm"]
pub type ArrmR = crate::BitReader;
#[doc = "Field `EXTTRIG` reader - Exttrig"]
pub type ExttrigR = crate::BitReader;
#[doc = "Field `CMPOK` reader - Cmpok"]
pub type CmpokR = crate::BitReader;
#[doc = "Field `ARROK` reader - Arrok"]
pub type ArrokR = crate::BitReader;
#[doc = "Field `UP` reader - Up"]
pub type UpR = crate::BitReader;
#[doc = "Field `DOWN` reader - Down"]
pub type DownR = crate::BitReader;
#[doc = "Field `CFGROK` reader - Cfgrok"]
pub type CfgrokR = crate::BitReader;
#[doc = "Field `CROK` reader - Crok"]
pub type CrokR = crate::BitReader;
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
    #[doc = "Bit 3 - Cmpok"]
    #[inline(always)]
    pub fn cmpok(&self) -> CmpokR {
        CmpokR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arrok"]
    #[inline(always)]
    pub fn arrok(&self) -> ArrokR {
        ArrokR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Up"]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Down"]
    #[inline(always)]
    pub fn down(&self) -> DownR {
        DownR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Cfgrok"]
    #[inline(always)]
    pub fn cfgrok(&self) -> CfgrokR {
        CfgrokR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Crok"]
    #[inline(always)]
    pub fn crok(&self) -> CrokR {
        CrokR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "LPTIMER flag and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}

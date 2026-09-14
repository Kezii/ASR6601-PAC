#[doc = "Register `RSC_ECR` reader"]
pub type R = crate::R<RscEcrSpec>;
#[doc = "Field `FE` reader - framing error flag"]
pub type FeR = crate::BitReader;
#[doc = "Field `PE` reader - parity error flag"]
pub type PeR = crate::BitReader;
#[doc = "Field `BE` reader - break error flag"]
pub type BeR = crate::BitReader;
#[doc = "Field `OE` reader - overrun error flag"]
pub type OeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - framing error flag"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - parity error flag"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - break error flag"]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - overrun error flag"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "receive status register / error clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsc_ecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RscEcrSpec;
impl crate::RegisterSpec for RscEcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsc_ecr::R`](R) reader structure"]
impl crate::Readable for RscEcrSpec {}

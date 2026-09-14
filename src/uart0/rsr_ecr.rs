#[doc = "Register `RSR_ECR` reader"]
pub type R = crate::R<RsrEcrSpec>;
#[doc = "Register `RSR_ECR` writer"]
pub type W = crate::W<RsrEcrSpec>;
#[doc = "Field `FE` reader - framing error flag"]
pub type FeR = crate::BitReader;
#[doc = "Field `FE` writer - framing error flag"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` reader - parity error flag"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - parity error flag"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` reader - break error flag"]
pub type BeR = crate::BitReader;
#[doc = "Field `BE` writer - break error flag"]
pub type BeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE` reader - overrun error flag"]
pub type OeR = crate::BitReader;
#[doc = "Field `OE` writer - overrun error flag"]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 0 - framing error flag"]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<'_, RsrEcrSpec> {
        FeW::new(self, 0)
    }
    #[doc = "Bit 1 - parity error flag"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, RsrEcrSpec> {
        PeW::new(self, 1)
    }
    #[doc = "Bit 2 - break error flag"]
    #[inline(always)]
    pub fn be(&mut self) -> BeW<'_, RsrEcrSpec> {
        BeW::new(self, 2)
    }
    #[doc = "Bit 3 - overrun error flag"]
    #[inline(always)]
    pub fn oe(&mut self) -> OeW<'_, RsrEcrSpec> {
        OeW::new(self, 3)
    }
}
#[doc = "receive status register / error clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsr_ecr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr_ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsrEcrSpec;
impl crate::RegisterSpec for RsrEcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsr_ecr::R`](R) reader structure"]
impl crate::Readable for RsrEcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rsr_ecr::W`](W) writer structure"]
impl crate::Writable for RsrEcrSpec {
    type Safety = crate::Unsafe;
}

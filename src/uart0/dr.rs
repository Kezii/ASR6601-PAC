#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DrSpec>;
#[doc = "Field `DATA` reader - transmit data character / receive data character"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - transmit data character / receive data character"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    #[doc = "Bits 0:7 - transmit data character / receive data character"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - framing error flag"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - parity error flag"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - break error flag"]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - overrun error flag"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - transmit data character / receive data character"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, DrSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bit 8 - framing error flag"]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<'_, DrSpec> {
        FeW::new(self, 8)
    }
    #[doc = "Bit 9 - parity error flag"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, DrSpec> {
        PeW::new(self, 9)
    }
    #[doc = "Bit 10 - break error flag"]
    #[inline(always)]
    pub fn be(&mut self) -> BeW<'_, DrSpec> {
        BeW::new(self, 10)
    }
    #[doc = "Bit 11 - overrun error flag"]
    #[inline(always)]
    pub fn oe(&mut self) -> OeW<'_, DrSpec> {
        OeW::new(self, 11)
    }
}
#[doc = "data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DrSpec {
    type Safety = crate::Unsafe;
}

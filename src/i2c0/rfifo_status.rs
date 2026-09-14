#[doc = "Register `RFIFO_STATUS` reader"]
pub type R = crate::R<RfifoStatusSpec>;
#[doc = "Field `OVERUN` reader - Overun"]
pub type OverunR = crate::BitReader;
#[doc = "Field `HFULL` reader - Hfull"]
pub type HfullR = crate::BitReader;
#[doc = "Field `FULL` reader - Full"]
pub type FullR = crate::BitReader;
#[doc = "Field `EMPTY` reader - Empty"]
pub type EmptyR = crate::BitReader;
#[doc = "Field `SIZE` reader - Size"]
pub type SizeR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Overun"]
    #[inline(always)]
    pub fn overun(&self) -> OverunR {
        OverunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hfull"]
    #[inline(always)]
    pub fn hfull(&self) -> HfullR {
        HfullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Full"]
    #[inline(always)]
    pub fn full(&self) -> FullR {
        FullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Empty"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "read fifo status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfifoStatusSpec;
impl crate::RegisterSpec for RfifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifo_status::R`](R) reader structure"]
impl crate::Readable for RfifoStatusSpec {}

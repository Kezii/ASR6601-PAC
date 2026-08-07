#[doc = "Register `WFIFO_STATUS` reader"]
pub type R = crate::R<WfifoStatusSpec>;
#[doc = "Field `FULL` reader - Full"]
pub type FullR = crate::BitReader;
#[doc = "Field `EMPTY` reader - Empty"]
pub type EmptyR = crate::BitReader;
#[doc = "Field `SIZE` reader - Size"]
pub type SizeR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Full"]
    #[inline(always)]
    pub fn full(&self) -> FullR {
        FullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Empty"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
#[doc = "write fifo status register\n\nYou can [`read`](crate::Reg::read) this register and get [`wfifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WfifoStatusSpec;
impl crate::RegisterSpec for WfifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wfifo_status::R`](R) reader structure"]
impl crate::Readable for WfifoStatusSpec {}

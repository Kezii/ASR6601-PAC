#[doc = "Register `CHIP_PATTERN` reader"]
pub type R = crate::R<ChipPatternSpec>;
#[doc = "Field `PATTERN` reader - Chip pattern"]
pub type PatternR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Chip pattern"]
    #[inline(always)]
    pub fn pattern(&self) -> PatternR {
        PatternR::new(self.bits)
    }
}
#[doc = "chip pattern register\n\nYou can [`read`](crate::Reg::read) this register and get [`chip_pattern::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChipPatternSpec;
impl crate::RegisterSpec for ChipPatternSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chip_pattern::R`](R) reader structure"]
impl crate::Readable for ChipPatternSpec {}

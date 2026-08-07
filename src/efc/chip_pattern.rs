#[doc = "Register `CHIP_PATTERN` reader"]
pub type R = crate::R<ChipPatternSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "chip pattern register\n\nYou can [`read`](crate::Reg::read) this register and get [`chip_pattern::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChipPatternSpec;
impl crate::RegisterSpec for ChipPatternSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chip_pattern::R`](R) reader structure"]
impl crate::Readable for ChipPatternSpec {}

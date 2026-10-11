#[doc = "Register `CNT` reader"]
pub type R = crate::R<CntSpec>;
#[doc = "Field `CNT` reader - Counting result, 16-bit"]
pub type CntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Counting result, 16-bit"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "LPTIM counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntSpec;
impl crate::RegisterSpec for CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CntSpec {}

#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Field `VALUE` reader - Current down-counter value, 32-bit"]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current down-counter value, 32-bit"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
#[doc = "value register\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}

#[doc = "Register `IDR` reader"]
pub type R = crate::R<IdrSpec>;
#[doc = "Field `ID` reader - pin\\[15:0\\] input"]
pub type IdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - pin\\[15:0\\] input"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "input data register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IdrSpec {}

#[doc = "Register `TEST_INFO_H` reader"]
pub type R = crate::R<TestInfoHSpec>;
#[doc = "Field `INFO` reader - Test info high value"]
pub type InfoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Test info high value"]
    #[inline(always)]
    pub fn info(&self) -> InfoR {
        InfoR::new(self.bits)
    }
}
#[doc = "test info high register\n\nYou can [`read`](crate::Reg::read) this register and get [`test_info_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestInfoHSpec;
impl crate::RegisterSpec for TestInfoHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test_info_h::R`](R) reader structure"]
impl crate::Readable for TestInfoHSpec {}

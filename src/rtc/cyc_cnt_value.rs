#[doc = "Register `CYC_CNT_VALUE` reader"]
pub type R = crate::R<CycCntValueSpec>;
#[doc = "Field `CYC_CNT_VALUE` reader - Cyc count value"]
pub type CycCntValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Cyc count value"]
    #[inline(always)]
    pub fn cyc_cnt_value(&self) -> CycCntValueR {
        CycCntValueR::new(self.bits)
    }
}
#[doc = "cyc counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cyc_cnt_value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CycCntValueSpec;
impl crate::RegisterSpec for CycCntValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cyc_cnt_value::R`](R) reader structure"]
impl crate::Readable for CycCntValueSpec {}

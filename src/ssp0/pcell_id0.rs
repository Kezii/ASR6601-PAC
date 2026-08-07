#[doc = "Register `PCELL_ID0` reader"]
pub type R = crate::R<PcellId0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "prime cell identification register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcell_id0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcellId0Spec;
impl crate::RegisterSpec for PcellId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcell_id0::R`](R) reader structure"]
impl crate::Readable for PcellId0Spec {}

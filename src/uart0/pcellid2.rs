#[doc = "Register `PCELLID2` reader"]
pub type R = crate::R<Pcellid2Spec>;
#[doc = "Field `CellID2` reader - primecell ID 2, fixed 0x05"]
pub type CellId2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - primecell ID 2, fixed 0x05"]
    #[inline(always)]
    pub fn cell_id2(&self) -> CellId2R {
        CellId2R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "primecell ID register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pcellid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcellid2Spec;
impl crate::RegisterSpec for Pcellid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcellid2::R`](R) reader structure"]
impl crate::Readable for Pcellid2Spec {}
